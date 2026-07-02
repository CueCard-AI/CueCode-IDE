use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, BackgroundExecutor, Global, Task};
use parking_lot::RwLock;
use settings::WorktreeId;

use crate::manifest::{LoadedManifest, find_manifest_path, load_manifest_from_disk};

pub const PLAN_WATCH_DEBOUNCE: Duration = Duration::from_secs(2);

struct WorktreePlanState {
    loaded: Arc<LoadedManifest>,
    generation: u64,
    _watcher: Task<()>,
}

pub(crate) struct PlanManifestStoreInner {
    worktrees: std::collections::HashMap<WorktreeId, WorktreePlanState>,
}

/// Per-worktree plan manifests refreshed by filesystem watchers.
pub struct PlanManifestStore {
    inner: Arc<RwLock<PlanManifestStoreInner>>,
}

impl PlanManifestStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(PlanManifestStoreInner {
                worktrees: std::collections::HashMap::new(),
            })),
        }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn try_global(cx: &App) -> Option<&Self> {
        cx.try_global::<Self>()
    }

    pub fn manifest(&self, worktree_id: WorktreeId) -> Option<Arc<LoadedManifest>> {
        self.inner
            .read()
            .worktrees
            .get(&worktree_id)
            .map(|state| state.loaded.clone())
    }

    pub fn generation(&self, worktree_id: WorktreeId) -> Option<u64> {
        self.inner
            .read()
            .worktrees
            .get(&worktree_id)
            .map(|state| state.generation)
    }

    fn set_worktree(
        &self,
        worktree_id: WorktreeId,
        loaded: Arc<LoadedManifest>,
        generation: u64,
        watcher: Task<()>,
    ) {
        self.inner.write().worktrees.insert(
            worktree_id,
            WorktreePlanState {
                loaded,
                generation,
                _watcher: watcher,
            },
        );
    }

    fn inner(&self) -> Arc<RwLock<PlanManifestStoreInner>> {
        self.inner.clone()
    }
}

impl Global for PlanManifestStore {}

pub fn init(cx: &mut App) {
    if cx.try_global::<PlanManifestStore>().is_none() {
        cx.set_global(PlanManifestStore::new());
    }
}

pub fn load_plan_manifest_for_worktree(
    worktree_root: &Path,
    worktree_id: WorktreeId,
    cx: &mut App,
) -> Option<Arc<LoadedManifest>> {
    init(cx);
    let fs = <dyn Fs>::global(cx);
    let loaded = match load_manifest_from_disk(worktree_root) {
        Ok(Some(loaded)) => Arc::new(loaded),
        Ok(None) => return None,
        Err(error) => {
            log::warn!(
                "failed to load plan manifest for {}: {error:#}",
                worktree_root.display()
            );
            return None;
        }
    };

    register_plan_manifest(
        fs,
        worktree_root.to_path_buf(),
        worktree_id,
        loaded.clone(),
        cx,
    );
    Some(loaded)
}

pub fn plan_manifest(worktree_id: WorktreeId, cx: &App) -> Option<Arc<LoadedManifest>> {
    PlanManifestStore::try_global(cx).and_then(|store| store.manifest(worktree_id))
}

pub fn register_plan_manifest(
    fs: Arc<dyn Fs>,
    worktree_root: PathBuf,
    worktree_id: WorktreeId,
    loaded: Arc<LoadedManifest>,
    cx: &mut App,
) {
    init(cx);
    let store = PlanManifestStore::global(cx);
    let generation = store.generation(worktree_id).unwrap_or(0) + 1;
    let watcher = watch_plan_manifest(
        cx.background_executor(),
        fs,
        worktree_root,
        worktree_id,
        PLAN_WATCH_DEBOUNCE,
        store.inner(),
    );
    store.set_worktree(worktree_id, loaded, generation, watcher);
}

fn watch_plan_manifest(
    executor: &BackgroundExecutor,
    fs: Arc<dyn Fs>,
    worktree_root: PathBuf,
    worktree_id: WorktreeId,
    debounce: Duration,
    store: Arc<RwLock<PlanManifestStoreInner>>,
) -> Task<()> {
    executor.spawn(async move {
        let Some(manifest_path) = find_manifest_path(&worktree_root) else {
            return;
        };
        let watch_dir = manifest_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| worktree_root.clone());

        let (mut events, _watcher) = fs.watch(&watch_dir, debounce).await;
        while let Some(batch) = events.next().await {
            if !batch.iter().any(|event| event.path == manifest_path) {
                continue;
            }
            match load_manifest_from_disk(&worktree_root) {
                Ok(Some(loaded)) => {
                    if let Some(state) = store.write().worktrees.get_mut(&worktree_id) {
                        state.loaded = Arc::new(loaded);
                        state.generation = state.generation.saturating_add(1);
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    log::warn!(
                        "failed to reload plan manifest for {}: {error:#}",
                        worktree_root.display()
                    );
                }
            }
        }
    })
}
