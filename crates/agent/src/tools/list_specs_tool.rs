use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use cuecode_specs::{merged_spec_entries, resolve_spec_query, status_label};
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// List indexed product specs from `.cursor/specs/` (titles, paths, status).
/// Use before `read_file` when you need to discover which spec to load.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSpecsToolInput {
    /// Optional fuzzy filter on title or path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListSpecsToolSpecEntry {
    path: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ListSpecsToolOutput {
    specs: Vec<ListSpecsToolSpecEntry>,
}

pub struct ListSpecsTool {
    project: Entity<Project>,
}

impl ListSpecsTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for ListSpecsTool {
    type Input = ListSpecsToolInput;
    type Output = String;

    const NAME: &'static str = "list_specs";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if input
            .ok()
            .and_then(|input| input.filter)
            .is_some_and(|filter| !filter.trim().is_empty())
        {
            "List matching specs".into()
        } else {
            "List specs".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|error| error.to_string())?;

            let entries = project.read_with(cx, |project, cx| {
                let worktree_ids = project
                    .visible_worktrees(cx)
                    .map(|worktree| worktree.read(cx).id())
                    .collect::<Vec<_>>();
                merged_spec_entries(cx, worktree_ids)
            });

            let filtered = if let Some(filter) = input.filter.filter(|filter| !filter.trim().is_empty())
            {
                let index = cuecode_specs::SpecIndex {
                    worktree_id: settings::WorktreeId::from_usize(0),
                    worktree_root: Default::default(),
                    entries,
                    updated_at: chrono::Utc::now(),
                };
                resolve_spec_query(&index, &filter)
            } else {
                entries
            };

            let specs = filtered
                .into_iter()
                .map(|entry| ListSpecsToolSpecEntry {
                    path: entry.path.to_string_lossy().into_owned(),
                    title: entry.title,
                    summary: entry.summary,
                    status: entry
                        .status
                        .as_ref()
                        .map(|status| status_label(Some(status)).to_string()),
                })
                .collect();

            serde_json::to_string_pretty(&ListSpecsToolOutput { specs })
                .map_err(|error| error.to_string())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_serializes_specs_array() {
        let output = ListSpecsToolOutput {
            specs: vec![ListSpecsToolSpecEntry {
                path: ".cursor/specs/core/01-vision.md".into(),
                title: "Vision".into(),
                summary: Some("Summary".into()),
                status: Some("active".into()),
            }],
        };
        let json = serde_json::to_value(&output).unwrap();
        assert!(json.get("specs").unwrap().is_array());
    }
}
