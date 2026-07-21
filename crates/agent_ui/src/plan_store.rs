use collections::HashMap;
use gpui::{AnyWindowHandle, App, AppContext as _, Entity, Global};
use parking_lot::RwLock;
use workspace::WorkspaceId;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlanHubTab {
    #[default]
    BuildTrack,
    AllArtifacts,
}

pub struct PlanStore {
    pub active_tab: PlanHubTab,
    pub build_track_selection: usize,
    pub selected_artifact_index: usize,
    pub focus_spec_path: Option<String>,
    pub detached_window: Option<AnyWindowHandle>,
    pub detached_threads_window: Option<AnyWindowHandle>,
    /// After Implement, Plan tab collapses to a strip above the thread composer.
    pub plan_collapsed: bool,
    pub collapsed_ticket_id: Option<String>,
    pub collapsed_task_progress: Option<String>,
    pub collapsed_ref_count: usize,
}

impl PlanStore {
    pub fn new() -> Self {
        Self {
            active_tab: PlanHubTab::BuildTrack,
            build_track_selection: 0,
            selected_artifact_index: 0,
            focus_spec_path: None,
            detached_window: None,
            detached_threads_window: None,
            plan_collapsed: false,
            collapsed_ticket_id: None,
            collapsed_task_progress: None,
            collapsed_ref_count: 0,
        }
    }

    pub fn collapse_for_ticket(
        &mut self,
        ticket_id: &str,
        task_progress: String,
        ref_count: usize,
    ) {
        self.plan_collapsed = true;
        self.collapsed_ticket_id = Some(ticket_id.to_string());
        self.collapsed_task_progress = Some(task_progress);
        self.collapsed_ref_count = ref_count;
        self.focus_spec_path = Some(ticket_id.to_string());
    }

    pub fn clear_collapsed(&mut self) {
        self.plan_collapsed = false;
        self.collapsed_ticket_id = None;
        self.collapsed_task_progress = None;
        self.collapsed_ref_count = 0;
    }

    pub fn is_threads_detached(&self) -> bool {
        self.detached_threads_window.is_some()
    }

    pub fn apply_focus_spec_path(&mut self, focus_spec_path: Option<String>) {
        self.focus_spec_path = focus_spec_path;
    }
}

struct PlanStoreRegistry {
    stores: RwLock<HashMap<WorkspaceId, Entity<PlanStore>>>,
}

impl Global for PlanStoreRegistry {}

pub fn plan_store_for(workspace_id: WorkspaceId, cx: &mut App) -> Entity<PlanStore> {
    if cx.try_global::<PlanStoreRegistry>().is_none() {
        cx.set_global(PlanStoreRegistry {
            stores: RwLock::new(HashMap::default()),
        });
    }

    if let Some(store) = cx
        .global::<PlanStoreRegistry>()
        .stores
        .read()
        .get(&workspace_id)
        .cloned()
    {
        return store;
    }

    let store = cx.new(|_| PlanStore::new());
    cx.global::<PlanStoreRegistry>()
        .stores
        .write()
        .insert(workspace_id, store.clone());
    store
}
