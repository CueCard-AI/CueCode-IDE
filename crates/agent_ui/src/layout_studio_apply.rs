use agent_settings::{
    AgentHost, LayoutBlueprint, LayoutPreset, NavPlacement, PlanHost, ThreadsPlacement,
};
use git_ui::git_panel::GitPanel;
use gpui::{App, Context, Window};
use outline_panel::OutlinePanel;
use project_panel::ProjectPanel;
use workspace::{MultiWorkspace, Workspace};

use crate::agent_panel::AgentPanel;

/// Open, close, and focus workspace surfaces so the live layout matches `blueprint`.
pub fn apply_blueprint_to_workspace(
    workspace: &mut Workspace,
    blueprint: &LayoutBlueprint,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    apply_threads_sidebar(blueprint, window, cx);
    apply_nav_panels(workspace, blueprint, window, cx);
    apply_agent_host(workspace, blueprint, window, cx);
}

fn apply_threads_sidebar(blueprint: &LayoutBlueprint, window: &mut Window, cx: &mut App) {
    let Some(multi_workspace) = window.window_handle().downcast::<MultiWorkspace>() else {
        return;
    };

    multi_workspace
        .update(cx, |multi_workspace, window, cx| {
            if !multi_workspace.multi_workspace_enabled(cx) {
                return;
            }

            match blueprint.threads {
                ThreadsPlacement::Hidden => {
                    multi_workspace.close_sidebar(window, cx);
                }
                ThreadsPlacement::Left | ThreadsPlacement::Right => {
                    multi_workspace.open_sidebar(cx);
                }
            }
        })
        .ok();
}

fn apply_nav_panels(
    workspace: &mut Workspace,
    blueprint: &LayoutBlueprint,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if blueprint.nav == NavPlacement::Hidden {
        workspace.close_panel::<ProjectPanel>(window, cx);
        workspace.close_panel::<OutlinePanel>(window, cx);
        workspace.close_panel::<GitPanel>(window, cx);
        return;
    }

    workspace.reveal_panel::<ProjectPanel>(window, cx);
    workspace.reveal_panel::<OutlinePanel>(window, cx);
    workspace.reveal_panel::<GitPanel>(window, cx);
}

fn apply_agent_host(
    workspace: &mut Workspace,
    blueprint: &LayoutBlueprint,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    if matches!(blueprint.agent, AgentHost::Detached { .. }) {
        workspace.close_panel::<AgentPanel>(window, cx);
        return;
    }

    if workspace.panel::<AgentPanel>(cx).is_none() {
        return;
    }

    let focus_plan = should_focus_plan(blueprint);
    let apply_implement_strip = blueprint.plan_strip_only
        || matches!(blueprint.preset, LayoutPreset::Implement);

    if focus_plan {
        AgentPanel::focus_plan_from_workspace(workspace, None, window, cx);
    } else {
        AgentPanel::focus_agent_chat_from_workspace(workspace, window, cx);
    }

    if apply_implement_strip {
        AgentPanel::apply_implement_layout_from_workspace(workspace, window, cx);
    }
}

fn should_focus_plan(blueprint: &LayoutBlueprint) -> bool {
    matches!(blueprint.preset, LayoutPreset::Plan)
        || matches!(blueprint.plan, PlanHost::Column { .. })
}
