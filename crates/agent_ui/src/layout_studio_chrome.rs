//! Read-only Layout Studio chrome for component preview and visual snapshots.

use agent_settings::{AgentHost, LayoutBlock, LayoutBlueprint};
use gpui::{App, IntoElement, Window, div, prelude::*};
use settings::{DockPosition, DockSide};
use ui::{Button, ButtonStyle, Label, LabelSize, prelude::*};

#[derive(Clone, Debug)]
pub struct LayoutStudioSnapshot {
    pub blueprint: LayoutBlueprint,
    pub selected_block: Option<LayoutBlock>,
    pub show_advanced: bool,
    pub error_message: Option<String>,
}

impl LayoutStudioSnapshot {
    pub fn new(blueprint: LayoutBlueprint) -> Self {
        Self {
            blueprint,
            selected_block: None,
            show_advanced: false,
            error_message: None,
        }
    }

    pub fn with_selected_block(mut self, block: LayoutBlock) -> Self {
        self.selected_block = Some(block);
        self
    }

    pub fn with_advanced(mut self) -> Self {
        self.show_advanced = true;
        self
    }

    pub fn with_error(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }

    fn status_label(&self) -> &'static str {
        "Workspace layout"
    }
}

pub fn render_layout_studio_snapshot(
    snapshot: &LayoutStudioSnapshot,
    _window: &Window,
    cx: &App,
) -> impl IntoElement {
    v_flex()
        .w(px(760.))
        .rounded_lg()
        .bg(cx.theme().colors().panel_background)
        .border_1()
        .border_color(cx.theme().colors().border_variant)
        .child(
            v_flex()
                .p_4()
                .gap_4()
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
                .child(render_dollhouse(snapshot, cx))
                .when(snapshot.show_advanced, |this| {
                    this.child(render_advanced(snapshot, cx))
                })
                .when_some(snapshot.error_message.clone(), |this, message| {
                    this.child(
                        Label::new(message)
                            .size(LabelSize::Small)
                            .color(Color::Error),
                    )
                })
                .child(
                    Label::new(snapshot.status_label())
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                ),
        )
}

fn block_label(block: LayoutBlock) -> &'static str {
    match block {
        LayoutBlock::Threads => "Threads",
        LayoutBlock::Nav => "Nav",
        LayoutBlock::Plan => "Plan",
        LayoutBlock::Agent => "Agent",
        LayoutBlock::Editor => "Editor",
    }
}

fn block_is_wide(block: LayoutBlock) -> bool {
    matches!(
        block,
        LayoutBlock::Plan | LayoutBlock::Editor | LayoutBlock::Agent
    )
}

fn render_block(
    snapshot: &LayoutStudioSnapshot,
    block: LayoutBlock,
    cx: &App,
) -> impl IntoElement {
    let blueprint = &snapshot.blueprint;
    let label = block_label(block);
    let wide = block_is_wide(block);
    let ghost = blueprint.block_is_ghost(block);
    let status = blueprint.block_status_label(block);
    let selected = snapshot.selected_block == Some(block);

    div()
        .flex()
        .when(wide, |this| this.flex_1())
        .when(!wide, |this| this.w(px(56.)).flex_none())
        .min_w(px(48.))
        .h(px(112.))
        .mx_1()
        .rounded_sm()
        .border_1()
        .when(ghost, |this| this.border_dashed())
        .border_color(if selected {
            cx.theme().colors().border_selected
        } else {
            cx.theme().colors().border_variant
        })
        .bg(if ghost {
            cx.theme().colors().elevated_surface_background
        } else {
            cx.theme().colors().panel_background
        })
        .child(
            v_flex()
                .size_full()
                .p_1()
                .gap_1()
                .child(
                    Label::new(label)
                        .size(LabelSize::XSmall)
                        .color(if ghost {
                            Color::Muted
                        } else {
                            Color::Default
                        }),
                )
                .child(
                    Label::new(status)
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                )
                .child(if selected {
                    v_flex()
                        .flex_1()
                        .gap(px(2.))
                        .children(
                            blueprint
                                .placement_options_for(block)
                                .into_iter()
                                .map(|option| {
                                    let active =
                                        blueprint.is_placement_active(block, option.action);
                                    div().w_full().child(
                                        Button::new(
                                            SharedString::from(format!(
                                                "preview-block-menu-{}-{}",
                                                label, option.label
                                            )),
                                            option.label,
                                        )
                                        .style(if active {
                                            ButtonStyle::Filled
                                        } else {
                                            ButtonStyle::Outlined
                                        })
                                        .label_size(LabelSize::XSmall)
                                        .disabled(true),
                                    )
                                }),
                        )
                        .into_any_element()
                } else {
                    div()
                        .flex_1()
                        .rounded_xs()
                        .bg(cx.theme().colors().element_background)
                        .into_any_element()
                }),
        )
}

