use std::sync::Arc;

use agent_settings::{LayoutBlueprint, LayoutPreset, PanelLayout};
use fs::Fs;
use gpui::{
    Action, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    SharedString, Subscription, Window, WindowOptions, div, prelude::*,
};
use notifications::status_toast::StatusToast;
use settings::{SettingsStore, update_settings_file};
use ui::{Button, ButtonStyle, IconButton, Label, LabelSize, prelude::*};
use util::ResultExt as _;
use workspace::{ModalView, Workspace, WorkspaceId};

use crate::layout_studio_apply::apply_blueprint_to_workspace;
use crate::layout_studio_editor::LayoutStudioEditor;
use crate::plan_store::plan_store_for;
use crate::planning_hub::PlanningHubView;
use crate::spec_planning::ensure_spec_indices_loaded;

pub struct LayoutStudioModal {
    fs: Arc<dyn Fs>,
    workspace: Entity<Workspace>,
    workspace_id: WorkspaceId,
    focus_handle: FocusHandle,
    editor: Entity<LayoutStudioEditor>,
    saved_blueprint: LayoutBlueprint,
    show_discard_confirm: bool,
    _settings_subscription: Subscription,
}

impl LayoutStudioModal {
    pub fn register(
        workspace: &mut Workspace,
        _window: Option<&mut Window>,
        _cx: &mut Context<Workspace>,
    ) {
        workspace.register_action(
            |workspace: &mut Workspace,
             _: &cuecode_actions::ArrangeWorkspace,
             window: &mut Window,
             cx: &mut Context<Workspace>| {
                let fs = workspace.app_state().fs.clone();
                let workspace_entity = cx.entity();
                let workspace_id = workspace
                    .database_id()
                    .expect("layout studio requires a persisted workspace");
                workspace.toggle_modal(window, cx, |window, cx| {
                    Self::new(fs, workspace_entity, workspace_id, window, cx)
                });
            },
        );
    }

    #[allow(dead_code)]
    pub fn open(workspace: &Entity<Workspace>, window: &mut Window, cx: &mut Context<Workspace>) {
        let fs = workspace.read(cx).app_state().fs.clone();
        let workspace = workspace.clone();
        workspace.update(cx, |workspace, cx| {
            let entity = cx.entity();
            let workspace_id = workspace
                .database_id()
                .expect("layout studio requires a persisted workspace");
            workspace.toggle_modal(window, cx, |window, cx| {
                Self::new(fs, entity, workspace_id, window, cx)
            });
        });
    }

    fn new(
        fs: Arc<dyn Fs>,
        workspace: Entity<Workspace>,
        workspace_id: WorkspaceId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let merged = cx.global::<SettingsStore>().merged_settings();
        let blueprint = LayoutBlueprint::from_merged_settings(merged);
        let saved_blueprint = blueprint.clone();
        let editor = cx.new(|cx| LayoutStudioEditor::new(blueprint, cx));

        let settings_subscription =
            cx.observe_global_in::<SettingsStore>(window, |this, _window, cx| {
                let editor_preset = this.editor.read(cx).blueprint().preset;
                if editor_preset != LayoutPreset::Custom {
                    let merged = cx.global::<SettingsStore>().merged_settings();
                    this.saved_blueprint = LayoutBlueprint::from_merged_settings(merged);
                }
                cx.notify();
            });

        Self {
            fs,
            workspace,
            workspace_id,
            focus_handle: cx.focus_handle(),
            editor,
            saved_blueprint,
            show_discard_confirm: false,
            _settings_subscription: settings_subscription,
        }
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.editor
            .read(cx)
            .blueprint()
            .has_unsaved_changes(&self.saved_blueprint)
    }

    fn status_label(&self, cx: &App) -> SharedString {
        if self.is_dirty(cx) {
            "Unsaved changes".into()
        } else {
            "Workspace layout".into()
        }
    }

    fn can_apply(&self, cx: &App) -> bool {
        self.editor.read(cx).can_apply(cx) && self.is_dirty(cx)
    }

