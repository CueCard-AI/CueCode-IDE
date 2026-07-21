use std::path::{Path, PathBuf};
use std::sync::Arc;

use cuecode_plans::{LoadedManifest, ensure_plan_manifests_loaded, has_plan_manifest, plan_manifest};
use cuecode_specs::SpecEntry;
use gpui::App;
use project::ProjectPath;
use settings::WorktreeId;
use util::rel_path::RelPath;
use workspace::Workspace;

pub fn ensure_spec_indices_loaded(workspace: &Workspace, cx: &mut App) {
    let worktrees = workspace
        .visible_worktrees(cx)
        .map(|worktree| {
            let worktree = worktree.read(cx);
            (worktree.id(), worktree.abs_path().to_path_buf())
        })
        .collect::<Vec<_>>();

    for (worktree_id, root) in &worktrees {
        if cuecode_specs::spec_index(*worktree_id, cx).is_none() {
            cuecode_specs::load_spec_index_for_worktree(root.as_ref(), *worktree_id, cx);
        }
    }

    ensure_plan_manifests_loaded(worktrees, cx);
}

pub fn spec_entries_for_workspace(workspace: &Workspace, cx: &App) -> Vec<SpecEntry> {
    let worktree_ids = workspace
        .visible_worktrees(cx)
        .map(|worktree| worktree.read(cx).id())
        .collect::<Vec<_>>();
    cuecode_specs::merged_spec_entries(cx, worktree_ids)
}

pub fn worktree_roots(workspace: &Workspace, cx: &App) -> Vec<(WorktreeId, PathBuf)> {
    workspace
        .visible_worktrees(cx)
        .map(|worktree| {
            let worktree = worktree.read(cx);
            (worktree.id(), worktree.abs_path().to_path_buf())
        })
        .collect()
}

pub fn has_project_manifest(workspace: &Workspace, cx: &App) -> bool {
    worktree_roots(workspace, cx)
        .into_iter()
        .any(|(_, root)| has_plan_manifest(&root))
}

pub fn primary_plan_manifest(
    workspace: &Workspace,
    cx: &App,
) -> Option<(WorktreeId, PathBuf, Arc<LoadedManifest>)> {
    for (worktree_id, root) in worktree_roots(workspace, cx) {
        if let Some(manifest) = plan_manifest(worktree_id, cx) {
            return Some((worktree_id, root, manifest));
        }
    }
    None
}

pub fn open_spec_entry(
    workspace: &mut Workspace,
    entry: &SpecEntry,
    window: &mut gpui::Window,
    cx: &mut gpui::Context<Workspace>,
) {
    let Ok(path) = RelPath::unix(entry.path.as_path()) else {
        return;
    };
    let project_path = ProjectPath {
        worktree_id: entry.worktree_id,
        path: path.into(),
    };
    workspace
        .open_path(project_path, None, true, window, cx)
        .detach();
}

pub fn worktree_root_for_entry<'a>(
    roots: &'a [(WorktreeId, PathBuf)],
    entry: &SpecEntry,
) -> Option<&'a Path> {
    roots
        .iter()
        .find(|(id, _)| *id == entry.worktree_id)
        .map(|(_, root)| root.as_path())
}
