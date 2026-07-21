use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use cuecode_plans::{
    ArtifactKind, ArtifactStatus, CheckboxProgress, LoadedManifest, PlanManifestStore,
    build_track_status_label, count_checkboxes_in_file, resolve_artifact_path,
};
use cuecode_specs::{SpecEntry, status_label};
use db::kvp::Dismissable;
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    ListOffset, ListState, ScrollHandle, SharedString, Subscription, Window, WindowHandle,
    WindowOptions, div, list, prelude::*, px,
};
use language::LanguageRegistry;
use markdown::{Markdown, MarkdownElement, MarkdownFont, MarkdownStyle};
use ui::{
    Button, ButtonStyle, IconButton, IconName, IconSize, Label, LabelSize, ListItem, ListItemSpacing,
    Modal, ModalFooter, ModalHeader, Section, Tooltip, WithScrollbar, prelude::*,
};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::{ModalView, MultiWorkspace, Workspace, WorkspaceId};

use crate::agent_panel::AgentPanel;
use crate::plan_session::{implement_build_phase, mark_build_phase_done};
use crate::plan_store::{PlanHubTab, PlanStore, plan_store_for};
use crate::spec_planning::{
    ensure_spec_indices_loaded, has_project_manifest, open_spec_entry, primary_plan_manifest,
    spec_entries_for_workspace, worktree_root_for_entry, worktree_roots,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlanningHubHost {
    Panel,
    Detached,
}

pub struct PlanningHubOrganizeDismissed;

impl PlanningHubOrganizeDismissed {
    pub fn dismissed(cx: &App) -> bool {
        <Self as Dismissable>::dismissed(cx)
    }

    pub fn dismiss(cx: &mut App) {
        <Self as Dismissable>::set_dismissed(true, cx);
    }
}

impl Dismissable for PlanningHubOrganizeDismissed {
    const KEY: &'static str = "dismissed-planning-hub-organize";
}

pub struct PlanningHubView {
    host: PlanningHubHost,
    workspace: Entity<Workspace>,
    workspace_id: WorkspaceId,
    plan_store: Entity<PlanStore>,
    language_registry: Arc<LanguageRegistry>,
    entries: Vec<SpecEntry>,
    worktree_roots: Vec<(settings::WorktreeId, PathBuf)>,
    manifest_worktree_id: Option<settings::WorktreeId>,
    manifest_worktree_root: Option<PathBuf>,
    manifest: Option<Arc<LoadedManifest>>,
    manifest_generation: u64,
    manifest_reloaded: bool,
    build_track_artifact_indices: Vec<usize>,
    build_track_progress: HashMap<String, CheckboxProgress>,
    filtered_indices: Vec<usize>,
    search_editor: Entity<InputField>,
    preview_markdown: Option<Entity<Markdown>>,
    preview_title: SharedString,
    preview_error: Option<SharedString>,
    focus_handle: FocusHandle,
    list_state: ListState,
    preview_scroll_handle: ScrollHandle,
    _plan_store_subscription: Subscription,
}

impl PlanningHubView {
    pub fn new_for_panel(
        workspace: Entity<Workspace>,
        workspace_id: WorkspaceId,
        focus_spec_path: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(
            PlanningHubHost::Panel,
            workspace,
            workspace_id,
            focus_spec_path,
            window,
            cx,
        )
    }

    pub fn new_for_detached(
        workspace: Entity<Workspace>,
        workspace_id: WorkspaceId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        Self::new(
            PlanningHubHost::Detached,
            workspace,
            workspace_id,
            None,
            window,
            cx,
        )
    }

    fn new(
        host: PlanningHubHost,
        workspace: Entity<Workspace>,
        workspace_id: WorkspaceId,
        focus_spec_path: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let entries = spec_entries_for_workspace(workspace.read(cx), cx);
        let worktree_roots = worktree_roots(workspace.read(cx), cx);
        let has_manifest = has_project_manifest(workspace.read(cx), cx);
        let plan = primary_plan_manifest(workspace.read(cx), cx);
        let language_registry = workspace.read(cx).project().read(cx).languages().clone();
        let plan_store = plan_store_for(workspace_id, cx);

        if let Some(path) = focus_spec_path.clone() {
            plan_store.update(cx, |store, _| {
                store.apply_focus_spec_path(Some(path));
            });
        }

        let (manifest_worktree_id, manifest_worktree_root, manifest, manifest_generation) =
            if let Some((worktree_id, root, loaded)) = plan {
                let generation = PlanManifestStore::try_global(cx)
                    .and_then(|store| store.generation(worktree_id))
                    .unwrap_or(1);
                (Some(worktree_id), Some(root), Some(loaded), generation)
            } else {
                (None, None, None, 0)
            };

        let build_track_artifact_indices = manifest
            .as_ref()
            .map(|loaded| {
                loaded
                    .manifest
                    .artifacts
                    .iter()
                    .enumerate()
                    .filter_map(|(index, artifact)| {
                        (artifact.kind == ArtifactKind::BuildPhase).then_some(index)
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        plan_store.update(cx, |store, _| {
            if has_manifest {
                store.active_tab = PlanHubTab::BuildTrack;
            } else {
                store.active_tab = PlanHubTab::AllArtifacts;
            }
            Self::apply_focus_spec_path_to_store(store, &entries, &build_track_artifact_indices, manifest.as_ref());
        });

        let filtered_indices: Vec<usize> = (0..entries.len()).collect();
        let list_len = filtered_indices.len().max(1);
        let selected_index = plan_store.read(cx).selected_artifact_index;

        let search_editor = cx.new(|cx| InputField::new(window, cx, "Search artifacts…"));
        let focus_handle = cx.focus_handle();

        let list_state =
            ListState::new(list_len, gpui::ListAlignment::Top, px(56.0)).measure_all();
        if selected_index > 0 {
            list_state.scroll_to(ListOffset {
                item_ix: selected_index,
                offset_in_item: px(0.0).into(),
            });
        }

        let plan_store_subscription = cx.observe(&plan_store, |this, _, cx| {
            let selected_index = this.plan_store.read(cx).selected_artifact_index;
            this.list_state.scroll_to(ListOffset {
                item_ix: selected_index,
                offset_in_item: px(0.0).into(),
            });
            cx.notify();
        });

        let mut this = Self {
            host,
            workspace,
            workspace_id,
            plan_store,
            language_registry,
            entries,
            worktree_roots,
            manifest_worktree_id,
            manifest_worktree_root,
            manifest,
            manifest_generation,
            manifest_reloaded: false,
            build_track_artifact_indices,
            build_track_progress: HashMap::new(),
            filtered_indices,
            search_editor,
            preview_markdown: None,
            preview_title: SharedString::default(),
            preview_error: None,
            focus_handle,
            list_state,
            preview_scroll_handle: ScrollHandle::new(),
            _plan_store_subscription: plan_store_subscription,
        };

        this.refresh_manifest_watch_state(cx);
        this.reload_build_track_progress(window, cx);
        this.reload_preview(window, cx);
        this
    }

    pub fn apply_focus_spec_path(
        &mut self,
        focus_spec_path: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.plan_store.update(cx, |store, _| {
            store.apply_focus_spec_path(focus_spec_path);
            Self::apply_focus_spec_path_to_store(
                store,
                &self.entries,
                &self.build_track_artifact_indices,
                self.manifest.as_ref(),
            );
        });
        let selected_index = self.plan_store.read(cx).selected_artifact_index;
        if selected_index > 0 {
            self.list_state.scroll_to(ListOffset {
                item_ix: selected_index,
                offset_in_item: px(0.0).into(),
            });
        }
        self.reload_preview(window, cx);
        cx.notify();
    }

    fn apply_focus_spec_path_to_store(
        store: &mut PlanStore,
        entries: &[SpecEntry],
        build_track_artifact_indices: &[usize],
        manifest: Option<&Arc<LoadedManifest>>,
    ) {
        let Some(focus) = store.focus_spec_path.as_deref() else {
            return;
        };

        if let Some(manifest) = manifest {
            if let Some(row_ix) = build_track_artifact_indices.iter().position(|artifact_index| {
                manifest
                    .manifest
                    .artifacts
                    .get(*artifact_index)
                    .is_some_and(|artifact| artifact.id == focus || artifact.id.ends_with(focus))
            }) {
                store.active_tab = PlanHubTab::BuildTrack;
                store.build_track_selection = row_ix;
                store.focus_spec_path = None;
                return;
            }
        }

        if let Some(index) = entries
            .iter()
            .position(|entry| paths_match(&entry.path, focus))
        {
            store.active_tab = PlanHubTab::AllArtifacts;
            store.selected_artifact_index = index;
            store.focus_spec_path = None;
        }
    }

    fn active_tab(&self, cx: &App) -> PlanHubTab {
        self.plan_store.read(cx).active_tab
    }

    fn build_track_selection(&self, cx: &App) -> usize {
        self.plan_store.read(cx).build_track_selection
    }

    fn selected_index(&self, cx: &App) -> usize {
        self.plan_store.read(cx).selected_artifact_index
    }

    fn selected_entry(&self, cx: &App) -> Option<&SpecEntry> {
        self.filtered_indices
            .get(self.selected_index(cx))
            .and_then(|index| self.entries.get(*index))
    }

    fn set_tab(&mut self, tab: PlanHubTab, window: &mut Window, cx: &mut Context<Self>) {
        self.plan_store.update(cx, |store, _| {
            store.active_tab = tab;
        });
        if tab == PlanHubTab::BuildTrack {
            self.reload_build_track_progress(window, cx);
        } else if tab == PlanHubTab::AllArtifacts {
            self.reload_preview(window, cx);
        }
        cx.notify();
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.filtered_indices.is_empty() {
            return;
        }
        let index = index.min(self.filtered_indices.len() - 1);
        self.plan_store.update(cx, |store, _| {
            store.selected_artifact_index = index;
        });
        self.list_state.scroll_to(ListOffset {
            item_ix: index,
            offset_in_item: px(0.0).into(),
        });
        self.reload_preview(window, cx);
        cx.notify();
    }

    fn set_build_track_selection(&mut self, row_ix: usize, cx: &mut Context<Self>) {
        self.plan_store.update(cx, |store, _| {
            store.build_track_selection = row_ix;
        });
        cx.notify();
    }

    fn reload_preview(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.selected_entry(cx).cloned() else {
            self.preview_markdown = None;
            self.preview_title = "No artifact selected".into();
            self.preview_error = None;
            return;
        };

        self.preview_title = entry.title.clone().into();
        let Some(worktree_root) = worktree_root_for_entry(&self.worktree_roots, &entry)
            .map(|path| path.to_path_buf())
        else {
            self.preview_error = Some("Worktree not found".into());
            self.preview_markdown = None;
            return;
        };

        let relative_path = entry.path.clone();
        let language_registry = self.language_registry.clone();
        cx.spawn_in(window, async move |this, cx| {
            let preview = cuecode_specs::read_spec_document(
                worktree_root.as_path(),
                relative_path.as_path(),
            );

            this.update_in(cx, |this, _window, cx| {
                match preview {
                    Ok(doc) => {
                        this.preview_error = None;
                        this.preview_markdown = Some(cx.new(|cx| {
                            Markdown::new(doc.body.into(), Some(language_registry), None, cx)
                        }));
                    }
                    Err(error) => {
                        this.preview_markdown = None;
                        this.preview_error = Some(format!("{error:#}").into());
                    }
                }
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn open_selected(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(entry) = self.selected_entry(cx).cloned() else {
            return;
        };
        self.workspace.update(cx, |workspace, cx| {
            open_spec_entry(workspace, &entry, window, cx);
        });
    }

    fn open_detached_window(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(handle) = self.plan_store.read(cx).detached_window {
            if handle
                .update(cx, |_, window, _| window.activate_window())
                .is_ok()
            {
                return;
            }
            self.plan_store.update(cx, |store, _| {
                store.detached_window = None;
            });
        }

        let workspace = self.workspace.clone();
        let workspace_id = self.workspace_id;
        let plan_store = self.plan_store.clone();

        self.workspace.update(cx, |workspace, cx| {
            ensure_spec_indices_loaded(workspace, cx);
        });

        let detached = cx
            .open_window(WindowOptions::default(), move |window, cx| {
                cx.new(|cx| {
                    PlanningHubView::new_for_detached(
                        workspace.clone(),
                        workspace_id,
                        window,
                        cx,
                    )
                })
            })
            .log_err();

        if let Some(handle) = detached {
            plan_store.update(cx, |store, _| {
                store.detached_window = Some(handle.into());
            });
        }
    }

    fn dock_back(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        let workspace_id = self.workspace_id;
        let workspace = self.workspace.clone();
        let focus_spec_path = self.plan_store.read(cx).focus_spec_path.clone();

        let Some(main_window) = multi_workspace_window_for(&workspace, cx) else {
            cx.defer(move |cx| close_detached_plan_window(workspace_id, cx));
            return;
        };

        cx.defer(move |cx| {
            close_detached_plan_window(workspace_id, cx);

            main_window
                .update(cx, |multi_workspace, window, cx| {
                    window.activate_window();
                    if multi_workspace.workspace().entity_id() != workspace.entity_id() {
                        multi_workspace.activate(workspace.clone(), None, window, cx);
                    }
                    multi_workspace.workspace().update(cx, |workspace, cx| {
                        AgentPanel::focus_plan_from_workspace(
                            workspace,
                            focus_spec_path,
                            window,
                            cx,
                        );
                    });
                })
                .log_err();
        });
    }

    fn refresh_manifest_watch_state(&mut self, cx: &App) {
        let Some(worktree_id) = self.manifest_worktree_id else {
            return;
        };
        let Some(store) = PlanManifestStore::try_global(cx) else {
            return;
        };
        let Some(generation) = store.generation(worktree_id) else {
            return;
        };
        if generation <= self.manifest_generation {
            return;
        }
        if let Some(loaded) = store.manifest(worktree_id) {
            self.manifest = Some(loaded);
            self.manifest_generation = generation;
            self.manifest_reloaded = true;
            self.build_track_artifact_indices = self
                .manifest
                .as_ref()
                .map(|loaded| {
                    loaded
                        .manifest
                        .artifacts
                        .iter()
                        .enumerate()
                        .filter_map(|(index, artifact)| {
                            (artifact.kind == ArtifactKind::BuildPhase).then_some(index)
                        })
                        .collect()
                })
                .unwrap_or_default();
        }
    }

    fn reload_manifest_from_store(&mut self, cx: &App) {
        let Some(worktree_id) = self.manifest_worktree_id else {
            return;
        };
        if let Some(loaded) =
            PlanManifestStore::try_global(cx).and_then(|store| store.manifest(worktree_id))
        {
            self.manifest = Some(loaded);
            self.manifest_generation = PlanManifestStore::try_global(cx)
                .and_then(|store| store.generation(worktree_id))
                .unwrap_or(self.manifest_generation);
            self.manifest_reloaded = false;
        }
    }

    fn reload_build_track_progress(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(manifest) = self.manifest.clone() else {
            self.build_track_progress.clear();
            return;
        };
        let Some(worktree_root) = self.manifest_worktree_root.clone() else {
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            let mut progress = HashMap::new();
            for artifact in manifest.manifest.build_phases() {
                let absolute =
                    resolve_artifact_path(&worktree_root, &manifest.manifest.roots, artifact);
                let counts = count_checkboxes_in_file(&absolute).unwrap_or_default();
                progress.insert(artifact.id.clone(), counts);
            }

            this.update_in(cx, |this, _window, cx| {
                this.build_track_progress = progress;
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn selected_build_phase(&self, cx: &App) -> Option<(usize, cuecode_plans::Artifact)> {
        let manifest = self.manifest.as_ref()?;
        let artifact_index = self
            .build_track_artifact_indices
            .get(self.build_track_selection(cx))?;
        let artifact = manifest.manifest.artifacts.get(*artifact_index)?.clone();
        Some((*artifact_index, artifact))
    }

    fn can_implement_selected_phase(&self, cx: &App) -> bool {
        let Some((_, artifact)) = self.selected_build_phase(cx) else {
            return false;
        };
        self.manifest
            .as_ref()
            .is_some_and(|manifest| manifest.manifest.deps_satisfied(&artifact))
            && artifact.status != ArtifactStatus::Done
    }

    fn can_mark_selected_phase_done(&self, cx: &App) -> bool {
        let Some((_, artifact)) = self.selected_build_phase(cx) else {
            return false;
        };
        if artifact.status == ArtifactStatus::Done {
            return false;
        }
        self.build_track_progress
            .get(&artifact.id)
            .is_some_and(|progress| progress.all_checked())
    }

    fn implement_selected_phase(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((_, artifact)) = self.selected_build_phase(cx) else {
            return;
        };
        let Some(worktree_id) = self.manifest_worktree_id else {
            return;
        };
        let Some(worktree_root) = self.manifest_worktree_root.clone() else {
            return;
        };
        let artifact_id = artifact.id.clone().into();
        let result = self.workspace.update(cx, |workspace, cx| {
            implement_build_phase(
                workspace,
                worktree_id,
                worktree_root,
                artifact_id,
                window,
                cx,
            )
        });
        if let Err(error) = result {
            log::error!("failed to implement build phase: {error:#}");
            return;
        }
        self.reload_manifest_from_store(cx);
        self.reload_build_track_progress(window, cx);
        cx.notify();
    }

    fn mark_selected_phase_done(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some((_, artifact)) = self.selected_build_phase(cx) else {
            return;
        };
        let Some(worktree_id) = self.manifest_worktree_id else {
            return;
        };
        let Some(worktree_root) = self.manifest_worktree_root.clone() else {
            return;
        };
        if let Err(error) = mark_build_phase_done(&worktree_root, worktree_id, &artifact.id, cx) {
            log::error!("failed to mark build phase done: {error:#}");
            return;
        }
        self.reload_manifest_from_store(cx);
        self.reload_build_track_progress(window, cx);
        cx.notify();
    }

    fn render_shell(
        &self,
        id: &'static str,
        key_context: &'static str,
        body: impl IntoElement,
        footer: Option<impl IntoElement>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let show_detach = matches!(self.host, PlanningHubHost::Panel);
        let show_dock = matches!(self.host, PlanningHubHost::Detached);

        let mut shell = v_flex()
            .id(id)
            .key_context(key_context)
            .size_full()
            .overflow_hidden()
            .track_focus(&self.focus_handle);
        shell = match self.host {
            PlanningHubHost::Panel => shell.bg(cx.theme().colors().panel_background),
            PlanningHubHost::Detached => shell.elevation_3(cx),
        };
        shell
            .child(
                v_flex()
                    .flex_none()
                    .gap_2()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        h_flex()
                            .justify_between()
                            .gap_2()
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(self.render_tab_button(
                                        PlanHubTab::BuildTrack,
                                        "Build track",
                                        cx,
                                    ))
                                    .child(self.render_tab_button(
                                        PlanHubTab::AllArtifacts,
                                        "All artifacts",
                                        cx,
                                    )),
                            )
                            .child(
                                h_flex()
                                    .gap_1()
                                    .when(show_detach, |this| {
                                        this.child(
                                            IconButton::new("plan-open-window", IconName::ArrowUpRight)
                                                .icon_size(IconSize::Small)
                                                .tooltip(Tooltip::text("Open in New Window"))
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.open_detached_window(window, cx);
                                                })),
                                        )
                                    })
                                    .when(show_dock, |this| {
                                        this.child(
                                            Button::new("plan-dock-back", "Dock")
                                                .style(ButtonStyle::Outlined)
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.dock_back(window, cx);
                                                })),
                                        )
                                    }),
                            ),
                    ),
            )
            .child(div().flex_1().min_h_0().overflow_hidden().child(body))
            .children(footer.map(|footer| {
                div()
                    .flex_none()
                    .p_2()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(footer)
            }))
    }

    fn render_active_tab(
        &self,
        window: &mut Window,
        markdown_style: MarkdownStyle,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match self.active_tab(cx) {
            PlanHubTab::BuildTrack => self.render_build_track(cx).into_any_element(),
            PlanHubTab::AllArtifacts => self
                .render_artifacts_pane(window, markdown_style, cx)
                .into_any_element(),
        }
    }

    fn render_tab_button(
        &self,
        tab: PlanHubTab,
        label: &'static str,
        cx: &Context<Self>,
    ) -> impl IntoElement {
        let selected = self.active_tab(cx) == tab;
        Button::new(format!("planning-hub-tab-{label}"), label)
            .style(if selected {
                ButtonStyle::Filled
            } else {
                ButtonStyle::Outlined
            })
            .on_click(cx.listener(move |this, _, window, cx| {
                this.set_tab(tab, window, cx);
            }))
    }

    fn render_build_track(&self, cx: &Context<Self>) -> impl IntoElement {
        let Some(manifest) = self.manifest.as_ref() else {
            return v_flex()
                .gap_3()
                .p_4()
                .child(
                    Label::new("Build track needs `.cuecode/plans/project.yaml`")
                        .size(LabelSize::Large),
                )
                .child(
                    Label::new("Add a plan manifest or run Organize (phase 1.6) to adopt one.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                );
        };

        let build_track_selection = self.build_track_selection(cx);
        let suggested_next = manifest.manifest.effective_suggested_next().map(str::to_owned);
        let mut children = Vec::new();

        if self.manifest_reloaded {
            children.push(
                h_flex()
                    .gap_2()
                    .p_2()
                    .rounded_md()
                    .bg(cx.theme().colors().surface_background)
                    .child(
                        Label::new("Manifest changed on disk — reloaded from file.")
                            .size(LabelSize::Small)
                            .color(Color::Accent),
                    )
                    .into_any_element(),
            );
        }

        if self.build_track_artifact_indices.is_empty() {
            children.push(
                Label::new("No build phases in the manifest.")
                    .size(LabelSize::Small)
                    .color(Color::Muted)
                    .into_any_element(),
            );
        }

        for (row_ix, artifact_index) in self.build_track_artifact_indices.iter().enumerate() {
            let Some(artifact) = manifest.manifest.artifacts.get(*artifact_index) else {
                continue;
            };
            let selected = row_ix == build_track_selection;
            let deps_ready = manifest.manifest.deps_satisfied(artifact);
            let progress = self
                .build_track_progress
                .get(&artifact.id)
                .copied()
                .unwrap_or_default();
            let is_suggested = suggested_next.as_deref() == Some(artifact.id.as_str());
            let phase_status = artifact.status;

            let list_item = ListItem::new(("planning-hub-build-phase", row_ix))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .on_click(cx.listener(move |this, _, _window, cx| {
                    this.set_build_track_selection(row_ix, cx);
                }))
                .child(
                    h_flex()
                        .justify_between()
                        .gap_2()
                        .child(
                            v_flex()
                                .gap_0p5()
                                .child(Label::new(artifact.id.clone()).size(LabelSize::Small))
                                .child(
                                    Label::new(format!(
                                        "{} · tasks {}",
                                        build_track_status_label(artifact.status),
                                        progress.label()
                                    ))
                                    .size(LabelSize::XSmall)
                                    .when(
                                        phase_status == ArtifactStatus::Done,
                                        |label| label.color(Color::Success),
                                    )
                                    .when(
                                        phase_status != ArtifactStatus::Done,
                                        |label| label.color(Color::Muted),
                                    ),
                                ),
                        )
                        .child(
                            h_flex()
                                .gap_1()
                                .when(phase_status == ArtifactStatus::Done, |this| {
                                    this.child(
                                        Label::new("Done")
                                            .size(LabelSize::XSmall)
                                            .color(Color::Success),
                                    )
                                })
                                .when(is_suggested, |this| {
                                    this.child(
                                        Label::new("Suggested next")
                                            .size(LabelSize::XSmall)
                                            .color(Color::Accent),
                                    )
                                })
                                .when(!deps_ready, |this| {
                                    this.child(
                                        Label::new("Blocked")
                                            .size(LabelSize::XSmall)
                                            .color(Color::Warning),
                                    )
                                }),
                        ),
                );

            children.push(
                div()
                    .w_full()
                    .rounded_sm()
                    .when(phase_status == ArtifactStatus::Done, |this| {
                        this.border_1()
                            .border_color(cx.theme().status().success_border)
                            .bg(cx.theme().status().success_background.opacity(0.12))
                    })
                    .when(
                        phase_status == ArtifactStatus::InProgress && !selected,
                        |this| {
                            this.border_1()
                                .border_color(cx.theme().colors().border_focused)
                        },
                    )
                    .child(list_item)
                    .into_any_element(),
            );
        }

        v_flex().gap_2().p_2().children(children)
    }

    fn render_artifacts_pane(
        &self,
        window: &mut Window,
        markdown_style: MarkdownStyle,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .size_full()
            .gap_0()
            .child(
                v_flex()
                    .w(rems(18.))
                    .h_full()
                    .min_w_0()
                    .border_r_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        div()
                            .p_2()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(self.search_editor.clone()),
                    )
                    .child(
                        div().flex_1().min_h_0().overflow_hidden().child(
                            if self.filtered_indices.is_empty() {
                                div()
                                    .p_4()
                                    .child(
                                        Label::new(
                                            if self.entries.is_empty() {
                                                "No specs found under `.cursor/specs/`."
                                            } else {
                                                "No artifacts match your search."
                                            },
                                        )
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                    )
                                    .into_any_element()
                            } else {
                                list(
                                    self.list_state.clone(),
                                    cx.processor(|this, ix, _window, cx| {
                                        this.render_list_row(ix, cx).into_any_element()
                                    }),
                                )
                                .flex_1()
                                .into_any_element()
                            },
                        ),
                    ),
            )
            .child(
                v_flex()
                    .flex_1()
                    .h_full()
                    .min_w_0()
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .border_b_1()
                            .border_color(cx.theme().colors().border_variant)
                            .child(
                                Label::new(self.preview_title.clone()).size(LabelSize::Large),
                            ),
                    )
                    .child({
                        let mut preview_children = Vec::new();
                        if let Some(error) = &self.preview_error {
                            preview_children.push(
                                Label::new(error.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Error)
                                    .into_any_element(),
                            );
                        }
                        if let Some(markdown) = &self.preview_markdown {
                            preview_children.push(
                                MarkdownElement::new(markdown.clone(), markdown_style.clone())
                                    .into_any_element(),
                            );
                        }
                        div()
                            .flex_1()
                            .min_h_0()
                            .vertical_scrollbar_for(&self.preview_scroll_handle, window, cx)
                            .child(
                                v_flex()
                                    .id("planning-hub-preview")
                                    .px_4()
                                    .py_3()
                                    .gap_2()
                                    .overflow_y_scroll()
                                    .track_scroll(&self.preview_scroll_handle)
                                    .children(preview_children),
                            )
                    }),
            )
    }

    fn render_list_row(&self, ix: usize, cx: &Context<Self>) -> impl IntoElement {
        let Some(candidate_index) = self.filtered_indices.get(ix) else {
            return div().into_any_element();
        };
        let Some(entry) = self.entries.get(*candidate_index) else {
            return div().into_any_element();
        };
        let selected = ix == self.selected_index(cx);

        ListItem::new(("planning-hub-item", *candidate_index))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected)
            .on_click(cx.listener(move |this, _, window, cx| {
                this.select_index(ix, window, cx);
            }))
            .child(
                v_flex()
                    .gap_0p5()
                    .child(Label::new(entry.title.clone()).size(LabelSize::Small))
                    .child(
                        Label::new(format!(
                            "{} · {}",
                            entry.path.display(),
                            status_label(entry.status.as_ref())
                        ))
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                    ),
            )
            .into_any_element()
    }

    fn render_footer(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        match self.active_tab(cx) {
            PlanHubTab::BuildTrack => Some(
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("planning-hub-implement", "Implement phase")
                            .style(ButtonStyle::Filled)
                            .disabled(!self.can_implement_selected_phase(cx))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.implement_selected_phase(window, cx);
                            })),
                    )
                    .child(
                        Button::new("planning-hub-mark-done", "Mark phase done?")
                            .style(ButtonStyle::Outlined)
                            .disabled(!self.can_mark_selected_phase_done(cx))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.mark_selected_phase_done(window, cx);
                            })),
                    )
                    .into_any_element(),
            ),
            PlanHubTab::AllArtifacts => Some(
                Button::new("planning-hub-open", "Open")
                    .style(ButtonStyle::Filled)
                    .disabled(self.selected_entry(cx).is_none())
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.open_selected(window, cx);
                    }))
                    .into_any_element(),
            ),
        }
    }
}