fn render_display_toggle(snapshot: &LayoutStudioSnapshot, _cx: &App) -> impl IntoElement {
    let single = !snapshot.blueprint.multiple_displays;
    let multiple = snapshot.blueprint.multiple_displays;

    h_flex()
        .gap_1()
        .child(
            Button::new("preview-layout-single-display", "Single display")
                .style(if single {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Outlined
                })
                .label_size(LabelSize::XSmall)
                .disabled(true),
        )
        .child(
            Button::new("preview-layout-multiple-displays", "Multiple displays")
                .style(if multiple {
                    ButtonStyle::Filled
                } else {
                    ButtonStyle::Outlined
                })
                .label_size(LabelSize::XSmall)
                .disabled(true),
        )
}

fn render_advanced(snapshot: &LayoutStudioSnapshot, _cx: &App) -> impl IntoElement {
    let agent_dock = match &snapshot.blueprint.agent {
        AgentHost::Column { side, .. } => match side {
            DockSide::Left => DockPosition::Left,
            DockSide::Right => DockPosition::Right,
        },
        AgentHost::Bottom { .. } => DockPosition::Bottom,
        AgentHost::Detached { .. } => DockPosition::Right,
    };

    v_flex()
        .gap_2()
        .child(
            Label::new("Panel Position")
                .size(LabelSize::Small)
                .color(Color::Muted),
        )
        .child(
            h_flex()
                .gap_1()
                .children(
                    [DockPosition::Left, DockPosition::Right, DockPosition::Bottom].map(|dock| {
                        let label = match dock {
                            DockPosition::Left => "Dock Left",
                            DockPosition::Right => "Dock Right",
                            DockPosition::Bottom => "Dock Bottom",
                        };
                        Button::new(
                            SharedString::from(format!("preview-advanced-dock-{dock:?}")),
                            label,
                        )
                        .style(if agent_dock == dock {
                            ButtonStyle::Filled
                        } else {
                            ButtonStyle::Outlined
                        })
                        .label_size(LabelSize::XSmall)
                        .disabled(true)
                    }),
                ),
        )
}

fn render_dollhouse(snapshot: &LayoutStudioSnapshot, cx: &App) -> impl IntoElement {
    let blueprint = &snapshot.blueprint;
    let agent_bottom = matches!(blueprint.agent, AgentHost::Bottom { .. });
    let show_detached = blueprint.multiple_displays
        || blueprint.plan_detached()
        || blueprint.agent_detached();

    v_flex()
        .gap_2()
        .child(render_display_toggle(snapshot, cx))
        .child(
            div()
                .w_full()
                .rounded_md()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().elevated_surface_background)
                .p_2()
                .child(
                    v_flex()
                        .gap_2()
                        .child(
                            h_flex()
                                .w_full()
                                .min_h(px(112.))
                                .children([
                                    LayoutBlock::Threads,
                                    LayoutBlock::Plan,
                                    LayoutBlock::Editor,
                                    LayoutBlock::Agent,
                                    LayoutBlock::Nav,
                                ]
                                .into_iter()
                                .map(|block| render_block(snapshot, block, cx))),
                        )
                        .when(agent_bottom, |this| {
                            this.child(h_flex().child(render_block(
                                snapshot,
                                LayoutBlock::Agent,
                                cx,
                            )))
                        }),
                ),
        )
        .when(show_detached, |this| {
                this.child(
                    h_flex()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .h(px(64.))
                                .rounded_md()
                                .border_1()
                                .border_color(cx.theme().colors().border_variant)
                                .p_2()
                                .child(
                                    Label::new("Main Workspace")
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                ),
                        )
                        .child(
                            div()
                                .w(px(160.))
                                .h(px(64.))
                                .rounded_md()
                                .border_1()
                                .border_color(cx.theme().colors().border_selected)
                                .p_2()
                                .child(
                                    Label::new("Second Window")
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                ),
                        ),
                )
            },
        )
        .child(render_placement_hint(snapshot))
}

fn render_placement_hint(snapshot: &LayoutStudioSnapshot) -> impl IntoElement {
    match snapshot.selected_block {
        None => Label::new("Click a panel to choose where it goes.")
            .size(LabelSize::Small)
            .color(Color::Muted),
        Some(LayoutBlock::Editor) => Label::new("Editor stays in the center.")
            .size(LabelSize::Small)
            .color(Color::Muted),
        Some(_) => Label::new("Optional: drag ⋮⋮ to a highlighted zone.")
            .size(LabelSize::Small)
            .color(Color::Muted),
    }
}
