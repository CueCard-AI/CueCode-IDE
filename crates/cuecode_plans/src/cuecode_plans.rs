mod manifest;
mod progress;
mod resolve;
mod summary;
mod validate;
mod watch;

pub use manifest::{
    Artifact, ArtifactKind, ArtifactRef, ArtifactRoot, ArtifactStatus, BuildTrack, LoadedManifest,
    ManifestDocs, ManifestRoots, PinPolicy, ProjectInfo, ProjectManifestFile, RefRole,
    MANIFEST_ALIAS_REL, MANIFEST_CANONICAL_REL, find_manifest_path, load_manifest_from_disk,
    manifest_paths, write_manifest,
};
pub use progress::{CheckboxProgress, count_checkboxes, count_checkboxes_in_file};
pub use resolve::{
    ImplementBundle, ResolvedRef, build_implement_bundle, project_relative_path,
    resolve_artifact_path, resolve_project_relative_path, resolve_ref_path,
};
pub use summary::{
    PLAN_DIGEST_PROMPT_BUDGET, build_track_status_label, format_plan_digest_markdown, status_label,
};
pub use validate::{
    ValidationReport, ensure_manifest_exists, manifest_path_for_worktree, set_artifact_status,
    validate_manifest, validate_manifest_on_disk, validate_worktree, validate_worktree_or_error,
};
pub use watch::{
    PlanManifestStore, PLAN_WATCH_DEBOUNCE, init, load_plan_manifest_for_worktree, plan_manifest,
    register_plan_manifest,
};

use std::path::Path;
use std::sync::Arc;

use gpui::App;
use settings::WorktreeId;

/// Register the global [`PlanManifestStore`]. Idempotent.
pub fn init_store(cx: &mut App) {
    init(cx);
}

/// Load manifests for all provided worktrees when missing from the store.
pub fn ensure_plan_manifests_loaded(
    worktrees: impl IntoIterator<Item = (WorktreeId, std::path::PathBuf)>,
    cx: &mut App,
) {
    init(cx);
    for (worktree_id, root) in worktrees {
        if plan_manifest(worktree_id, cx).is_none() {
            load_plan_manifest_for_worktree(root.as_path(), worktree_id, cx);
        }
    }
}

/// First loaded manifest among the given worktrees (single-root dogfood helper).
pub fn first_plan_manifest(
    cx: &App,
    worktree_ids: impl IntoIterator<Item = WorktreeId>,
) -> Option<Arc<LoadedManifest>> {
    for worktree_id in worktree_ids {
        if let Some(manifest) = plan_manifest(worktree_id, cx) {
            return Some(manifest);
        }
    }
    None
}

pub fn has_plan_manifest(worktree_root: &Path) -> bool {
    find_manifest_path(worktree_root).is_some()
}