    fn request_dismiss(&mut self, cx: &mut Context<Self>) {
        if self.is_dirty(cx) {
            self.show_discard_confirm = true;
            cx.notify();
        } else {
            cx.emit(DismissEvent);
        }
    }

    fn confirm_discard(&mut self, cx: &mut Context<Self>) {
        self.show_discard_confirm = false;
        cx.emit(DismissEvent);
    }

    fn cancel_discard(&mut self, cx: &mut Context<Self>) {
        self.show_discard_confirm = false;
        cx.notify();
    }

    fn apply_layout(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_apply(cx) {
            let editor = self.editor.read(cx);
            if editor.validate().is_err() {
                self.editor.update(cx, |editor, cx| {
                    editor.show_validation_error(cx);
                });
            }
            return;
        }

        let blueprint = self.editor.read(cx).blueprint().clone();
        let detach_plan = blueprint.plan_detached();
        let merged =
            PanelLayout::from_merged_settings(cx.global::<SettingsStore>().merged_settings());
        let toast_message: SharedString = match blueprint.editor_crowding_warning() {
            Some(warning) => format!("{}\n{}", blueprint.apply_summary(), warning).into(),
            None => blueprint.apply_summary().into(),
        };
        let fs = self.fs.clone();
        let blueprint_for_settings = blueprint.clone();
        update_settings_file(fs, cx, move |settings, _| {
            blueprint_for_settings.write_settings(&merged, settings);
        });

        let workspace = self.workspace.clone();
        let workspace_id = self.workspace_id;

        cx.defer_in(window, move |_, window, cx| {
            if detach_plan {
                open_detached_plan_window(&workspace, workspace_id, cx);
            }

            workspace.update(cx, |workspace, cx| {
                apply_blueprint_to_workspace(workspace, &blueprint, window, cx);

                let toast = StatusToast::new(toast_message, cx, |this, _| this);
                workspace.toggle_status_toast(toast, cx);
            });
        });

        self.saved_blueprint = self.editor.read(cx).blueprint().clone();
        cx.emit(DismissEvent);
    }

    fn render_discard_confirm(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(gpui::black().opacity(0.45))
            .child(
                v_flex()
                    .w(px(360.))
                    .gap_3()
                    .p_4()
                    .rounded_lg()
                    .bg(cx.theme().colors().panel_background)
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(Label::new("Discard layout changes?").size(LabelSize::Large))
                    .child(
                        Label::new("Your custom layout will be lost.")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .child(
                                Button::new("layout-keep-editing", "Keep editing")
                                    .style(ButtonStyle::Outlined)
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.cancel_discard(cx);
                                    })),
                            )
                            .child(
                                Button::new("layout-discard", "Discard")
                                    .style(ButtonStyle::Filled)
                                    .label_size(LabelSize::Small)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.confirm_discard(cx);
                                    })),
                            ),
                    ),
            )
    }
}

impl ModalView for LayoutStudioModal {
    fn fade_out_background(&self) -> bool {
        true
    }
}

impl Focusable for LayoutStudioModal {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for LayoutStudioModal {}

impl Render for LayoutStudioModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.status_label(cx);

