use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Render, Subscription, Window, div, prelude::*,
};
use ui::{Button, ButtonStyle, Label, LabelSize, prelude::*};
use workspace::{Workspace, WorkspaceId};

use crate::agent_panel::AgentPanel;
use crate::plan_store::plan_store_for;

pub struct DetachedAgentChatView {
    workspace: Entity<Workspace>,
    workspace_id: WorkspaceId,
    focus_handle: FocusHandle,
    _panel_subscription: Option<Subscription>,
}

impl DetachedAgentChatView {
    pub fn new(
        workspace: Entity<Workspace>,
        workspace_id: WorkspaceId,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();
        let panel_subscription = workspace.read(cx).panel::<AgentPanel>(cx).map(|panel| {
            cx.observe(&panel, |_, _, cx| cx.notify())
        });

        Self {
            workspace,
            workspace_id,
            focus_handle,
            _panel_subscription: panel_subscription,
        }
    }

    fn dock_back(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        AgentPanel::dock_threads_from_detached(self.workspace.clone(), self.workspace_id, cx);
    }
}

impl Focusable for DetachedAgentChatView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for DetachedAgentChatView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(panel) = self.workspace.read(cx).panel::<AgentPanel>(cx) else {
            return v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child(Label::new("Agent panel unavailable").color(Color::Muted))
                .into_any();
        };

        let conversation = panel.read(cx).active_conversation_view();

        v_flex()
            .key_context("DetachedAgentChat")
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .track_focus(&self.focus_handle)
            .child(
                h_flex()
                    .flex_none()
                    .justify_between()
                    .items_center()
                    .gap_2()
                    .p_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        Label::new("Agent Chat")
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Button::new("threads-dock-back", "Dock")
                            .style(ButtonStyle::Outlined)
                            .label_size(LabelSize::Small)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.dock_back(window, cx);
                            })),
                    ),
            )
            .child(
                div().flex_1().min_h_0().overflow_hidden().map(|container| {
                    if let Some(conversation_view) = conversation {
                        container.child(conversation_view.clone())
                    } else {
                        container
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Label::new("No active thread").color(Color::Muted))
                    }
                }),
            )
            .into_any()
    }
}

pub fn close_detached_threads_window(workspace_id: WorkspaceId, cx: &mut App) {
    let store = plan_store_for(workspace_id, cx);
    if let Some(handle) = store.update(cx, |store, _| store.detached_threads_window.take()) {
        let _ = handle.update(cx, |_, window, _| window.remove_window());
    }
}

pub fn activate_detached_threads_window(workspace_id: WorkspaceId, cx: &mut App) -> bool {
    let store = plan_store_for(workspace_id, cx);
    let Some(handle) = store.read(cx).detached_threads_window else {
        return false;
    };
    handle
        .update(cx, |_, window, _| window.activate_window())
        .is_ok()
}