impl Focusable for PlanningHubView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PlanningHubView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.refresh_manifest_watch_state(cx);

        let markdown_style = MarkdownStyle::themed(MarkdownFont::Agent, window, cx);
        let footer = self.render_footer(cx);

        self.render_shell(
            "planning-hub-view",
            "PlanningHubView",
            self.render_active_tab(window, markdown_style, cx),
            footer,
            window,
            cx,
        )
    }
}

pub struct PlanningHubModal {
    workspace: Entity<Workspace>,
    focus_handle: FocusHandle,
}

impl PlanningHubModal {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace
            .register_action(
                |workspace: &mut Workspace,
                 action: &cuecode_actions::FocusPlan,
                 window: &mut Window,
                 cx: &mut Context<Workspace>| {
                    AgentPanel::focus_plan_from_workspace(
                        workspace,
                        action.focus_spec_path.clone(),
                        window,
                        cx,
                    );
                },
            )
            .register_action(
                |workspace: &mut Workspace,
                 _: &cuecode_actions::FocusAgentChat,
                 window: &mut Window,
                 cx: &mut Context<Workspace>| {
                    AgentPanel::focus_agent_chat_from_workspace(workspace, window, cx);
                },
            )
            .register_action(
                |workspace: &mut Workspace,
                 _: &cuecode_actions::OpenAgentChatInNewWindow,
                 window: &mut Window,
                 cx: &mut Context<Workspace>| {
                    if let Some(panel) = workspace.panel::<AgentPanel>(cx) {
                        panel.update(cx, |panel, cx| {
                            panel.open_detached_threads_window(window, cx);
                        });
                    }
                },
            )
            .register_action(
                |workspace: &mut Workspace,
                 _: &cuecode_actions::FocusTerminal,
                 window: &mut Window,
                 cx: &mut Context<Workspace>| {
                    AgentPanel::focus_terminal_from_workspace(workspace, window, cx);
                },
            )
            .register_action(
                |workspace: &mut Workspace,
                 action: &cuecode_actions::OpenPlanningHub,
                 window: &mut Window,
                 cx: &mut Context<Workspace>| {
                    ensure_spec_indices_loaded(workspace, cx);
                    if has_project_manifest(workspace, cx)
                        || PlanningHubOrganizeDismissed::dismissed(cx)
                    {
                        AgentPanel::focus_plan_from_workspace(
                            workspace,
                            action.focus_spec_path.clone(),
                            window,
                            cx,
                        );
                    } else {
                        let workspace_entity = cx.entity();
                        workspace.toggle_modal(window, cx, move |_window, cx| {
                            PlanningHubModal::new(workspace_entity, cx)
                        });
                    }
                },
            );
    }

    fn new(workspace: Entity<Workspace>, cx: &mut Context<Self>) -> Self {
        Self {
            workspace,
            focus_handle: cx.focus_handle(),
        }
    }

    fn dismiss_organize_prompt(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        PlanningHubOrganizeDismissed::dismiss(cx);
        cx.emit(DismissEvent);
        let workspace = self.workspace.clone();
        let Some(window_handle) = window.window_handle().downcast::<MultiWorkspace>() else {
            return;
        };
        cx.defer(move |cx| {
            window_handle
                .update(cx, |_, window, cx| {
                    workspace.update(cx, |workspace, cx| {
                        AgentPanel::focus_plan_from_workspace(workspace, None, window, cx);
                    });
                })
                .ok();
        });
    }

    fn cancel(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for PlanningHubModal {}

impl Focusable for PlanningHubModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for PlanningHubModal {}

impl Render for PlanningHubModal {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("planning-hub-organize-modal")
            .key_context("PlanningHubModal")
            .w(rems(48.))
            .elevation_3(cx)
            .overflow_hidden()
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::cancel))
            .capture_any_mouse_down(cx.listener(|this, _, window, cx| {
                this.focus_handle.focus(window, cx);
            }))
            .child(
                Modal::new("planning-hub-organize", None)
                    .header(
                        ModalHeader::new()
                            .headline("Plan")
                            .description("No `.cuecode/project.yaml` yet.")
                            .show_dismiss_button(true),
                    )
                    .section(
                        Section::new().child(
                            v_flex()
                                .gap_4()
                                .p_2()
                                .child(
                                    Label::new(
                                        "We can infer planning docs across your repo and propose a build track.",
                                    )
                                    .size(LabelSize::Default),
                                )
                                .child(
                                    Label::new(
                                        "Organize ships in build phase 1.6 — browse existing specs meanwhile.",
                                    )
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                                ),
                        ),
                    )
                    .footer(
                        ModalFooter::new().end_slot(
                            h_flex()
                                .gap_2()
                                .child(
                                    Button::new("planning-hub-organize", "Organize this project")
                                        .style(ButtonStyle::Filled)
                                        .disabled(true)
                                        .tooltip(Tooltip::text("Coming in build phase 1.6")),
                                )
                                .child(
                                    Button::new("planning-hub-skip", "Skip for now")
                                        .style(ButtonStyle::Outlined)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.dismiss_organize_prompt(window, cx);
                                        })),
                                )
                                .child(
                                    Button::new("planning-hub-browse", "Browse specs")
                                        .style(ButtonStyle::Transparent)
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.dismiss_organize_prompt(window, cx);
                                        })),
                                ),
                        ),
                    ),
            )
            .into_any_element()
    }
}

fn paths_match(entry_path: &Path, focus: &str) -> bool {
    entry_path == Path::new(focus)
        || entry_path.to_string_lossy() == focus
        || entry_path.ends_with(focus)
}

pub fn close_detached_plan_window(workspace_id: WorkspaceId, cx: &mut App) {
    let store = plan_store_for(workspace_id, cx);
    if let Some(handle) = store.update(cx, |store, _| store.detached_window.take()) {
        let _ = handle.update(cx, |_, window, _| window.remove_window());
    }
}

pub fn multi_workspace_window_for(
    workspace: &Entity<Workspace>,
    cx: &App,
) -> Option<WindowHandle<MultiWorkspace>> {
    let workspace_entity_id = workspace.entity_id();
    cx.windows().into_iter().find_map(|window| {
        let handle = window.downcast::<MultiWorkspace>()?;
        handle.read(cx).ok().and_then(|multi_workspace| {
            let owns_workspace = multi_workspace.workspace().entity_id() == workspace_entity_id
                || multi_workspace
                    .workspaces()
                    .any(|candidate| candidate.entity_id() == workspace_entity_id);
            owns_workspace.then_some(handle)
        })
    })
}
