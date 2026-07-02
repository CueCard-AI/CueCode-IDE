use agent_settings::{LayoutBlock, LayoutBlueprint, LayoutPreset};
use component::{Component, ComponentScope, example_group_with_title, single_example};
use gpui::{AnyElement, App, Window};
use settings::DockSide;
use ui::prelude::*;

use crate::layout_studio_chrome::{LayoutStudioSnapshot, render_layout_studio_snapshot};
use crate::layout_studio_editor::LayoutStudioEditor;

#[derive(RegisterComponent)]
pub struct LayoutStudioPreview;

impl Component for LayoutStudioPreview {
    fn scope() -> ComponentScope {
        ComponentScope::Layout
    }

    fn name() -> &'static str {
        "Layout Studio"
    }

    fn sort_name() -> &'static str {
        "Layout Studio"
    }

    fn description() -> &'static str {
        "Arrange Workspace modal — interactive dollhouse editor and static preset snapshots."
    }

    fn preview(window: &mut Window, cx: &mut App) -> AnyElement {
        let collision_blueprint = {
            let mut blueprint = LayoutBlueprint::plan();
            blueprint.agent = agent_settings::AgentHost::Column {
                side: DockSide::Left,
                width: LayoutBlueprint::DEFAULT_AGENT_WIDTH,
            };
            blueprint.preset = LayoutPreset::Custom;
            blueprint
        };

        let interactive = cx.new(|cx| LayoutStudioEditor::new(LayoutBlueprint::plan(), cx));

        let examples = vec![
            single_example(
                "Interactive editor",
                div()
                    .w(px(640.))
                    .child(interactive)
                    .into_any_element(),
            ),
            single_example(
                "Plan preset",
                render_layout_studio_snapshot(
                    &LayoutStudioSnapshot::new(LayoutBlueprint::plan()),
                    window,
                    cx,
                )
                .into_any_element(),
            ),
            single_example(
                "Implement preset",
                render_layout_studio_snapshot(
                    &LayoutStudioSnapshot::new(LayoutBlueprint::implement()),
                    window,
                    cx,
                )
                .into_any_element(),
            ),
            single_example(
                "Dual display",
                render_layout_studio_snapshot(
                    &LayoutStudioSnapshot::new(LayoutBlueprint::dual()),
                    window,
                    cx,
                )
                .into_any_element(),
            ),
            single_example(
                "Collision error",
                render_layout_studio_snapshot(
                    &LayoutStudioSnapshot::new(collision_blueprint)
                        .with_selected_block(LayoutBlock::Agent)
                        .with_error("Plan and Agent can't share the same side"),
                    window,
                    cx,
                )
                .into_any_element(),
            ),
            single_example(
                "Advanced panel position",
                render_layout_studio_snapshot(
                    &LayoutStudioSnapshot::new(LayoutBlueprint::implement()).with_advanced(),
                    window,
                    cx,
                )
                .into_any_element(),
            ),
        ];

        v_flex()
            .gap_6()
            .p_4()
            .child(example_group_with_title("Layout Studio presets", examples))
            .into_any_element()
    }
}