        div()
            .key_context("LayoutStudio")
            .track_focus(&self.focus_handle)
            .relative()
            .w(px(760.))
            .elevation_3(cx)
            .rounded_lg()
            .bg(cx.theme().colors().panel_background)
            .border_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                v_flex()
                    .max_h(vh(0.9, window))
                    .p_4()
                    .gap_4()
                    .child(
                        h_flex()
                            .justify_between()
                            .items_start()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(Label::new("Arrange Workspace").size(LabelSize::Large))
                                    .child(
                                        Label::new(
                                            "Click a panel header, then use arrows to move it.",
                                        )
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                    ),
                            )
                            .child(
                                IconButton::new("layout-studio-close", IconName::Close)
                                    .icon_size(IconSize::Small)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.request_dismiss(cx);
                                    })),
                            ),
                    )
                    .child(self.editor.clone())
                    .child(
                        h_flex()
                            .justify_between()
                            .items_center()
                            .child(
                                Label::new(status)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                h_flex()
                                    .gap_2()
                                    .child(
                                        Button::new("layout-reset", "Reset")
                                            .style(ButtonStyle::Outlined)
                                            .label_size(LabelSize::Small)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                let saved = this.saved_blueprint.clone();
                                                this.editor.update(cx, |editor, cx| {
                                                    editor.reset_blueprint(saved, cx);
                                                });
                                            }))
                                    )
                                    .child(
                                        Button::new("layout-cancel", "Cancel")
                                            .style(ButtonStyle::Outlined)
                                            .label_size(LabelSize::Small)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.request_dismiss(cx);
                                            })),
                                    )
                                    .child({
                                        let can_apply = self.can_apply(cx);
                                        Button::new("layout-apply", "Apply Layout")
                                            .style(ButtonStyle::Filled)
                                            .label_size(LabelSize::Small)
                                            .disabled(!can_apply)
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.apply_layout(window, cx);
                                            }))
                                    }),
                            ),
                    ),
            )
            .when(self.show_discard_confirm, |this| {
                this.child(self.render_discard_confirm(cx))
            })
            .on_action(cx.listener(|this, _: &menu::Cancel, _, cx| {
                this.request_dismiss(cx);
            }))
    }
}

pub fn open_detached_plan_window(
    workspace: &Entity<Workspace>,
    workspace_id: WorkspaceId,
    cx: &mut App,
) {
    let store = plan_store_for(workspace_id, cx);
    if let Some(handle) = store.read(cx).detached_window {
        if handle.update(cx, |_, window, _| window.activate_window()).is_ok() {
            return;
        }
        store.update(cx, |store, _| {
            store.detached_window = None;
        });
    }

    workspace.update(cx, |workspace, cx| {
        ensure_spec_indices_loaded(workspace, cx);
    });

    let workspace = workspace.clone();
    let plan_store = plan_store_for(workspace_id, cx);

    let detached = cx
        .open_window(WindowOptions::default(), move |window, cx| {
            cx.new(|cx| {
                PlanningHubView::new_for_detached(workspace.clone(), workspace_id, window, cx)
            })
        })
        .log_err();

    if let Some(handle) = detached {
        plan_store.update(cx, |store, _| {
            store.detached_window = Some(handle.into());
        });
    }
}

pub fn prompt_layout_studio_for_plan(
    workspace: &Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    prompt_layout_studio(
        workspace,
        window,
        cx,
        "layout_studio_plan_prompt_dismissed",
        "Set up your workspace for planning?",
    );
}

pub fn prompt_layout_studio_for_implement(
    workspace: &Entity<Workspace>,
    window: &mut Window,
    cx: &mut App,
) {
    prompt_layout_studio(
        workspace,
        window,
        cx,
        "layout_studio_implement_prompt_dismissed",
        "Switch to Implement layout?",
    );
}

fn layout_studio_prompt_seen(key: &str, cx: &App) -> bool {
    use db::kvp::KeyValueStore;

    util::ResultExt::log_err(KeyValueStore::global(cx).read_kvp(key))
        .flatten()
        .is_some()
}

fn mark_layout_studio_prompt_seen(key: &str, cx: &mut App) {
    use db::kvp::KeyValueStore;

    let db = KeyValueStore::global(cx);
    let key = key.to_string();
    db::write_and_log(cx, move || async move {
        db.write_kvp(key, "1".to_string()).await
    });
}

fn prompt_layout_studio(
    workspace: &Entity<Workspace>,
    _window: &mut Window,
    cx: &mut App,
    key: &str,
    message: &str,
) {
    if layout_studio_prompt_seen(key, cx) {
        return;
    }

    mark_layout_studio_prompt_seen(key, cx);

    workspace.update(cx, |workspace, cx| {
        let toast = StatusToast::new(message, cx, |this, _| {
            this.action("Arrange Workspace", |window, cx| {
                window.dispatch_action(cuecode_actions::ArrangeWorkspace.boxed_clone(), cx);
            })
        });
        workspace.toggle_status_toast(toast, cx);
    });
}
