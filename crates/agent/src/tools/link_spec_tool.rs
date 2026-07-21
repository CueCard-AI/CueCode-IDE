use std::path::PathBuf;
use std::sync::Arc;

use crate::{AgentTool, Thread, ToolCallEventStream, ToolInput};
use agent_client_protocol::schema as acp;
use cuecode_specs::{event_linked, read_spec_document, render_spec_envelope};
use gpui::{App, Entity, SharedString, Task, WeakEntity};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use util::markdown::MarkdownInlineCode;

/// Link a `.cursor/specs/` document to this session. Sets the active spec and
/// injects the full body into context on subsequent turns.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinkSpecToolInput {
    /// Project-relative path under `.cursor/specs/` (e.g. `core/05-innovations.md`).
    pub path: String,
}

pub struct LinkSpecTool {
    project: Entity<Project>,
    thread: WeakEntity<Thread>,
}

impl LinkSpecTool {
    pub fn new(project: Entity<Project>, thread: WeakEntity<Thread>) -> Self {
        Self { project, thread }
    }
}

impl AgentTool for LinkSpecTool {
    type Input = LinkSpecToolInput;
    type Output = String;

    const NAME: &'static str = "link_spec";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Read
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Some(path) = input.ok().map(|input| input.path) {
            format!("Link spec {}", MarkdownInlineCode(&path)).into()
        } else {
            "Link spec".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        let thread = self.thread.clone();
        cx.spawn(async move |cx| {
            let input = input.recv().await.map_err(|error| error.to_string())?;
            let relative_path = PathBuf::from(input.path.trim());
            if relative_path.as_os_str().is_empty() {
                return Err("path must not be empty".into());
            }

            let worktrees = project.read_with(cx, |project, cx| {
                project
                    .visible_worktrees(cx)
                    .map(|worktree| worktree.read(cx).abs_path())
                    .collect::<Vec<_>>()
            });

            for root in worktrees {
                let candidate = root.join(&relative_path);
                if !candidate.is_file() {
                    continue;
                }
                let doc = read_spec_document(root.as_ref(), &relative_path)
                    .map_err(|error| error.to_string())?;
                let rendered = render_spec_envelope(&doc.path, &doc.title, &doc.body);
                if let Some(thread) = thread.upgrade() {
                    thread.update(cx, |thread, _cx| {
                        thread.set_active_spec_path(Some(relative_path.clone()));
                    });
                }
                event_linked(&relative_path);
                return Ok(rendered);
            }

            Err(format!(
                "Spec not found: {} (expected under `.cursor/specs/` in a worktree root)",
                relative_path.display()
            ))
        })
    }
}
