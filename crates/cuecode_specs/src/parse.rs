use std::path::Path;

use anyhow::{Context as _, Result};
use regex::Regex;
use serde::Deserialize;
use settings::WorktreeId;

use crate::index::{SpecEntry, SpecStatus};

const SPECS_REL_DIR: &str = ".cursor/specs";

/// Relative path from worktree root to the specs directory.
pub fn specs_dir_relative() -> &'static str {
    SPECS_REL_DIR
}

pub fn specs_root(worktree_root: &Path) -> std::path::PathBuf {
    worktree_root.join(SPECS_REL_DIR)
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct SpecFrontmatter {
    title: Option<String>,
    status: Option<String>,
    tags: Option<Vec<String>>,
    summary: Option<String>,
}

/// Split optional YAML frontmatter from markdown body.
pub fn split_frontmatter(source: &str) -> (Option<&str>, &str) {
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    if !source.starts_with("---") {
        return (None, source);
    }
    let rest = source.get(3..).unwrap_or("");
    let rest = rest.strip_prefix('\n').unwrap_or(rest);
    if let Some(end) = rest.find("\n---") {
        let yaml = &rest[..end];
        let body = rest[end + 4..].strip_prefix('\n').unwrap_or(&rest[end + 4..]);
        (Some(yaml), body)
    } else {
        (None, source)
    }
}

pub(crate) fn parse_frontmatter(yaml: &str) -> SpecFrontmatter {
    serde_yaml::from_str(yaml).unwrap_or_default()
}

pub fn parse_status(raw: &str) -> Option<SpecStatus> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "draft" => Some(SpecStatus::Draft),
        "active" => Some(SpecStatus::Active),
        "done" | "complete" | "completed" => Some(SpecStatus::Done),
        "deprecated" => Some(SpecStatus::Deprecated),
        _ => None,
    }
}

pub fn extract_markdown_title(body: &str) -> Option<String> {
    for line in body.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("# ") {
            return Some(strip_heading_suffix(rest));
        }
        if let Some(rest) = line.strip_prefix("## ") {
            return Some(strip_heading_suffix(rest));
        }
    }
    None
}

fn strip_heading_suffix(heading: &str) -> String {
    heading
        .split('{')
        .next()
        .unwrap_or(heading)
        .trim()
        .to_string()
}

pub fn extract_anchor_ids(body: &str) -> Vec<String> {
    static ANCHOR: std::sync::LazyLock<Regex> =
        std::sync::LazyLock::new(|| Regex::new(r"\{#([a-zA-Z0-9_-]+)\}").expect("valid regex"));
    ANCHOR
        .captures_iter(body)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .collect()
}

pub fn title_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("spec")
        .replace('-', " ")
}

pub fn parse_spec_file(
    worktree_root: &Path,
    worktree_id: WorktreeId,
    absolute_path: &Path,
    source: &str,
) -> Result<SpecEntry> {
    let relative_path = absolute_path
        .strip_prefix(worktree_root)
        .with_context(|| {
            format!(
                "spec path {} is outside worktree {}",
                absolute_path.display(),
                worktree_root.display()
            )
        })?
        .to_path_buf();

    let (frontmatter_src, body) = split_frontmatter(source);
    let frontmatter = frontmatter_src
        .map(parse_frontmatter)
        .unwrap_or_default();

    let title = frontmatter
        .title
        .filter(|t| !t.trim().is_empty())
        .or_else(|| extract_markdown_title(body))
        .unwrap_or_else(|| title_from_path(absolute_path));

    let status = frontmatter
        .status
        .as_deref()
        .and_then(parse_status);

    let tags = frontmatter.tags.unwrap_or_default();
    let summary = frontmatter.summary.filter(|s| !s.trim().is_empty());
    let anchor_ids = extract_anchor_ids(body);

    Ok(SpecEntry {
        worktree_id,
        path: relative_path,
        title,
        status,
        tags,
        summary,
        anchor_ids,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings::WorktreeId;

    #[test]
    fn parses_lenient_frontmatter() {
        let src = "---\ntitle: My Spec\nstatus: active\ntags: [a, b]\nunknown: x\n---\n# Body {#body}\n";
        let (yaml, body) = split_frontmatter(src);
        let fm = parse_frontmatter(yaml.unwrap());
        assert_eq!(fm.title.as_deref(), Some("My Spec"));
        assert_eq!(parse_status(fm.status.as_deref().unwrap()), Some(SpecStatus::Active));
        assert_eq!(extract_anchor_ids(body), vec!["body"]);
    }

    #[test]
    fn title_from_heading_when_no_frontmatter() {
        let src = "# Vision {#vision}\n\nText";
        let entry = parse_spec_file(
            Path::new("/repo"),
            WorktreeId::from_usize(1),
            Path::new("/repo/.cursor/specs/01-vision.md"),
            src,
        )
        .unwrap();
        assert_eq!(entry.title, "Vision");
        assert_eq!(entry.anchor_ids, vec!["vision"]);
    }
}
