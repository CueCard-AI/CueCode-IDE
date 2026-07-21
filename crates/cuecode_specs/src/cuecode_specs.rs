mod document;
mod index;
mod parse;
mod telemetry;
mod watch;

pub use document::{
    format_spec_index_markdown, read_spec_document, render_spec_envelope, specs_dir_exists,
    status_label, SpecDocument, SPEC_INDEX_PROMPT_BUDGET,
};
pub use index::{resolve_spec_query, scan_spec_index, scan_spec_index_from_disk, SpecEntry, SpecIndex, SpecStatus};
pub use parse::{extract_anchor_ids, parse_spec_file, specs_dir_relative, specs_root, split_frontmatter};
pub use telemetry::{
    event_index_loaded, event_linked, event_mention_attached, event_watch_refresh,
};
pub use watch::{register_spec_index, SpecIndexStore, SPEC_WATCH_DEBOUNCE};

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use fs::Fs;
use gpui::App;
use settings::WorktreeId;

/// Register the global [`SpecIndexStore`]. Idempotent.
pub fn init(cx: &mut App) {
    if cx.try_global::<SpecIndexStore>().is_none() {
        cx.set_global(SpecIndexStore::new());
    }
}

/// Build or refresh the spec index for a worktree and start watching `.cursor/specs/`.
pub fn load_spec_index(worktree_root: &Path, cx: &mut App) -> SpecIndex {
    load_spec_index_for_worktree(worktree_root, WorktreeId::from_usize(0), cx)
}

/// Same as [`load_spec_index`] with an explicit [`WorktreeId`] (preferred for multi-root).
pub fn load_spec_index_for_worktree(
    worktree_root: &Path,
    worktree_id: WorktreeId,
    cx: &mut App,
) -> SpecIndex {
    init(cx);
    let fs = <dyn Fs>::global(cx);
    let started = Instant::now();
    let index = match scan_spec_index_from_disk(worktree_root, worktree_id) {
        Ok(index) => index,
        Err(error) => {
            log::warn!(
                "failed to scan spec index for {}: {error:#}",
                worktree_root.display()
            );
            SpecIndex {
                worktree_id,
                worktree_root: worktree_root.to_path_buf(),
                entries: Vec::new(),
                updated_at: chrono::Utc::now(),
            }
        }
    };
    crate::telemetry::event_index_loaded(
        index.entry_count(),
        started.elapsed().as_millis() as u64,
    );
    register_spec_index(
        fs,
        worktree_root.to_path_buf(),
        worktree_id,
        Arc::new(index.clone()),
        cx,
    );
    index
}

/// Latest cached index for a worktree, if [`load_spec_index`] has been called.
pub fn spec_index(worktree_id: WorktreeId, cx: &App) -> Option<Arc<SpecIndex>> {
    SpecIndexStore::try_global(cx).and_then(|store| store.index(worktree_id))
}

/// Merge cached entries from multiple worktrees (sorted by path).
pub fn merged_spec_entries(
    cx: &App,
    worktree_ids: impl IntoIterator<Item = WorktreeId>,
) -> Vec<SpecEntry> {
    let Some(store) = SpecIndexStore::try_global(cx) else {
        return Vec::new();
    };
    let mut entries = Vec::new();
    for worktree_id in worktree_ids {
        if let Some(index) = store.index(worktree_id) {
            entries.extend(index.entries.iter().cloned());
        }
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    entries
}
