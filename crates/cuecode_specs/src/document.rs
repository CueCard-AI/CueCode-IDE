use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};

use crate::index::SpecStatus;
use crate::parse::{parse_spec_file, specs_root, split_frontmatter};

pub const SPEC_INDEX_PROMPT_BUDGET: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecDocument {
    pub path: PathBuf,
    pub title: String,
    pub status: Option<SpecStatus>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub body: String,
    pub anchor_ids: Vec<String>,
}

/// Compact markdown table for system prompt injection (~4KB budget).
pub fn format_spec_index_markdown(entries: &[crate::index::SpecEntry]) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut out = String::from(
        "The workspace maintains product specs under `.cursor/specs/`. \
         Use `list_specs` or `@spec` in the composer to load full bodies.\n\n\
         | Path | Title | Status |\n| --- | --- | --- |\n",
    );

    for entry in entries {
        let line = format!(
            "| {} | {} | {} |\n",
            entry.path.display(),
            entry.title.replace('|', "\\|"),
            status_label(entry.status.as_ref())
        );
        if out.len() + line.len() > SPEC_INDEX_PROMPT_BUDGET {
            out.push_str("\n*(index truncated)*\n");
            break;
        }
        out.push_str(&line);
    }

    out
}

pub fn status_label(status: Option<&SpecStatus>) -> &'static str {
    match status {
        Some(SpecStatus::Draft) => "draft",
        Some(SpecStatus::Active) => "active",
        Some(SpecStatus::Done) => "done",
        Some(SpecStatus::Deprecated) => "deprecated",
        None => "-",
    }
}

/// Wrap a spec body for model context (mirrors skill envelope shape).
pub fn render_spec_envelope(path: &Path, title: &str, body: &str) -> String {
    format!(
        "<spec_content path=\"{}\">\n# {}\n\n{}\n</spec_content>\n",
        path.display(),
        title.replace('"', "\\\""),
        body.trim()
    )
}

pub fn read_spec_document(worktree_root: &Path, relative_path: &Path) -> Result<SpecDocument> {
    let absolute_path = worktree_root.join(relative_path);
    let source = std::fs::read_to_string(&absolute_path)
        .with_context(|| format!("reading spec file {}", absolute_path.display()))?;
    let (_, body) = split_frontmatter(&source);
    let entry = parse_spec_file(
        worktree_root,
        settings::WorktreeId::from_usize(0),
        &absolute_path,
        &source,
    )?;
    Ok(SpecDocument {
        path: relative_path.to_path_buf(),
        title: entry.title,
        status: entry.status,
        tags: entry.tags,
        summary: entry.summary,
        body: body.to_string(),
        anchor_ids: entry.anchor_ids,
    })
}

pub fn specs_dir_exists(worktree_root: &Path) -> bool {
    specs_root(worktree_root).is_dir()
}
