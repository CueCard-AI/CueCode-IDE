use std::cmp::Ordering;
use std::path::{Path, PathBuf};

use anyhow::{Context as _, Result};
use chrono::{DateTime, Utc};
use settings::WorktreeId;
use walkdir::WalkDir;

use crate::parse::{parse_spec_file, specs_root};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SpecStatus {
    Draft,
    Active,
    Done,
    Deprecated,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct SpecEntry {
    pub worktree_id: WorktreeId,
    pub path: PathBuf,
    pub title: String,
    pub status: Option<SpecStatus>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub anchor_ids: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SpecIndex {
    pub worktree_id: WorktreeId,
    pub worktree_root: PathBuf,
    pub entries: Vec<SpecEntry>,
    pub updated_at: DateTime<Utc>,
}

impl SpecIndex {
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

/// Scan `.cursor/specs/**/*.md` under `worktree_root` and build an in-memory index.
pub fn scan_spec_index(
    worktree_root: &Path,
    worktree_id: WorktreeId,
    read_file: impl Fn(&Path) -> Result<String>,
) -> Result<SpecIndex> {
    let specs_root = specs_root(worktree_root);
    if !specs_root.is_dir() {
        return Ok(SpecIndex {
            worktree_id,
            worktree_root: worktree_root.to_path_buf(),
            entries: Vec::new(),
            updated_at: Utc::now(),
        });
    }

    let mut entries = Vec::new();
    for entry in WalkDir::new(&specs_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let source = read_file(path)
            .with_context(|| format!("reading spec file {}", path.display()))?;
        entries.push(parse_spec_file(worktree_root, worktree_id, path, &source)?);
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(SpecIndex {
        worktree_id,
        worktree_root: worktree_root.to_path_buf(),
        entries,
        updated_at: Utc::now(),
    })
}

pub fn scan_spec_index_from_disk(
    worktree_root: &Path,
    worktree_id: WorktreeId,
) -> Result<SpecIndex> {
    scan_spec_index(worktree_root, worktree_id, |path| {
        Ok(std::fs::read_to_string(path)?)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MatchScore(u8);

/// Fuzzy match for `@spec` composer completion (path slug, title, anchors).
pub fn resolve_spec_query(index: &SpecIndex, query: &str) -> Vec<SpecEntry> {
    let query = query.trim();
    if query.is_empty() {
        return index.entries.clone();
    }

    let query_lower = query.to_ascii_lowercase();
    let mut scored: Vec<(MatchScore, SpecEntry)> = index
        .entries
        .iter()
        .filter_map(|entry| {
            score_entry(entry, &query_lower).map(|score| (score, entry.clone()))
        })
        .collect();

    scored.sort_by(|(a, left), (b, right)| {
        b.cmp(a).then_with(|| left.path.cmp(&right.path))
    });

    scored.into_iter().map(|(_, entry)| entry).collect()
}

fn score_entry(entry: &SpecEntry, query_lower: &str) -> Option<MatchScore> {
    let path_str = entry.path.to_string_lossy().to_ascii_lowercase();
    let title_lower = entry.title.to_ascii_lowercase();

    if path_str == query_lower || title_lower == query_lower {
        return Some(MatchScore(100));
    }
    if path_str.ends_with(query_lower)
        || path_str.contains(&format!("/{query_lower}"))
        || path_str.contains(&format!("{query_lower}."))
    {
        return Some(MatchScore(80));
    }
    if title_lower.contains(query_lower) {
        return Some(MatchScore(60));
    }
    if entry
        .anchor_ids
        .iter()
        .any(|id| id.eq_ignore_ascii_case(query_lower))
    {
        return Some(MatchScore(50));
    }
    if entry
        .tags
        .iter()
        .any(|tag| tag.to_ascii_lowercase().contains(query_lower))
    {
        return Some(MatchScore(40));
    }
    if path_str.contains(query_lower) {
        return Some(MatchScore(30));
    }
    None
}

impl PartialOrd for MatchScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MatchScore {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}
