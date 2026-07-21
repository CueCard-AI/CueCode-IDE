use std::time::Duration;

use agent_settings::{
    AgentHost, ControlPanelSegment, DisplayTarget, LayoutBlock, LayoutBlueprint, LayoutPreset,
    LayoutValidationError, PlacementOutcome, PlacementStepDirection, PlanHost,
};
use gpui::{
    Animation, AnimationExt as _, App, Context, FocusHandle, Focusable, MouseButton, Render,
    Window, div, ease_in_out, prelude::*,
};
use settings::DockSide;
use ui::{
    Button, ButtonStyle, IconButton, IconButtonShape, IconName, IconSize, Label, LabelSize,
    Tooltip, prelude::*,
};

const NARROW_BLOCK_WIDTH: Pixels = px(72.);
const NARROW_BLOCK_SELECTED_WIDTH: Pixels = px(104.);
const OFF_CHIP_WIDTH: Pixels = px(88.);
const BLOCK_HEIGHT: Pixels = px(128.);

enum CardFeedback {
    Notice(String),
    Error(String),
}

pub struct LayoutStudioEditor {
    focus_handle: FocusHandle,
    pub blueprint: LayoutBlueprint,
    selected_block: Option<LayoutBlock>,
    card_feedback: Option<(LayoutBlock, CardFeedback)>,
    morph_epoch: u64,
}

