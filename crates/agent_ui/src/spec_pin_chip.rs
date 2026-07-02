use acp_thread::AcpThread;
use agent::Thread;
use gpui::{
    App, Context, Empty, Entity, FocusHandle, Focusable, SharedString, Subscription, WeakEntity,
    Window,
};
use ui::{Button, ButtonStyle, Icon, IconName, IconSize, LabelSize, Tooltip, prelude::*};
use workspace::Workspace;

use crate::spec_planning::spec_entries_for_workspace;

pub struct SpecPinChip {
    thread: Entity<AcpThread>,
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    entries: Vec<cuecode_specs::SpecEntry>,
    _subscriptions: Vec<Subscription>,
}

impl SpecPinChip {
    pub fn new(
        thread: Entity<AcpThread>,
        _native_thread: Option<WeakEntity<Thread>>,
        workspace: WeakEntity<Workspace>,
        focus_handle: FocusHandle,
        cx: &mut Context<Self>,
    ) -> Self {
        let entries = workspace
            .read_with(cx, |workspace, cx| spec_entries_for_workspace(workspace, cx))
            .unwrap_or_default();

        let thread_subscription = cx.observe(&thread, |_this, _thread, cx| {
            cx.notify();
        });

        Self {
            thread,
            workspace,
            focus_handle,
            entries,
            _subscriptions: vec![thread_subscription],
        }
    }

    fn refresh_entries(&mut self, cx: &mut Context<Self>) {
        if let Some(workspace) = self.workspace.upgrade() {
            workspace.update(cx, |workspace, cx| {
                crate::spec_planning::ensure_spec_indices_loaded(workspace, cx);
            });
            self.entries = workspace.read_with(cx, |workspace, cx| {
                spec_entries_for_workspace(workspace, cx)
            });
        }
    }

    fn active_spec_title(&self, cx: &App) -> Option<SharedString> {
        let path = self.thread.read(cx).active_spec_path()?;
        self.entries
            .iter()
            .find(|entry| entry.path.as_path() == path)
            .map(|entry| entry.title.clone().into())
            .or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .map(SharedString::from)
            })
    }

    fn active_spec_path_string(&self, cx: &App) -> Option<String> {
        self.thread
            .read(cx)
            .active_spec_path()
            .and_then(|path| path.to_str().map(String::from))
    }

    fn open_planning_hub(&self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let focus_spec_path = self.active_spec_path_string(cx);
        workspace.update(cx, |_workspace, cx| {
            window.dispatch_action(
                Box::new(cuecode_actions::FocusPlan {
                    focus_spec_path,
                }),
                cx,
            );
        });
    }
}

impl Focusable for SpecPinChip {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SpecPinChip {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.thread.read(cx).active_spec_path().is_none() {
            return Empty.into_any_element();
        }

        if self.entries.is_empty() {
            self.refresh_entries(cx);
        }

        let Some(title) = self.active_spec_title(cx) else {
            return Empty.into_any_element();
        };

        let label = format!("📎 {title}");
        Button::new("spec-pin-chip", label)
            .label_size(LabelSize::Small)
            .style(ButtonStyle::Subtle)
            .start_icon(
                Icon::new(IconName::FileMarkdown)
                    .size(IconSize::XSmall)
                    .color(Color::Default),
            )
            .tooltip(Tooltip::text("Open Plan"))
            .on_click(cx.listener(|this, _, window, cx| {
                this.open_planning_hub(window, cx);
            }))
            .into_any_element()
    }
}
