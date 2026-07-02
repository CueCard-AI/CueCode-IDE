use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use fs::Fs;
use futures::StreamExt as _;
use gpui::{App, BackgroundExecutor, Global, Task};
use parking_lot::RwLock;
use settings::WorktreeId;

use crate::index::{scan_spec_index, SpecIndex};
use crate::parse::specs_root;

pub const SPEC_WATCH_DEBOUNCE: Duration = Duration::from_secs(2);

struct WorktreeSpecState {
    index: Arc<SpecIndex>,
    _watcher: Task<()>,
}

pub(crate) struct SpecIndexStoreInner {
    worktrees: std::collections::HashMap<WorktreeId, WorktreeSpecState>,
}

/// Per-worktree spec indices refreshed by filesystem watchers.
pub struct SpecIndexStore {
    inner: Arc<RwLock<SpecIndexStoreInner>>,
}

impl SpecIndexStore {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(SpecIndexStoreInner {
                worktrees: std::collections::HashMap::new(),
            })),
        }
    }

    pub fn global(cx: &App) -> &Self {
        cx.global::<Self>()
    }

    pub fn index(&self, worktree_id: WorktreeId) -> Option<Arc<SpecIndex>> {
        self.inner
            .read()
            .worktrees
            .get(&worktree_id)
            .map(|state| state.index.clone())
    }

    fn set_worktree(&self, worktree_id: WorktreeId, index: Arc<SpecIndex>, watcher: Task<()>) {
        self.inner.write().worktrees.insert(
            worktree_id,
            WorktreeSpecState {
                index,
                _watcher: watcher,
            },
        );
    }

    pub fn try_global(cx: &App) -> Option<&Self> {
        cx.try_global::<Self>()
    }

    fn inner(&self) -> Arc<RwLock<SpecIndexStoreInner>> {
        self.inner.clone()
    }
}

impl Global for SpecIndexStore {}

pub(crate) fn watch_spec_index(
    executor: &BackgroundExecutor,
    fs: Arc<dyn Fs>,
    worktree_root: PathBuf,
    worktree_id: WorktreeId,
    debounce: Duration,
    store: Arc<RwLock<SpecIndexStoreInner>>,
) -> Task<()> {
    executor.spawn(async move {
        let specs_dir = specs_root(&worktree_root);
        if fs.metadata(&specs_dir).await.ok().flatten().is_none() {
            return;
        }

        let (mut events, _watcher) = fs.watch(&specs_dir, debounce).await;
        while let Some(batch) = events.next().await {
            if !batch
                .iter()
                .any(|event| is_spec_markdown_event(&specs_dir, &event.path))
            {
                continue;
            }
            crate::telemetry::event_watch_refresh(batch.len());
            match scan_spec_index(&worktree_root, worktree_id, |path| {
                Ok(std::fs::read_to_string(path)?)
            }) {
                Ok(index) => {
                    if let Some(state) = store.write().worktrees.get_mut(&worktree_id) {
                        state.index = Arc::new(index);
                    }
                }
                Err(error) => {
                    log::warn!(
                        "failed to rebuild spec index for {}: {error:#}",
                        worktree_root.display()
                    );
                }
            }
        }
    })
}

fn is_spec_markdown_event(specs_dir: &Path, path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("md") && path.starts_with(specs_dir)
}

pub fn register_spec_index(
    fs: Arc<dyn Fs>,
    worktree_root: PathBuf,
    worktree_id: WorktreeId,
    index: Arc<SpecIndex>,
    cx: &mut App,
) {
    let store = SpecIndexStore::global(cx);
    let inner = store.inner();
    let watcher = watch_spec_index(
        cx.background_executor(),
        fs,
        worktree_root,
        worktree_id,
        SPEC_WATCH_DEBOUNCE,
        inner,
    );
    store.set_worktree(worktree_id, index, watcher);
}