impl LayoutStudioEditor {
    pub fn new(blueprint: LayoutBlueprint, cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            blueprint,
            selected_block: None,
            card_feedback: None,
            morph_epoch: 0,
        }
    }

    pub fn blueprint(&self) -> &LayoutBlueprint {
        &self.blueprint
    }

    pub fn is_interacting(&self, _cx: &App) -> bool {
        false
    }

    pub fn can_apply(&self, cx: &App) -> bool {
        self.validate().is_ok() && !self.is_interacting(cx)
    }

    pub fn show_validation_error(&mut self, cx: &mut Context<Self>) {
        self.card_feedback = Some((
            LayoutBlock::Plan,
            CardFeedback::Error("Plan and Agent can't share the same side".into()),
        ));
        cx.notify();
    }

    fn block_is_wide(block: LayoutBlock) -> bool {
        matches!(
            block,
            LayoutBlock::Plan | LayoutBlock::Editor | LayoutBlock::Agent
        )
    }

    pub fn reset_blueprint(&mut self, blueprint: LayoutBlueprint, cx: &mut Context<Self>) {
        self.blueprint = blueprint;
        self.selected_block = None;
        self.card_feedback = None;
        self.morph_epoch = self.morph_epoch.saturating_add(1);
        cx.notify();
    }

    fn select_block(&mut self, block: LayoutBlock, cx: &mut Context<Self>) {
        self.selected_block = Some(block);
        cx.notify();
    }

    fn set_multiple_displays(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.blueprint.multiple_displays = enabled;
        if !enabled {
            if matches!(
                self.blueprint.plan,
                PlanHost::Detached {
                    display: DisplayTarget::Secondary
                }
            ) {
                self.blueprint.plan = PlanHost::Column {
                    side: DockSide::Left,
                    width: LayoutBlueprint::DEFAULT_PLAN_WIDTH,
                };
            }
            if matches!(
                self.blueprint.agent,
                AgentHost::Detached {
                    display: DisplayTarget::Secondary
                }
            ) {
                self.blueprint.agent = AgentHost::Column {
                    side: DockSide::Right,
                    width: LayoutBlueprint::DEFAULT_AGENT_WIDTH,
                };
            }
        }
        self.blueprint.preset = LayoutPreset::Custom;
        self.card_feedback = None;
        cx.notify();
    }

    fn set_card_outcome(&mut self, block: LayoutBlock, outcome: PlacementOutcome) {
        match outcome {
            PlacementOutcome::Changed => {
                self.card_feedback = None;
                self.selected_block = Some(block);
                self.morph_epoch = self.morph_epoch.saturating_add(1);
            }
            PlacementOutcome::AlreadyThere(message) => {
                self.card_feedback = Some((block, CardFeedback::Notice(message.into())));
            }
            PlacementOutcome::Invalid(error) => {
                self.card_feedback = Some((
                    block,
                    CardFeedback::Error(error.invalid_message().into()),
                ));
            }
        }
    }

    pub fn handle_placement_step(
        &mut self,
        block: LayoutBlock,
        direction: PlacementStepDirection,
        cx: &mut Context<Self>,
    ) {
        let outcome = self
            .blueprint
            .apply_placement_step(block, direction);
        self.set_card_outcome(block, outcome);
        cx.notify();
    }

    pub fn validate(&self) -> Result<(), LayoutValidationError> {
        self.blueprint.validate()
    }

    fn render_step_controls(
        &self,
        block: LayoutBlock,
        blueprint: &LayoutBlueprint,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        if blueprint.placement_ring(block).is_empty() {
            return div()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    Label::new("Fixed")
                        .size(LabelSize::XSmall)
                        .color(Color::Muted),
                );
        }

        h_flex()
            .size_full()
            .gap_1()
            .items_center()
            .justify_center()
            .child(
                IconButton::new(
                    SharedString::from(format!("layout-step-prev-{}", block_label(block))),
                    IconName::ArrowLeft,
                )
                .shape(IconButtonShape::Square)
                .icon_size(IconSize::XSmall)
                .style(ButtonStyle::Subtle)
                .tooltip(Tooltip::text("Previous placement"))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.handle_placement_step(block, PlacementStepDirection::Prev, cx);
                })),
            )
            .child(
                IconButton::new(
                    SharedString::from(format!("layout-step-next-{}", block_label(block))),
                    IconName::ArrowRight,
                )
                .shape(IconButtonShape::Square)
                .icon_size(IconSize::XSmall)
                .style(ButtonStyle::Subtle)
                .tooltip(Tooltip::text("Next placement"))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.handle_placement_step(block, PlacementStepDirection::Next, cx);
                })),
            )
    }

    fn render_card_feedback(&self, block: LayoutBlock) -> impl IntoElement {
        let Some((feedback_block, feedback)) = &self.card_feedback else {
            return div();
        };
        if *feedback_block != block {
            return div();
        }

        match feedback {
            CardFeedback::Notice(message) => div().child(
                Label::new(message.clone())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            ),
            CardFeedback::Error(message) => div().child(
                Label::new(message.clone())
                    .size(LabelSize::XSmall)
                    .color(Color::Error),
            ),
        }
    }

    fn render_block_card(
        &self,
        block: LayoutBlock,
        blueprint: &LayoutBlueprint,
        off_chip: bool,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let label = if off_chip {
            format!("{} (off)", block_label(block))
        } else {
            block_label(block).to_string()
        };
        let wide = Self::block_is_wide(block) && !off_chip;
        let ghost = blueprint.block_is_ghost(block) || off_chip;
        let status = blueprint.block_status_short(block);
        let selected = self.selected_block == Some(block);
        let show_plan_badge =
            block == LayoutBlock::Agent && blueprint.shows_plan_badge_on_agent() && !off_chip;

        div()
            .flex()
            .when(wide, |this| this.flex_1())
            .when(!wide, |this| {
                let width = if off_chip {
                    OFF_CHIP_WIDTH
                } else if selected {
                    NARROW_BLOCK_SELECTED_WIDTH
                } else {
                    NARROW_BLOCK_WIDTH
                };
                this.w(width).flex_none()
            })
            .min_w(if off_chip {
                OFF_CHIP_WIDTH
            } else if selected {
                NARROW_BLOCK_SELECTED_WIDTH
            } else {
                NARROW_BLOCK_WIDTH
            })
            .h(BLOCK_HEIGHT)
            .id(SharedString::from(format!("layout-block-{label}")))
            .opacity(if ghost { 0.72 } else { 1.0 })
            .child(
                div()
                    .size_full()
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
                            .child(
                                div()
                                    .h(px(28.))
                                    .px_1()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .flex_shrink_0()
                                    .cursor_pointer()
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, _, cx| {
                                            this.select_block(block, cx);
                                        }),
                                    )
                                    .child(
                                        Label::new(label.clone())
                                            .size(LabelSize::XSmall)
                                            .color(if ghost {
                                                Color::Muted
                                            } else {
                                                Color::Default
                                            }),
                                    )
                                    .when(show_plan_badge, |this| {
                                        this.child(
                                            div()
                                                .px_1()
                                                .py_px()
                                                .rounded_xs()
                                                .border_1()
                                                .border_color(cx.theme().colors().border_variant)
                                                .child(
                                                    Label::new("Plan tab")
                                                        .size(LabelSize::XSmall)
                                                        .color(Color::Muted),
                                                ),
                                        )
                                    })
                                    .child(
                                        IconButton::new(
                                            SharedString::from(format!(
                                                "layout-block-info-{label}"
                                            )),
                                            IconName::Info,
                                        )
                                        .icon_size(IconSize::XSmall)
                                        .tooltip(Tooltip::text(LayoutBlueprint::block_description(
                                            block,
                                        ))),
                                    ),
                            )
                            .child(
                                div()
                                    .px_1()
                                    .flex_shrink_0()
                                    .child(
                                        Label::new(status)
                                            .size(LabelSize::XSmall)
                                            .color(Color::Muted),
                                    ),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .p_1()
                                    .min_h_0()
                                    .child(self.render_step_controls(block, blueprint, cx)),
                            )
                            .child(
                                div()
                                    .px_1()
                                    .pb_1()
                                    .flex_shrink_0()
                                    .child(self.render_card_feedback(block)),
                            ),
                    ),
            )
    }

    fn render_segment(
        &self,
        segment: ControlPanelSegment,
        blueprint: &LayoutBlueprint,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        match segment {
            ControlPanelSegment::Block(block) => {
                self.render_block_card(block, blueprint, false, cx)
            }
            ControlPanelSegment::OffChip(block) => {
                self.render_block_card(block, blueprint, true, cx)
            }
        }
    }

    fn render_control_panel_row(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let blueprint = &self.blueprint;
        let segments = blueprint.control_panel_segments();

        h_flex()
            .id("layout-control-panel-row")
            .w_full()
            .gap_2()
            .items_start()
            .min_h(BLOCK_HEIGHT)
            .children(segments.into_iter().map(|segment| {
                self.render_segment(segment, blueprint, cx)
                    .into_any_element()
            }))
    }

    fn render_display_toggle(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let single = !self.blueprint.multiple_displays;
        let multiple = self.blueprint.multiple_displays;

        h_flex()
            .gap_1()
            .child(
                Button::new("layout-single-display", "Single display")
                    .style(if single {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Outlined
                    })
                    .label_size(LabelSize::XSmall)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.set_multiple_displays(false, cx);
                    })),
            )
            .child(
                Button::new("layout-multiple-displays", "Multiple displays")
                    .style(if multiple {
                        ButtonStyle::Filled
                    } else {
                        ButtonStyle::Outlined
                    })
                    .label_size(LabelSize::XSmall)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.set_multiple_displays(true, cx);
                    })),
            )
    }

    fn render_bottom_band(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let agent_bottom = matches!(self.blueprint.agent, AgentHost::Bottom { .. });

        h_flex().w_full().when(agent_bottom, |this| {
            this.child(self.render_block_card(
                LayoutBlock::Agent,
                &self.blueprint,
                false,
                cx,
            ))
        })
    }

    fn render_dollhouse(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let morph_id = SharedString::from(format!("layout-morph-{}", self.morph_epoch));
        let show_detached = self.blueprint.multiple_displays
            || self.blueprint.plan_detached()
            || self.blueprint.agent_detached();

        v_flex()
            .gap_2()
            .child(self.render_display_toggle(cx))
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
                                Label::new("Control panel — schematic")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .child(
                                Label::new("◀ ▶ steps through valid placements for each panel.")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                            .child(div().relative().w_full().child(self.render_control_panel_row(cx)))
                            .child(self.render_bottom_band(cx)),
                    )
                    .with_animation(
                        morph_id,
                        Animation::new(Duration::from_millis(280)).with_easing(ease_in_out),
                        |this, progress| this.opacity(0.2 + 0.8 * progress),
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
                                .border_color(cx.theme().colors().border_variant)
                                .p_2()
                                .flex()
                                .items_center()
                                .child(
                                    Label::new("Second window")
                                        .size(LabelSize::XSmall)
                                        .color(Color::Muted),
                                ),
                        ),
                )
            })
    }

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

impl Focusable for LayoutStudioEditor {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LayoutStudioEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .child(self.render_dollhouse(cx))
    }
}
