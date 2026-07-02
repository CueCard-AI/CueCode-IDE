use gpui::{Pixels, px};
use settings::{
    DockPosition, DockSide, LayoutOuterStripOrderSetting, LayoutPlanHostSetting, SettingsContent,
    SidebarDockPosition,
};

use crate::PanelLayout;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutBlock {
    Threads,
    Plan,
    Editor,
    Agent,
    Nav,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayTarget {
    Main,
    Secondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutPreset {
    Plan,
    Implement,
    Classic,
    Agentic,
    Dual,
    Custom,
}

impl LayoutPreset {
    pub fn label(self) -> &'static str {
        match self {
            Self::Plan => "Plan",
            Self::Implement => "Implement",
            Self::Classic => "Classic",
            Self::Agentic => "Agentic",
            Self::Dual => "Dual",
            Self::Custom => "Custom",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Plan => "Backlog + editor + agent on one screen",
            Self::Implement => "Wide editor and agent; plan as strip",
            Self::Classic => "Nav left, agent right",
            Self::Agentic => "Agent left, nav right",
            Self::Dual => "Plan on second window",
            Self::Custom => "Your arrangement",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadsPlacement {
    Left,
    Right,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavPlacement {
    Left,
    Right,
    Hidden,
}

/// Order of Threads vs Nav within the same outer strip (screen edge → editor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OuterStripOrder {
    #[default]
    ThreadsFirst,
    NavFirst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OuterEdgePosition {
    Edge,
    Inner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutGutter {
    FarLeft,
    BetweenOuterLeft,
    LeftColumn,
    BeforeEditor,
    AfterEditor,
    RightColumn,
    BetweenOuterRight,
    FarRight,
    Bottom,
    Detached,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlanHost {
    TabInAgent,
    Column { side: DockSide, width: Pixels },
    Detached { display: DisplayTarget },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentHost {
    Column { side: DockSide, width: Pixels },
    Bottom { height: Pixels },
    Detached { display: DisplayTarget },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutBlueprint {
    pub preset: LayoutPreset,
    pub threads: ThreadsPlacement,
    pub plan: PlanHost,
    pub agent: AgentHost,
    pub nav: NavPlacement,
    pub outer_left_order: OuterStripOrder,
    pub outer_right_order: OuterStripOrder,
    pub multiple_displays: bool,
    /// 0.0 = plan-heavy, 0.5 = balanced, 1.0 = agent-heavy
    pub focus_slider: f32,
    pub plan_strip_only: bool,
    /// Sidebar side to restore when threads is hidden (persisted via `sidebar_side`).
    pub threads_resume_side: DockSide,
    /// Nav dock side to restore when nav is hidden.
    pub nav_resume_side: DockSide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutValidationError {
    PlanAgentSameColumn,
    UnsupportedSlot,
}

impl LayoutValidationError {
    pub fn invalid_message(self) -> &'static str {
        match self {
            Self::PlanAgentSameColumn => "Plan and Agent can't share the same side",
            Self::UnsupportedSlot => "Not available for this panel",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlacementOutcome {
    Changed,
    AlreadyThere(&'static str),
    Invalid(LayoutValidationError),
}

impl PlacementOutcome {
    pub fn is_changed(self) -> bool {
        matches!(self, Self::Changed)
    }

    pub fn invalid_reason(self) -> Option<&'static str> {
        match self {
            Self::Invalid(error) => Some(error.invalid_message()),
            _ => None,
        }
    }

    pub fn already_there_message(self) -> Option<&'static str> {
        match self {
            Self::AlreadyThere(message) => Some(message),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlacementStepDirection {
    Prev,
    Next,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlPanelSegment {
    Block(LayoutBlock),
    OffChip(LayoutBlock),
}

/// Where the threads sidebar is rendered relative to workspace docks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreadsSidebarSlot {
    /// Outside the workspace tile (screen edge) — default ThreadsFirst pairing.
    Outer,
    /// Between the nav dock and the editor — NavFirst pairing.
    Inner,
    /// Not shown on this side (hidden or threads on the other side).
    Absent,
}

/// Where the agent panel is rendered relative to workspace nav docks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgentPanelSlot {
    /// Agent not in a side column on this edge (bottom, detached, opposite side, etc.).
    Absent,
    /// Agent occupies the side dock alone (nav hidden or on the opposite side).
    Outer,
    /// Between the nav dock and the editor — Agent + Nav share this side.
    Inner,
}

/// A column in a split side row (screen edge → editor).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SideColumnKind {
    NavDock,
    AgentPanel,
    ThreadsSidebar,
}

impl LayoutBlueprint {
    pub const DEFAULT_PLAN_WIDTH: Pixels = px(320.);
    pub const DEFAULT_AGENT_WIDTH: Pixels = px(420.);
    pub const DEFAULT_AGENT_HEIGHT: Pixels = px(320.);
    pub const MIN_EDITOR_WIDTH: Pixels = px(400.);

    pub fn plan() -> Self {
        Self {
            preset: LayoutPreset::Plan,
            threads: ThreadsPlacement::Left,
            plan: PlanHost::Column {
                side: DockSide::Left,
                width: Self::DEFAULT_PLAN_WIDTH,
            },
            agent: AgentHost::Column {
                side: DockSide::Right,
                width: Self::DEFAULT_AGENT_WIDTH,
            },
            nav: NavPlacement::Left,
            outer_left_order: OuterStripOrder::ThreadsFirst,
            outer_right_order: OuterStripOrder::NavFirst,
            multiple_displays: false,
            focus_slider: 0.25,
            plan_strip_only: false,
            threads_resume_side: DockSide::Left,
            nav_resume_side: DockSide::Left,
        }
    }

    pub fn implement() -> Self {
        Self {
            preset: LayoutPreset::Implement,
            threads: ThreadsPlacement::Left,
            plan: PlanHost::TabInAgent,
            agent: AgentHost::Column {
                side: DockSide::Right,
                width: px(480.),
            },
            nav: NavPlacement::Hidden,
            outer_left_order: OuterStripOrder::ThreadsFirst,
            outer_right_order: OuterStripOrder::NavFirst,
            multiple_displays: false,
            focus_slider: 0.85,
            plan_strip_only: true,
            threads_resume_side: DockSide::Left,
            nav_resume_side: DockSide::Left,
        }
    }

    pub fn classic() -> Self {
        Self {
            preset: LayoutPreset::Classic,
            threads: ThreadsPlacement::Left,
            plan: PlanHost::TabInAgent,
            agent: AgentHost::Column {
                side: DockSide::Right,
                width: Self::DEFAULT_AGENT_WIDTH,
            },
            nav: NavPlacement::Left,
            outer_left_order: OuterStripOrder::ThreadsFirst,
            outer_right_order: OuterStripOrder::NavFirst,
            multiple_displays: false,
            focus_slider: 0.5,
            plan_strip_only: false,
            threads_resume_side: DockSide::Left,
            nav_resume_side: DockSide::Left,
        }
    }

    pub fn agentic() -> Self {
        Self {
            preset: LayoutPreset::Agentic,
            threads: ThreadsPlacement::Left,
            plan: PlanHost::TabInAgent,
            agent: AgentHost::Column {
                side: DockSide::Left,
                width: px(640.),
            },
            nav: NavPlacement::Right,
            outer_left_order: OuterStripOrder::ThreadsFirst,
            outer_right_order: OuterStripOrder::NavFirst,
            multiple_displays: false,
            focus_slider: 0.75,
            plan_strip_only: false,
            threads_resume_side: DockSide::Left,
            nav_resume_side: DockSide::Right,
        }
    }

    pub fn dual() -> Self {
        Self {
            preset: LayoutPreset::Dual,
            threads: ThreadsPlacement::Left,
            plan: PlanHost::Detached {
                display: DisplayTarget::Secondary,
            },
            agent: AgentHost::Column {
                side: DockSide::Right,
                width: Self::DEFAULT_AGENT_WIDTH,
            },
            nav: NavPlacement::Left,
            outer_left_order: OuterStripOrder::ThreadsFirst,
            outer_right_order: OuterStripOrder::NavFirst,
            multiple_displays: true,
            focus_slider: 0.5,
            plan_strip_only: false,
            threads_resume_side: DockSide::Left,
            nav_resume_side: DockSide::Left,
        }
    }

    fn outer_strip_order_from_setting(
        setting: LayoutOuterStripOrderSetting,
    ) -> OuterStripOrder {
        match setting {
            LayoutOuterStripOrderSetting::ThreadsFirst => OuterStripOrder::ThreadsFirst,
            LayoutOuterStripOrderSetting::NavFirst => OuterStripOrder::NavFirst,
        }
    }

    fn outer_strip_order_to_setting(order: OuterStripOrder) -> LayoutOuterStripOrderSetting {
        match order {
            OuterStripOrder::ThreadsFirst => LayoutOuterStripOrderSetting::ThreadsFirst,
            OuterStripOrder::NavFirst => LayoutOuterStripOrderSetting::NavFirst,
        }
    }

    fn plan_host_from_settings(agent: &settings::AgentSettingsContent) -> PlanHost {
        match agent
            .layout_plan_host
            .unwrap_or(LayoutPlanHostSetting::TabInAgent)
        {
            LayoutPlanHostSetting::TabInAgent => PlanHost::TabInAgent,
            LayoutPlanHostSetting::ColumnLeft => PlanHost::Column {
                side: DockSide::Left,
                width: agent
                    .layout_plan_column_width
                    .map(px)
                    .unwrap_or(Self::DEFAULT_PLAN_WIDTH),
            },
            LayoutPlanHostSetting::ColumnRight => PlanHost::Column {
                side: DockSide::Right,
                width: agent
                    .layout_plan_column_width
                    .map(px)
                    .unwrap_or(Self::DEFAULT_PLAN_WIDTH),
            },
            LayoutPlanHostSetting::Detached => PlanHost::Detached {
                display: DisplayTarget::Secondary,
            },
        }
    }

    pub fn from_merged_settings(content: &SettingsContent) -> Self {
        let panel = PanelLayout::from_merged_settings(content);
        let agent_settings = content.agent.as_ref();
        let agent_dock = panel.agent_dock.unwrap_or(DockPosition::Left);

        let threads_hidden = agent_settings
            .and_then(|agent| agent.layout_threads_hidden)
            .unwrap_or(false);
        let threads = if threads_hidden {
            ThreadsPlacement::Hidden
        } else {
            agent_settings
                .and_then(|agent| agent.sidebar_side)
                .map(|side| match side {
                    SidebarDockPosition::Left => ThreadsPlacement::Left,
                    SidebarDockPosition::Right => ThreadsPlacement::Right,
                })
                .unwrap_or(ThreadsPlacement::Left)
        };

        let threads_resume_side = agent_settings
            .and_then(|agent| agent.sidebar_side)
            .map(|side| match side {
                SidebarDockPosition::Left => DockSide::Left,
                SidebarDockPosition::Right => DockSide::Right,
            })
            .unwrap_or(DockSide::Left);

        let nav_hidden = agent_settings
            .and_then(|agent| agent.layout_nav_hidden)
            .unwrap_or(false);
        let nav_side = panel
            .project_panel_dock
            .or(panel.outline_panel_dock)
            .unwrap_or(DockSide::Right);
        let nav = if nav_hidden {
            NavPlacement::Hidden
        } else {
            match nav_side {
                DockSide::Left => NavPlacement::Left,
                DockSide::Right => NavPlacement::Right,
            }
        };
        let nav_resume_side = nav_side;

        let plan = agent_settings
            .map(Self::plan_host_from_settings)
            .unwrap_or(PlanHost::TabInAgent);

        let agent = match agent_dock {
            DockPosition::Bottom => AgentHost::Bottom {
                height: agent_settings
                    .and_then(|agent| agent.default_height)
                    .map(px)
                    .unwrap_or(Self::DEFAULT_AGENT_HEIGHT),
            },
            DockPosition::Left => AgentHost::Column {
                side: DockSide::Left,
                width: agent_settings
                    .and_then(|agent| agent.default_width)
                    .map(px)
                    .unwrap_or(px(640.)),
            },
            DockPosition::Right => AgentHost::Column {
                side: DockSide::Right,
                width: agent_settings
                    .and_then(|agent| agent.default_width)
                    .map(px)
                    .unwrap_or(Self::DEFAULT_AGENT_WIDTH),
            },
        };

        let outer_left_order = agent_settings
            .and_then(|agent| agent.layout_outer_left_order)
            .map(Self::outer_strip_order_from_setting)
            .unwrap_or(OuterStripOrder::ThreadsFirst);
        let outer_right_order = agent_settings
            .and_then(|agent| agent.layout_outer_right_order)
            .map(Self::outer_strip_order_from_setting)
            .unwrap_or(OuterStripOrder::NavFirst);
        let plan_strip_only = agent_settings
            .and_then(|agent| agent.layout_plan_strip_only)
            .unwrap_or(false);
        let multiple_displays = matches!(plan, PlanHost::Detached { .. })
            || matches!(agent, AgentHost::Detached { .. });

        let preset = LayoutPreset::Custom;

        Self {
            preset,
            threads,
            plan,
            agent,
            nav,
            outer_left_order,
            outer_right_order,
            multiple_displays,
            focus_slider: 0.5,
            plan_strip_only,
            threads_resume_side,
            nav_resume_side,
        }
    }

    pub fn outer_left_blocks(&self) -> Vec<LayoutBlock> {
        Self::outer_blocks(
            self.threads == ThreadsPlacement::Left,
            self.nav == NavPlacement::Left,
            self.outer_left_order,
        )
    }

    pub fn outer_right_blocks(&self) -> Vec<LayoutBlock> {
        Self::outer_blocks(
            self.threads == ThreadsPlacement::Right,
            self.nav == NavPlacement::Right,
            self.outer_right_order,
        )
    }

    fn outer_blocks(
        has_threads: bool,
        has_nav: bool,
        order: OuterStripOrder,
    ) -> Vec<LayoutBlock> {
        match (has_threads, has_nav) {
            (true, true) => match order {
                OuterStripOrder::ThreadsFirst => {
                    vec![LayoutBlock::Threads, LayoutBlock::Nav]
                }
                OuterStripOrder::NavFirst => vec![LayoutBlock::Nav, LayoutBlock::Threads],
            },
            (true, false) => vec![LayoutBlock::Threads],
            (false, true) => vec![LayoutBlock::Nav],
            (false, false) => Vec::new(),
        }
    }

    pub fn validate(&self) -> Result<(), LayoutValidationError> {
        if self.plan_column_side() == self.agent_column_side() {
            return Err(LayoutValidationError::PlanAgentSameColumn);
        }
        Ok(())
    }

    pub fn plan_column_side(&self) -> Option<DockSide> {
        match &self.plan {
            PlanHost::Column { side, .. } => Some(*side),
            _ => None,
        }
    }

    pub fn agent_column_side(&self) -> Option<DockSide> {
        match &self.agent {
            AgentHost::Column { side, .. } => Some(*side),
            _ => None,
        }
    }

    pub fn apply_focus_slider(&mut self) {
        let t = self.focus_slider.clamp(0., 1.);
        self.focus_slider = t;

        let plan_width = px(280. + (1. - t) * 120.);
        let agent_width = px(340. + t * 220.);

        if let PlanHost::Column { width, .. } = &mut self.plan {
            *width = plan_width;
        }
        if let AgentHost::Column { width, .. } = &mut self.agent {
            *width = agent_width;
        }
    }

    pub fn to_panel_layout(&self) -> PanelLayout {
        let agent_dock = match &self.agent {
            AgentHost::Column { side, .. } => match side {
                DockSide::Left => DockPosition::Left,
                DockSide::Right => DockPosition::Right,
            },
            AgentHost::Bottom { .. } => DockPosition::Bottom,
            AgentHost::Detached { .. } => DockPosition::Right,
        };

        let nav_dock = match self.nav {
            NavPlacement::Left => DockSide::Left,
            NavPlacement::Right => DockSide::Right,
            NavPlacement::Hidden => {
                if agent_dock == DockPosition::Right {
                    DockSide::Left
                } else {
                    DockSide::Right
                }
            }
        };

        PanelLayout {
            agent_dock: Some(agent_dock),
            project_panel_dock: Some(nav_dock),
            outline_panel_dock: Some(nav_dock),
            collaboration_panel_dock: Some(match nav_dock {
                DockSide::Left => DockPosition::Left,
                DockSide::Right => DockPosition::Right,
            }),
            git_panel_dock: Some(match nav_dock {
                DockSide::Left => DockPosition::Left,
                DockSide::Right => DockPosition::Right,
            }),
        }
    }

    pub fn write_settings(&self, merged: &PanelLayout, settings: &mut SettingsContent) {
        let panel = self.to_panel_layout();
        panel.write_diff_to(merged, settings);

        if let Some(agent_dock) = panel.agent_dock {
            if !self.agent_and_nav_share_side() {
                PanelLayout::reconcile_side_panels_for_agent_dock(agent_dock, merged, settings);
            }
        }

        let agent = settings.agent.get_or_insert_default();
        match &self.agent {
            AgentHost::Column { width, .. } => {
                agent.default_width = Some(f32::from(*width));
            }
            AgentHost::Bottom { height } => {
                agent.default_height = Some(f32::from(*height));
            }
            AgentHost::Detached { .. } => {}
        }

        agent.layout_plan_host = Some(match &self.plan {
            PlanHost::TabInAgent => LayoutPlanHostSetting::TabInAgent,
            PlanHost::Column {
                side: DockSide::Left,
                ..
            } => LayoutPlanHostSetting::ColumnLeft,
            PlanHost::Column {
                side: DockSide::Right,
                ..
            } => LayoutPlanHostSetting::ColumnRight,
            PlanHost::Detached { .. } => LayoutPlanHostSetting::Detached,
        });
        if let PlanHost::Column { width, .. } = &self.plan {
            agent.layout_plan_column_width = Some(f32::from(*width));
        }

        agent.layout_outer_left_order =
            Some(Self::outer_strip_order_to_setting(self.outer_left_order));
        agent.layout_outer_right_order =
            Some(Self::outer_strip_order_to_setting(self.outer_right_order));
        agent.layout_threads_hidden = Some(self.threads == ThreadsPlacement::Hidden);
        agent.layout_nav_hidden = Some(self.nav == NavPlacement::Hidden);
        agent.layout_plan_strip_only = Some(self.plan_strip_only);

        agent.sidebar_side = Some(match self.threads {
            ThreadsPlacement::Left => SidebarDockPosition::Left,
            ThreadsPlacement::Right => SidebarDockPosition::Right,
            ThreadsPlacement::Hidden => match self.threads_resume_side {
                DockSide::Left => SidebarDockPosition::Left,
                DockSide::Right => SidebarDockPosition::Right,
            },
        });
    }

    pub fn plan_detached(&self) -> bool {
        matches!(
            self.plan,
            PlanHost::Detached {
                display: DisplayTarget::Secondary
            }
        )
    }

    pub fn placement_equivalent(&self, other: &Self) -> bool {
        self.threads == other.threads
            && self.nav == other.nav
            && Self::plan_host_equivalent(&self.plan, &other.plan)
            && Self::agent_host_equivalent(&self.agent, &other.agent)
            && self.outer_left_order == other.outer_left_order
            && self.outer_right_order == other.outer_right_order
            && self.multiple_displays == other.multiple_displays
            && self.plan_strip_only == other.plan_strip_only
    }

    fn plan_host_equivalent(left: &PlanHost, right: &PlanHost) -> bool {
        match (left, right) {
            (PlanHost::TabInAgent, PlanHost::TabInAgent) => true,
            (
                PlanHost::Column { side: left, .. },
                PlanHost::Column { side: right, .. },
            ) => left == right,
            (
                PlanHost::Detached { display: left, .. },
                PlanHost::Detached { display: right, .. },
            ) => left == right,
            _ => false,
        }
    }

    fn agent_host_equivalent(left: &AgentHost, right: &AgentHost) -> bool {
        match (left, right) {
            (
                AgentHost::Column { side: left, .. },
                AgentHost::Column { side: right, .. },
            ) => left == right,
            (AgentHost::Bottom { .. }, AgentHost::Bottom { .. }) => true,
            (
                AgentHost::Detached { display: left, .. },
                AgentHost::Detached { display: right, .. },
            ) => left == right,
            _ => false,
        }
    }

    pub fn agent_detached(&self) -> bool {
        matches!(
            self.agent,
            AgentHost::Detached {
                display: DisplayTarget::Secondary
            }
        )
    }

    pub fn resolve_drop_slot(block: LayoutBlock, target: LayoutSlot) -> LayoutSlot {
        match block {
            LayoutBlock::Nav | LayoutBlock::Threads => match target {
                LayoutSlot::LeftInner | LayoutSlot::LeftOuter => LayoutSlot::LeftOuter,
                LayoutSlot::RightInner | LayoutSlot::RightOuter => LayoutSlot::RightOuter,
                other => other,
            },
            _ => target,
        }
    }

    pub fn preview_place(
        blueprint: &Self,
        block: LayoutBlock,
        target: LayoutSlot,
    ) -> Result<(), LayoutValidationError> {
        Self::with_block_placed(blueprint, block, target).map(|_| ())
    }

    pub fn with_block_placed(
        blueprint: &Self,
        block: LayoutBlock,
        target: LayoutSlot,
    ) -> Result<Self, LayoutValidationError> {
        let slot = Self::resolve_drop_slot(block, target);
        let mut next = blueprint.clone();
        next.try_place_block(block, slot)?;
        Ok(next)
    }

    pub fn with_block_at_gutter(
        blueprint: &Self,
        block: LayoutBlock,
        gutter: LayoutGutter,
    ) -> Result<Self, LayoutValidationError> {
        let mut next = blueprint.clone();
        next.try_place_block_at_gutter(block, gutter)?;
        Ok(next)
    }

    pub fn try_place_block_at_gutter(
        &mut self,
        block: LayoutBlock,
        gutter: LayoutGutter,
    ) -> Result<(), LayoutValidationError> {
        match gutter {
            LayoutGutter::FarLeft => {
                self.try_place_outer(block, DockSide::Left, OuterEdgePosition::Edge)
            }
            LayoutGutter::BetweenOuterLeft => {
                self.try_place_outer(block, DockSide::Left, OuterEdgePosition::Inner)
            }
            LayoutGutter::LeftColumn | LayoutGutter::BeforeEditor => {
                let slot = Self::resolve_drop_slot(block, LayoutSlot::LeftInner);
                self.try_place_block(block, slot)
            }
            LayoutGutter::AfterEditor | LayoutGutter::RightColumn => {
                let slot = Self::resolve_drop_slot(block, LayoutSlot::RightInner);
                self.try_place_block(block, slot)
            }
            LayoutGutter::BetweenOuterRight => {
                self.try_place_outer(block, DockSide::Right, OuterEdgePosition::Inner)
            }
            LayoutGutter::FarRight => {
                self.try_place_outer(block, DockSide::Right, OuterEdgePosition::Edge)
            }
            LayoutGutter::Bottom => self.try_place_block(block, LayoutSlot::Bottom),
            LayoutGutter::Detached => self.try_place_block(block, LayoutSlot::Detached),
        }
    }

    pub fn block_status_short(self: &Self, block: LayoutBlock) -> &'static str {
        match block {
            LayoutBlock::Threads => match self.threads {
                ThreadsPlacement::Hidden => "Hidden",
                ThreadsPlacement::Left => {
                    if self.outer_blocks_share_side(LayoutBlock::Threads) {
                        if self.outer_block_is_at_edge(LayoutBlock::Threads, DockSide::Left) {
                            "Edge L"
                        } else {
                            "Inside L"
                        }
                    } else {
                        "Left"
                    }
                }
                ThreadsPlacement::Right => {
                    if self.outer_blocks_share_side(LayoutBlock::Threads) {
                        if self.outer_block_is_at_edge(LayoutBlock::Threads, DockSide::Right) {
                            "Edge R"
                        } else {
                            "Inside R"
                        }
                    } else {
                        "Right"
                    }
                }
            },
            LayoutBlock::Nav => match self.nav {
                NavPlacement::Hidden => "Hidden",
                NavPlacement::Left => {
                    if self.outer_blocks_share_side(LayoutBlock::Nav) {
                        if self.outer_block_is_at_edge(LayoutBlock::Nav, DockSide::Left) {
                            "Edge L"
                        } else {
                            "Inside L"
                        }
                    } else {
                        "Left"
                    }
                }
                NavPlacement::Right => {
                    if self.outer_blocks_share_side(LayoutBlock::Nav) {
                        if self.outer_block_is_at_edge(LayoutBlock::Nav, DockSide::Right) {
                            "Edge R"
                        } else {
                            "Inside R"
                        }
                    } else {
                        "Right"
                    }
                }
            },
            LayoutBlock::Plan => match &self.plan {
                PlanHost::TabInAgent => "In tab",
                PlanHost::Column { side: DockSide::Left, .. } => "Left col",
                PlanHost::Column { side: DockSide::Right, .. } => "Right col",
                PlanHost::Detached { .. } => "2nd win",
            },
            LayoutBlock::Agent => match &self.agent {
                AgentHost::Column { side: DockSide::Left, .. } => "Left col",
                AgentHost::Column { side: DockSide::Right, .. } => "Right col",
                AgentHost::Bottom { .. } => "Bottom",
                AgentHost::Detached { .. } => "2nd win",
            },
            LayoutBlock::Editor => "Center",
        }
    }

    pub fn block_status_label(self: &Self, block: LayoutBlock) -> &'static str {
        match block {
            LayoutBlock::Threads => match self.threads {
                ThreadsPlacement::Left => "Left edge",
                ThreadsPlacement::Right => "Right edge",
                ThreadsPlacement::Hidden => "Hidden",
            },
            LayoutBlock::Nav => match self.nav {
                NavPlacement::Left => "Left edge",
                NavPlacement::Right => "Right edge",
                NavPlacement::Hidden => "Hidden",
            },
            LayoutBlock::Plan => match &self.plan {
                PlanHost::TabInAgent => "In Agent tab",
                PlanHost::Column { side: DockSide::Left, .. } => "Left column",
                PlanHost::Column { side: DockSide::Right, .. } => "Right column",
                PlanHost::Detached { .. } => "Second window",
            },
            LayoutBlock::Agent => match &self.agent {
                AgentHost::Column { side: DockSide::Left, .. } => "Left column",
                AgentHost::Column { side: DockSide::Right, .. } => "Right column",
                AgentHost::Bottom { .. } => "Bottom band",
                AgentHost::Detached { .. } => "Second window",
            },
            LayoutBlock::Editor => "Center",
        }
    }

    pub fn block_is_ghost(self: &Self, block: LayoutBlock) -> bool {
        match block {
            LayoutBlock::Threads => self.threads == ThreadsPlacement::Hidden,
            LayoutBlock::Nav => self.nav == NavPlacement::Hidden,
            LayoutBlock::Plan => {
                matches!(self.plan, PlanHost::TabInAgent) || self.plan_detached()
            }
            LayoutBlock::Agent => {
                matches!(
                    self.agent,
                    AgentHost::Bottom { .. } | AgentHost::Detached { .. }
                )
            }
            LayoutBlock::Editor => false,
        }
    }

    pub fn drop_failure_message(block: LayoutBlock, gutter: LayoutGutter) -> String {
        match (block, gutter) {
            (LayoutBlock::Threads | LayoutBlock::Nav, LayoutGutter::Bottom) => format!(
                "{} can't go in the bottom band. Use Left edge or Right edge.",
                block_label(block)
            ),
            (LayoutBlock::Threads | LayoutBlock::Nav, LayoutGutter::Detached) => format!(
                "{} stays in the main window. Use Left edge or Right edge.",
                block_label(block)
            ),
            (LayoutBlock::Plan, LayoutGutter::Bottom) => {
                "Plan can't go in the bottom band. Use a column or second window.".into()
            }
            (LayoutBlock::Threads | LayoutBlock::Nav, _) => format!(
                "{} only goes on the left or right edge. Pick Left edge or Right edge below.",
                block_label(block)
            ),
            _ => format!(
                "{} can't be placed at {}",
                block_label(block),
                gutter.drop_hint(block)
            ),
        }
    }

    fn try_place_outer(
        &mut self,
        block: LayoutBlock,
        side: DockSide,
        position: OuterEdgePosition,
    ) -> Result<(), LayoutValidationError> {
        match block {
            LayoutBlock::Threads => {
                self.threads = match side {
                    DockSide::Left => ThreadsPlacement::Left,
                    DockSide::Right => ThreadsPlacement::Right,
                };
                self.threads_resume_side = side;
            }
            LayoutBlock::Nav => {
                self.nav = match side {
                    DockSide::Left => NavPlacement::Left,
                    DockSide::Right => NavPlacement::Right,
                };
                self.nav_resume_side = side;
            }
            _ => return Err(LayoutValidationError::UnsupportedSlot),
        }

        let other_on_same_side = match (block, side) {
            (LayoutBlock::Threads, DockSide::Left) => self.nav == NavPlacement::Left,
            (LayoutBlock::Nav, DockSide::Left) => self.threads == ThreadsPlacement::Left,
            (LayoutBlock::Threads, DockSide::Right) => self.nav == NavPlacement::Right,
            (LayoutBlock::Nav, DockSide::Right) => self.threads == ThreadsPlacement::Right,
            _ => false,
        };

        if other_on_same_side {
            match (side, position, block) {
                (DockSide::Left, OuterEdgePosition::Edge, LayoutBlock::Threads) => {
                    self.outer_left_order = OuterStripOrder::ThreadsFirst;
                }
                (DockSide::Left, OuterEdgePosition::Edge, LayoutBlock::Nav) => {
                    self.outer_left_order = OuterStripOrder::NavFirst;
                }
                (DockSide::Left, OuterEdgePosition::Inner, LayoutBlock::Nav) => {
                    self.outer_left_order = OuterStripOrder::ThreadsFirst;
                }
                (DockSide::Left, OuterEdgePosition::Inner, LayoutBlock::Threads) => {
                    self.outer_left_order = OuterStripOrder::NavFirst;
                }
                (DockSide::Right, OuterEdgePosition::Edge, LayoutBlock::Threads) => {
                    self.outer_right_order = OuterStripOrder::ThreadsFirst;
                }
                (DockSide::Right, OuterEdgePosition::Edge, LayoutBlock::Nav) => {
                    self.outer_right_order = OuterStripOrder::NavFirst;
                }
                (DockSide::Right, OuterEdgePosition::Inner, LayoutBlock::Nav) => {
                    self.outer_right_order = OuterStripOrder::ThreadsFirst;
                }
                (DockSide::Right, OuterEdgePosition::Inner, LayoutBlock::Threads) => {
                    self.outer_right_order = OuterStripOrder::NavFirst;
                }
                _ => {}
            }
        } else {
            match (block, side) {
                (LayoutBlock::Threads, DockSide::Left) => {
                    self.outer_left_order = OuterStripOrder::ThreadsFirst;
                }
                (LayoutBlock::Nav, DockSide::Left) => {
                    self.outer_left_order = OuterStripOrder::NavFirst;
                }
                (LayoutBlock::Threads, DockSide::Right) => {
                    self.outer_right_order = OuterStripOrder::ThreadsFirst;
                }
                (LayoutBlock::Nav, DockSide::Right) => {
                    self.outer_right_order = OuterStripOrder::NavFirst;
                }
                _ => {}
            }
        }

        self.preset = LayoutPreset::Custom;
        self.apply_focus_slider();
        self.validate()?;
        Ok(())
    }

    pub fn try_place_block(
        &mut self,
        block: LayoutBlock,
        slot: LayoutSlot,
    ) -> Result<(), LayoutValidationError> {
        match block {
            LayoutBlock::Threads => match slot {
                LayoutSlot::LeftOuter => {
                    return self.try_place_outer(
                        LayoutBlock::Threads,
                        DockSide::Left,
                        OuterEdgePosition::Edge,
                    );
                }
                LayoutSlot::RightOuter => {
                    return self.try_place_outer(
                        LayoutBlock::Threads,
                        DockSide::Right,
                        OuterEdgePosition::Edge,
                    );
                }
                _ => return Err(LayoutValidationError::UnsupportedSlot),
            },
            LayoutBlock::Plan => match slot {
                LayoutSlot::LeftInner => {
                    self.plan = PlanHost::Column {
                        side: DockSide::Left,
                        width: Self::DEFAULT_PLAN_WIDTH,
                    };
                }
                LayoutSlot::RightInner => {
                    self.plan = PlanHost::Column {
                        side: DockSide::Right,
                        width: Self::DEFAULT_PLAN_WIDTH,
                    };
                }
                LayoutSlot::Detached => {
                    self.plan = PlanHost::Detached {
                        display: DisplayTarget::Secondary,
                    };
                    self.multiple_displays = true;
                }
                _ => return Err(LayoutValidationError::UnsupportedSlot),
            },
            LayoutBlock::Agent => match slot {
                LayoutSlot::LeftInner => {
                    self.agent = AgentHost::Column {
                        side: DockSide::Left,
                        width: Self::DEFAULT_AGENT_WIDTH,
                    };
                }
                LayoutSlot::RightInner => {
                    self.agent = AgentHost::Column {
                        side: DockSide::Right,
                        width: Self::DEFAULT_AGENT_WIDTH,
                    };
                }
                LayoutSlot::Bottom => {
                    self.agent = AgentHost::Bottom {
                        height: Self::DEFAULT_AGENT_HEIGHT,
                    };
                }
                LayoutSlot::Detached => {
                    self.agent = AgentHost::Detached {
                        display: DisplayTarget::Secondary,
                    };
                    self.multiple_displays = true;
                }
                _ => return Err(LayoutValidationError::UnsupportedSlot),
            },
            LayoutBlock::Nav => match slot {
                LayoutSlot::LeftOuter => {
                    return self.try_place_outer(
                        LayoutBlock::Nav,
                        DockSide::Left,
                        OuterEdgePosition::Edge,
                    );
                }
                LayoutSlot::RightOuter => {
                    return self.try_place_outer(
                        LayoutBlock::Nav,
                        DockSide::Right,
                        OuterEdgePosition::Edge,
                    );
                }
                _ => return Err(LayoutValidationError::UnsupportedSlot),
            },
            LayoutBlock::Editor => return Err(LayoutValidationError::UnsupportedSlot),
        }

        self.preset = LayoutPreset::Custom;
        self.apply_focus_slider();
        self.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutSlot {
    LeftOuter,
    LeftInner,
    RightInner,
    RightOuter,
    Bottom,
    Detached,
}

impl LayoutSlot {
    pub fn label(self) -> &'static str {
        match self {
            Self::LeftOuter => "Left edge",
            Self::LeftInner => "Left column",
            Self::RightInner => "Right column",
            Self::RightOuter => "Right edge",
            Self::Bottom => "Bottom band",
            Self::Detached => "Second window",
        }
    }
}

impl LayoutGutter {
    pub fn dom_id(self) -> &'static str {
        match self {
            Self::FarLeft => "layout-gutter-far-left",
            Self::BetweenOuterLeft => "layout-gutter-between-outer-left",
            Self::LeftColumn => "layout-gutter-left-column",
            Self::BeforeEditor => "layout-gutter-before-editor",
            Self::AfterEditor => "layout-gutter-after-editor",
            Self::RightColumn => "layout-gutter-right-column",
            Self::BetweenOuterRight => "layout-gutter-between-outer-right",
            Self::FarRight => "layout-gutter-far-right",
            Self::Bottom => "layout-gutter-bottom",
            Self::Detached => "layout-gutter-detached",
        }
    }

    pub fn drop_hint(self, block: LayoutBlock) -> String {
        match self {
            Self::FarLeft => match block {
                LayoutBlock::Nav | LayoutBlock::Threads => "Left edge".into(),
                _ => "Left edge (Nav or Threads only)".into(),
            },
            Self::BetweenOuterLeft => match block {
                LayoutBlock::Nav => "Nav inside".into(),
                LayoutBlock::Threads => "Threads inside".into(),
                _ => "Left column".into(),
            },
            Self::LeftColumn => LayoutSlot::LeftInner.label().into(),
            Self::BeforeEditor => match block {
                LayoutBlock::Nav | LayoutBlock::Threads => "→ Left edge".into(),
                _ => "Left column".into(),
            },
            Self::AfterEditor => match block {
                LayoutBlock::Nav | LayoutBlock::Threads => "→ Right edge".into(),
                _ => "Right column".into(),
            },
            Self::RightColumn => LayoutSlot::RightInner.label().into(),
            Self::BetweenOuterRight => match block {
                LayoutBlock::Nav => "Nav inside".into(),
                LayoutBlock::Threads => "Threads inside".into(),
                _ => "Right column".into(),
            },
            Self::FarRight => match block {
                LayoutBlock::Nav | LayoutBlock::Threads => "Right edge".into(),
                _ => "Right edge (Nav or Threads only)".into(),
            },
            Self::Bottom => LayoutSlot::Bottom.label().into(),
            Self::Detached => LayoutSlot::Detached.label().into(),
        }
    }

    pub fn from_layout_slot(slot: LayoutSlot) -> Self {
        match slot {
            LayoutSlot::LeftOuter => Self::FarLeft,
            LayoutSlot::LeftInner => Self::LeftColumn,
            LayoutSlot::RightInner => Self::AfterEditor,
            LayoutSlot::RightOuter => Self::FarRight,
            LayoutSlot::Bottom => Self::Bottom,
            LayoutSlot::Detached => Self::Detached,
        }
    }

    pub fn valid_for_block(self, block: LayoutBlock) -> bool {
        match block {
            LayoutBlock::Editor => false,
            LayoutBlock::Threads | LayoutBlock::Nav => !matches!(self, Self::Bottom | Self::Detached),
            LayoutBlock::Plan => !matches!(self, Self::Bottom),
            LayoutBlock::Agent => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockPlacementAction {
    Gutter(LayoutGutter),
    HideThreads,
    HideNav,
    PlanTabInAgent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockPlacementOption {
    pub label: &'static str,
    pub action: BlockPlacementAction,
}

impl LayoutBlueprint {
    pub fn block_description(block: LayoutBlock) -> &'static str {
        match block {
            LayoutBlock::Threads => "Chat history and thread list sidebar",
            LayoutBlock::Nav => "Project, outline, and git panels",
            LayoutBlock::Plan => "Backlog and planning hub column",
            LayoutBlock::Editor => "Code editor — always stays in the center",
            LayoutBlock::Agent => "AI chat, tools, and composer",
        }
    }

    fn outer_block_is_innermost(&self, block: LayoutBlock, side: DockSide) -> bool {
        let blocks = match side {
            DockSide::Left => self.outer_left_blocks(),
            DockSide::Right => self.outer_right_blocks(),
        };
        blocks.last() == Some(&block)
    }

    fn outer_block_is_at_edge(&self, block: LayoutBlock, side: DockSide) -> bool {
        let blocks = match side {
            DockSide::Left => self.outer_left_blocks(),
            DockSide::Right => self.outer_right_blocks(),
        };
        blocks.first() == Some(&block)
    }

    fn outer_blocks_share_side(&self, block: LayoutBlock) -> bool {
        match block {
            LayoutBlock::Threads => self.nav == NavPlacement::Left && self.threads == ThreadsPlacement::Left
                || self.nav == NavPlacement::Right && self.threads == ThreadsPlacement::Right,
            LayoutBlock::Nav => self.threads == ThreadsPlacement::Left && self.nav == NavPlacement::Left
                || self.threads == ThreadsPlacement::Right && self.nav == NavPlacement::Right,
            _ => false,
        }
    }

    pub fn check_placement(
        blueprint: &Self,
        block: LayoutBlock,
        action: BlockPlacementAction,
    ) -> Result<(), LayoutValidationError> {
        match Self::preview_placement(blueprint, block, action) {
            PlacementOutcome::Changed | PlacementOutcome::AlreadyThere(_) => Ok(()),
            PlacementOutcome::Invalid(error) => Err(error),
        }
    }

    pub fn preview_placement(
        blueprint: &Self,
        block: LayoutBlock,
        action: BlockPlacementAction,
    ) -> PlacementOutcome {
        let mut next = blueprint.clone();
        match next.try_apply_placement(block, action) {
            Ok(()) if next.placement_equivalent(blueprint) => {
                PlacementOutcome::AlreadyThere(Self::placement_already_there_message(
                    block, action,
                ))
            }
            Ok(()) => PlacementOutcome::Changed,
            Err(error) => PlacementOutcome::Invalid(error),
        }
    }

    pub fn apply_placement(
        &mut self,
        block: LayoutBlock,
        action: BlockPlacementAction,
    ) -> PlacementOutcome {
        let before = self.clone();
        match self.try_apply_placement(block, action) {
            Ok(()) if self.placement_equivalent(&before) => PlacementOutcome::AlreadyThere(
                Self::placement_already_there_message(block, action),
            ),
            Ok(()) => PlacementOutcome::Changed,
            Err(error) => PlacementOutcome::Invalid(error),
        }
    }

    fn placement_already_there_message(
        block: LayoutBlock,
        action: BlockPlacementAction,
    ) -> &'static str {
        match (block, action) {
            (LayoutBlock::Threads, BlockPlacementAction::HideThreads) => {
                "Threads is already hidden"
            }
            (LayoutBlock::Nav, BlockPlacementAction::HideNav) => "Nav is already hidden",
            (LayoutBlock::Plan, BlockPlacementAction::PlanTabInAgent) => {
                "Plan is already in the Agent tab"
            }
            (_, BlockPlacementAction::HideThreads) => "Threads is already hidden",
            (_, BlockPlacementAction::HideNav) => "Nav is already hidden",
            (_, BlockPlacementAction::PlanTabInAgent) => "Plan is already in the Agent tab",
            (block, BlockPlacementAction::Gutter(gutter)) => {
                Self::placement_gutter_already_there_message(block, gutter)
            }
        }
    }

    fn placement_gutter_already_there_message(
        block: LayoutBlock,
        gutter: LayoutGutter,
    ) -> &'static str {
        let label = block_label(block);
        match gutter {
            LayoutGutter::FarLeft | LayoutGutter::BeforeEditor | LayoutGutter::LeftColumn => {
                match block {
                    LayoutBlock::Threads | LayoutBlock::Nav => {
                        return match label {
                            "Threads" => "Threads is already on the left edge",
                            _ => "Nav is already on the left edge",
                        };
                    }
                    LayoutBlock::Plan => return "Plan is already in the left column",
                    LayoutBlock::Agent => return "Agent is already in the left column",
                    LayoutBlock::Editor => return "Editor stays in the center",
                }
            }
            LayoutGutter::BetweenOuterLeft => match block {
                LayoutBlock::Threads => "Threads is already inside on the left",
                LayoutBlock::Nav => "Nav is already inside on the left",
                _ => "Already in this position",
            },
            LayoutGutter::AfterEditor
            | LayoutGutter::RightColumn
            | LayoutGutter::FarRight => match block {
                LayoutBlock::Threads => "Threads is already on the right edge",
                LayoutBlock::Nav => "Nav is already on the right edge",
                LayoutBlock::Plan => "Plan is already in the right column",
                LayoutBlock::Agent => "Agent is already in the right column",
                LayoutBlock::Editor => "Editor stays in the center",
            },
            LayoutGutter::BetweenOuterRight => match block {
                LayoutBlock::Threads => "Threads is already inside on the right",
                LayoutBlock::Nav => "Nav is already inside on the right",
                _ => "Already in this position",
            },
            LayoutGutter::Bottom => match block {
                LayoutBlock::Agent => "Agent is already in the bottom band",
                _ => "Already in this position",
            },
            LayoutGutter::Detached => match block {
                LayoutBlock::Plan => "Plan is already in the second window",
                LayoutBlock::Agent => "Agent is already in the second window",
                _ => "Already in this position",
            },
        }
    }

    pub fn placement_disabled_reason(
        blueprint: &Self,
        block: LayoutBlock,
        action: BlockPlacementAction,
    ) -> Option<&'static str> {
        match Self::preview_placement(blueprint, block, action) {
            PlacementOutcome::Invalid(error) => Some(error.invalid_message()),
            PlacementOutcome::AlreadyThere(_) | PlacementOutcome::Changed => None,
        }
    }

    pub fn placement_options_for(&self, block: LayoutBlock) -> Vec<BlockPlacementOption> {
        match block {
            LayoutBlock::Threads => {
                let mut options = vec![BlockPlacementOption {
                    label: "Left edge",
                    action: BlockPlacementAction::Gutter(LayoutGutter::FarLeft),
                }];
                if self.threads == ThreadsPlacement::Left && self.nav == NavPlacement::Left {
                    options.push(BlockPlacementOption {
                        label: "Inside left",
                        action: BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterLeft),
                    });
                }
                options.push(BlockPlacementOption {
                    label: "Right edge",
                    action: BlockPlacementAction::Gutter(LayoutGutter::FarRight),
                });
                if self.threads == ThreadsPlacement::Right && self.nav == NavPlacement::Right {
                    options.insert(
                        2,
                        BlockPlacementOption {
                            label: "Inside right",
                            action: BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterRight),
                        },
                    );
                }
                options.push(BlockPlacementOption {
                    label: "Hidden",
                    action: BlockPlacementAction::HideThreads,
                });
                options
            }
            LayoutBlock::Nav => {
                let mut options = vec![BlockPlacementOption {
                    label: "Left edge",
                    action: BlockPlacementAction::Gutter(LayoutGutter::FarLeft),
                }];
                if self.nav == NavPlacement::Left && self.threads == ThreadsPlacement::Left {
                    options.push(BlockPlacementOption {
                        label: "Inside left",
                        action: BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterLeft),
                    });
                }
                options.push(BlockPlacementOption {
                    label: "Right edge",
                    action: BlockPlacementAction::Gutter(LayoutGutter::FarRight),
                });
                if self.nav == NavPlacement::Right && self.threads == ThreadsPlacement::Right {
                    options.insert(
                        2,
                        BlockPlacementOption {
                            label: "Inside right",
                            action: BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterRight),
                        },
                    );
                }
                options.push(BlockPlacementOption {
                    label: "Hidden",
                    action: BlockPlacementAction::HideNav,
                });
                options
            }
            LayoutBlock::Plan => vec![
                BlockPlacementOption {
                    label: "Left column",
                    action: BlockPlacementAction::Gutter(LayoutGutter::LeftColumn),
                },
                BlockPlacementOption {
                    label: "Right column",
                    action: BlockPlacementAction::Gutter(LayoutGutter::AfterEditor),
                },
                BlockPlacementOption {
                    label: "In Agent tab",
                    action: BlockPlacementAction::PlanTabInAgent,
                },
                BlockPlacementOption {
                    label: "Second window",
                    action: BlockPlacementAction::Gutter(LayoutGutter::Detached),
                },
            ],
            LayoutBlock::Agent => vec![
                BlockPlacementOption {
                    label: "Left column",
                    action: BlockPlacementAction::Gutter(LayoutGutter::LeftColumn),
                },
                BlockPlacementOption {
                    label: "Right column",
                    action: BlockPlacementAction::Gutter(LayoutGutter::AfterEditor),
                },
                BlockPlacementOption {
                    label: "Bottom band",
                    action: BlockPlacementAction::Gutter(LayoutGutter::Bottom),
                },
                BlockPlacementOption {
                    label: "Second window",
                    action: BlockPlacementAction::Gutter(LayoutGutter::Detached),
                },
            ],
            LayoutBlock::Editor => Vec::new(),
        }
    }

    pub fn is_placement_active(&self, block: LayoutBlock, action: BlockPlacementAction) -> bool {
        match (block, action) {
            (LayoutBlock::Threads, BlockPlacementAction::Gutter(LayoutGutter::FarLeft)) => {
                self.threads == ThreadsPlacement::Left
                    && self.outer_block_is_at_edge(LayoutBlock::Threads, DockSide::Left)
            }
            (LayoutBlock::Threads, BlockPlacementAction::Gutter(LayoutGutter::FarRight)) => {
                self.threads == ThreadsPlacement::Right
                    && self.outer_block_is_at_edge(LayoutBlock::Threads, DockSide::Right)
            }
            (
                LayoutBlock::Threads,
                BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterLeft),
            ) => {
                self.threads == ThreadsPlacement::Left
                    && self.nav == NavPlacement::Left
                    && self.outer_block_is_innermost(LayoutBlock::Threads, DockSide::Left)
            }
            (
                LayoutBlock::Threads,
                BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterRight),
            ) => {
                self.threads == ThreadsPlacement::Right
                    && self.nav == NavPlacement::Right
                    && self.outer_block_is_innermost(LayoutBlock::Threads, DockSide::Right)
            }
            (LayoutBlock::Threads, BlockPlacementAction::HideThreads) => {
                self.threads == ThreadsPlacement::Hidden
            }
            (LayoutBlock::Nav, BlockPlacementAction::Gutter(LayoutGutter::FarLeft)) => {
                self.nav == NavPlacement::Left
                    && self.outer_block_is_at_edge(LayoutBlock::Nav, DockSide::Left)
            }
            (LayoutBlock::Nav, BlockPlacementAction::Gutter(LayoutGutter::FarRight)) => {
                self.nav == NavPlacement::Right
                    && self.outer_block_is_at_edge(LayoutBlock::Nav, DockSide::Right)
            }
            (
                LayoutBlock::Nav,
                BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterLeft),
            ) => {
                self.nav == NavPlacement::Left
                    && self.threads == ThreadsPlacement::Left
                    && self.outer_block_is_innermost(LayoutBlock::Nav, DockSide::Left)
            }
            (
                LayoutBlock::Nav,
                BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterRight),
            ) => {
                self.nav == NavPlacement::Right
                    && self.threads == ThreadsPlacement::Right
                    && self.outer_block_is_innermost(LayoutBlock::Nav, DockSide::Right)
            }
            (LayoutBlock::Nav, BlockPlacementAction::HideNav) => self.nav == NavPlacement::Hidden,
            (LayoutBlock::Plan, BlockPlacementAction::Gutter(LayoutGutter::LeftColumn)) => {
                self.plan_column_side() == Some(DockSide::Left)
            }
            (LayoutBlock::Plan, BlockPlacementAction::Gutter(LayoutGutter::AfterEditor)) => {
                self.plan_column_side() == Some(DockSide::Right)
            }
            (LayoutBlock::Plan, BlockPlacementAction::PlanTabInAgent) => {
                matches!(self.plan, PlanHost::TabInAgent)
            }
            (LayoutBlock::Plan, BlockPlacementAction::Gutter(LayoutGutter::Detached)) => {
                self.plan_detached()
            }
            (LayoutBlock::Agent, BlockPlacementAction::Gutter(LayoutGutter::LeftColumn)) => {
                self.agent_column_side() == Some(DockSide::Left)
            }
            (LayoutBlock::Agent, BlockPlacementAction::Gutter(LayoutGutter::AfterEditor)) => {
                self.agent_column_side() == Some(DockSide::Right)
            }
            (LayoutBlock::Agent, BlockPlacementAction::Gutter(LayoutGutter::Bottom)) => {
                matches!(self.agent, AgentHost::Bottom { .. })
            }
            (LayoutBlock::Agent, BlockPlacementAction::Gutter(LayoutGutter::Detached)) => {
                self.agent_detached()
            }
            _ => false,
        }
    }

    pub fn try_apply_placement(
        &mut self,
        block: LayoutBlock,
        action: BlockPlacementAction,
    ) -> Result<(), LayoutValidationError> {
        match action {
            BlockPlacementAction::Gutter(gutter) => self.try_place_block_at_gutter(block, gutter),
            BlockPlacementAction::HideThreads => {
                if self.threads == ThreadsPlacement::Left {
                    self.threads_resume_side = DockSide::Left;
                } else if self.threads == ThreadsPlacement::Right {
                    self.threads_resume_side = DockSide::Right;
                }
                self.threads = ThreadsPlacement::Hidden;
                self.preset = LayoutPreset::Custom;
                Ok(())
            }
            BlockPlacementAction::HideNav => {
                if self.nav == NavPlacement::Left {
                    self.nav_resume_side = DockSide::Left;
                } else if self.nav == NavPlacement::Right {
                    self.nav_resume_side = DockSide::Right;
                }
                self.nav = NavPlacement::Hidden;
                self.preset = LayoutPreset::Custom;
                Ok(())
            }
            BlockPlacementAction::PlanTabInAgent => {
                self.plan = PlanHost::TabInAgent;
                self.preset = LayoutPreset::Custom;
                self.validate()
            }
        }
    }

    pub fn drag_gutters_for(&self, block: LayoutBlock) -> Vec<LayoutGutter> {
        [
            LayoutGutter::FarLeft,
            LayoutGutter::BetweenOuterLeft,
            LayoutGutter::BeforeEditor,
            LayoutGutter::AfterEditor,
            LayoutGutter::BetweenOuterRight,
            LayoutGutter::FarRight,
            LayoutGutter::Bottom,
            LayoutGutter::Detached,
        ]
        .into_iter()
        .filter(|gutter| {
            matches!(
                Self::preview_placement(self, block, BlockPlacementAction::Gutter(*gutter)),
                PlacementOutcome::Changed
            )
        })
        .collect()
    }

    pub fn meaningful_placement_actions_for(&self, block: LayoutBlock) -> Vec<BlockPlacementOption> {
        self.placement_options_for(block)
            .into_iter()
            .filter(|option| match Self::preview_placement(self, block, option.action) {
                PlacementOutcome::Changed => true,
                PlacementOutcome::AlreadyThere(_) => {
                    self.is_placement_active(block, option.action)
                }
                PlacementOutcome::Invalid(_) => true,
            })
            .collect()
    }

    pub fn placement_ring(&self, block: LayoutBlock) -> Vec<BlockPlacementAction> {
        self.placement_options_for(block)
            .into_iter()
            .filter(|option| {
                !matches!(
                    Self::preview_placement(self, block, option.action),
                    PlacementOutcome::Invalid(_)
                )
            })
            .map(|option| option.action)
            .collect()
    }

    pub fn apply_placement_step(
        &mut self,
        block: LayoutBlock,
        direction: PlacementStepDirection,
    ) -> PlacementOutcome {
        let ring = self.placement_ring(block);
        if ring.is_empty() {
            return PlacementOutcome::Invalid(LayoutValidationError::UnsupportedSlot);
        }

        let current = ring
            .iter()
            .position(|action| self.is_placement_active(block, *action));
        let len = ring.len();
        let next_idx = match (current, direction) {
            (Some(idx), PlacementStepDirection::Prev) => (idx + len - 1) % len,
            (Some(idx), PlacementStepDirection::Next) => (idx + 1) % len,
            (None, PlacementStepDirection::Prev) => len - 1,
            (None, PlacementStepDirection::Next) => 0,
        };

        if current == Some(next_idx) {
            return PlacementOutcome::AlreadyThere(
                Self::placement_already_there_message(block, ring[next_idx]),
            );
        }

        self.apply_placement(block, ring[next_idx])
    }

    pub fn block_is_hidden(&self, block: LayoutBlock) -> bool {
        match block {
            LayoutBlock::Threads => self.threads == ThreadsPlacement::Hidden,
            LayoutBlock::Nav => self.nav == NavPlacement::Hidden,
            _ => false,
        }
    }

    pub fn shows_plan_badge_on_agent(&self) -> bool {
        matches!(self.plan, PlanHost::TabInAgent)
    }

    /// Runtime slot for the multi-workspace threads sidebar on `side`.
    pub fn threads_sidebar_slot(&self, side: DockSide) -> ThreadsSidebarSlot {
        let threads_on_side = match side {
            DockSide::Left => self.threads == ThreadsPlacement::Left,
            DockSide::Right => self.threads == ThreadsPlacement::Right,
        };
        if self.threads == ThreadsPlacement::Hidden || !threads_on_side {
            return ThreadsSidebarSlot::Absent;
        }

        let nav_on_same_side = match side {
            DockSide::Left => self.nav == NavPlacement::Left,
            DockSide::Right => self.nav == NavPlacement::Right,
        };
        if !nav_on_same_side {
            return ThreadsSidebarSlot::Outer;
        }

        let nav_first = match side {
            DockSide::Left => self.outer_left_order == OuterStripOrder::NavFirst,
            DockSide::Right => self.outer_right_order == OuterStripOrder::NavFirst,
        };
        if nav_first {
            ThreadsSidebarSlot::Inner
        } else {
            ThreadsSidebarSlot::Outer
        }
    }

    pub fn agent_and_nav_share_side(&self) -> bool {
        match (&self.agent, self.nav) {
            (AgentHost::Column { side, .. }, NavPlacement::Left) => *side == DockSide::Left,
            (AgentHost::Column { side, .. }, NavPlacement::Right) => *side == DockSide::Right,
            _ => false,
        }
    }

    /// Runtime slot for the agent panel on `side` when sharing with nav.
    pub fn agent_panel_slot(&self, side: DockSide) -> AgentPanelSlot {
        let agent_on_side = match &self.agent {
            AgentHost::Column { side: agent_side, .. } => *agent_side == side,
            _ => return AgentPanelSlot::Absent,
        };
        if !agent_on_side {
            return AgentPanelSlot::Absent;
        }

        let nav_on_same_side = match side {
            DockSide::Left => self.nav == NavPlacement::Left,
            DockSide::Right => self.nav == NavPlacement::Right,
        };
        if self.nav == NavPlacement::Hidden || !nav_on_same_side {
            return AgentPanelSlot::Outer;
        }

        AgentPanelSlot::Inner
    }

    /// Ordered columns on `side` (screen edge → editor) when using split row layout.
    pub fn side_columns(&self, side: DockSide) -> Vec<SideColumnKind> {
        let nav_on_side = match side {
            DockSide::Left => self.nav == NavPlacement::Left,
            DockSide::Right => self.nav == NavPlacement::Right,
        };
        if self.nav == NavPlacement::Hidden || !nav_on_side {
            return Vec::new();
        }

        // Edge→editor order on both sides: Nav (edge), Threads (inside),
        // Agent (innermost, adjacent to the editor). This matches the
        // Arrange Workspace schematic — the agent column always hugs the
        // editor regardless of which side the strip lives on.
        let mut columns = vec![SideColumnKind::NavDock];
        if self.threads_sidebar_slot(side) == ThreadsSidebarSlot::Inner {
            columns.push(SideColumnKind::ThreadsSidebar);
        }
        if self.agent_panel_slot(side) == AgentPanelSlot::Inner {
            columns.push(SideColumnKind::AgentPanel);
        }
        columns
    }

    /// True when `side` has multiple tiled columns (split row).
    pub fn uses_side_column_row(&self, side: DockSide) -> bool {
        self.side_columns(side).len() > 1
    }

    /// Estimated minimum editor width consumed by fixed columns on both sides.
    pub fn minimum_side_columns_width(&self) -> Pixels {
        let mut total = Pixels::ZERO;
        for side in [DockSide::Left, DockSide::Right] {
            for column in self.side_columns(side) {
                total += match column {
                    SideColumnKind::NavDock => px(180.),
                    SideColumnKind::AgentPanel => px(300.),
                    SideColumnKind::ThreadsSidebar => px(200.),
                };
            }
        }
        total
    }

    pub fn editor_crowding_warning(&self) -> Option<&'static str> {
        let crowded_side = [DockSide::Left, DockSide::Right]
            .into_iter()
            .find(|side| self.side_columns(*side).len() >= 3)?;
        match crowded_side {
            DockSide::Left => Some(
                "Three columns on the left — editor may be narrow. Consider moving Threads or Nav.",
            ),
            DockSide::Right => Some(
                "Three columns on the right — editor may be narrow. Consider moving Threads or Nav.",
            ),
        }
    }

    pub fn control_panel_segments(&self) -> Vec<ControlPanelSegment> {
        let mut segments = Vec::new();

        for block in self.outer_left_blocks() {
            segments.push(ControlPanelSegment::Block(block));
        }
        if let PlanHost::Column {
            side: DockSide::Left,
            ..
        } = self.plan
        {
            segments.push(ControlPanelSegment::Block(LayoutBlock::Plan));
        }
        if matches!(
            self.agent,
            AgentHost::Column {
                side: DockSide::Left,
                ..
            }
        ) {
            segments.push(ControlPanelSegment::Block(LayoutBlock::Agent));
        }

        segments.push(ControlPanelSegment::Block(LayoutBlock::Editor));

        if matches!(
            self.agent,
            AgentHost::Column {
                side: DockSide::Right,
                ..
            }
        ) {
            segments.push(ControlPanelSegment::Block(LayoutBlock::Agent));
        }
        if let PlanHost::Column {
            side: DockSide::Right,
            ..
        } = self.plan
        {
            segments.push(ControlPanelSegment::Block(LayoutBlock::Plan));
        }

        for block in self.outer_right_blocks() {
            segments.push(ControlPanelSegment::Block(block));
        }
        if self.threads == ThreadsPlacement::Hidden {
            segments.push(ControlPanelSegment::OffChip(LayoutBlock::Threads));
        }
        if self.nav == NavPlacement::Hidden {
            segments.push(ControlPanelSegment::OffChip(LayoutBlock::Nav));
        }

        segments
    }

    pub fn has_unsaved_changes(&self, saved: &Self) -> bool {
        !self.placement_equivalent(saved)
    }

    pub fn apply_summary(&self) -> String {
        let mut parts = Vec::new();

        match &self.agent {
            AgentHost::Column { side: DockSide::Left, .. } => parts.push("Agent docked left"),
            AgentHost::Column { side: DockSide::Right, .. } => parts.push("Agent docked right"),
            AgentHost::Bottom { .. } => parts.push("Agent in bottom band"),
            AgentHost::Detached { .. } => parts.push("Agent in second window"),
        }

        match &self.plan {
            PlanHost::TabInAgent => parts.push("Plan tab focused"),
            PlanHost::Column { .. } => parts.push("Plan tab focused"),
            PlanHost::Detached { .. } => parts.push("Plan opened separately"),
        }

        match self.nav {
            NavPlacement::Hidden => parts.push("Nav hidden"),
            _ => parts.push("Nav visible"),
        }

        if self.threads == ThreadsPlacement::Hidden {
            parts.push("Threads hidden");
        }

        if self.plan_strip_only || matches!(self.preset, LayoutPreset::Implement) {
            parts.push("Plan strip active");
        }

        parts.join(" · ")
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_preset_validates() {
        LayoutBlueprint::plan().validate().unwrap();
    }

    #[test]
    fn reject_plan_agent_same_column() {
        let mut blueprint = LayoutBlueprint::plan();
        blueprint.agent = AgentHost::Column {
            side: DockSide::Left,
            width: LayoutBlueprint::DEFAULT_AGENT_WIDTH,
        };
        blueprint.plan = PlanHost::Column {
            side: DockSide::Left,
            width: LayoutBlueprint::DEFAULT_PLAN_WIDTH,
        };
        assert_eq!(
            blueprint.validate(),
            Err(LayoutValidationError::PlanAgentSameColumn)
        );
    }

    #[test]
    fn dual_preset_requests_detached_plan() {
        assert!(LayoutBlueprint::dual().plan_detached());
    }

    #[test]
    fn try_place_agent_on_plan_slot_fails_when_collision() {
        let mut blueprint = LayoutBlueprint::plan();
        blueprint
            .try_place_block(LayoutBlock::Agent, LayoutSlot::LeftInner)
            .unwrap_err();
    }

    #[test]
    fn resolve_nav_inner_drop_to_outer_edge() {
        assert_eq!(
            LayoutBlueprint::resolve_drop_slot(LayoutBlock::Nav, LayoutSlot::RightInner),
            LayoutSlot::RightOuter
        );
    }

    #[test]
    fn preview_place_nav_on_agent_column_succeeds() {
        LayoutBlueprint::preview_place(
            &LayoutBlueprint::plan(),
            LayoutBlock::Nav,
            LayoutSlot::RightInner,
        )
        .unwrap();
    }

    #[test]
    fn with_block_placed_returns_updated_blueprint() {
        let next = LayoutBlueprint::with_block_placed(
            &LayoutBlueprint::plan(),
            LayoutBlock::Nav,
            LayoutSlot::RightInner,
        )
        .unwrap();
        assert_eq!(next.nav, NavPlacement::Right);
    }

    #[test]
    fn threads_after_editor_resolves_to_right_edge() {
        LayoutBlueprint::with_block_at_gutter(
            &LayoutBlueprint::plan(),
            LayoutBlock::Threads,
            LayoutGutter::AfterEditor,
        )
        .unwrap();
    }

    #[test]
    fn reject_unsupported_plan_bottom_slot() {
        assert_eq!(
            LayoutBlueprint::preview_place(
                &LayoutBlueprint::plan(),
                LayoutBlock::Plan,
                LayoutSlot::Bottom,
            ),
            Err(LayoutValidationError::UnsupportedSlot)
        );
    }

    #[test]
    fn between_outer_left_places_nav_inside_threads() {
        let mut blueprint = LayoutBlueprint::plan();
        blueprint
            .try_place_block_at_gutter(LayoutBlock::Nav, LayoutGutter::BetweenOuterLeft)
            .unwrap();
        assert_eq!(
            blueprint.outer_left_blocks(),
            vec![LayoutBlock::Threads, LayoutBlock::Nav]
        );
    }

    #[test]
    fn between_outer_left_swaps_threads_inside_nav() {
        let mut blueprint = LayoutBlueprint::plan();
        blueprint.outer_left_order = OuterStripOrder::NavFirst;
        blueprint
            .try_place_block_at_gutter(LayoutBlock::Threads, LayoutGutter::BetweenOuterLeft)
            .unwrap();
        assert_eq!(
            blueprint.outer_left_blocks(),
            vec![LayoutBlock::Nav, LayoutBlock::Threads]
        );
    }

    #[test]
    fn placement_options_include_inside_when_both_outer_left() {
        let blueprint = LayoutBlueprint::plan();
        let options = blueprint.placement_options_for(LayoutBlock::Nav);
        assert!(options.iter().any(|option| {
            matches!(
                option.action,
                BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterLeft)
            )
        }));
    }

    #[test]
    fn agent_left_column_disabled_when_plan_on_left() {
        let blueprint = LayoutBlueprint::plan();
        let reason = LayoutBlueprint::placement_disabled_reason(
            &blueprint,
            LayoutBlock::Agent,
            BlockPlacementAction::Gutter(LayoutGutter::LeftColumn),
        );
        assert_eq!(
            reason,
            Some("Plan and Agent can't share the same side")
        );
    }

    #[test]
    fn far_left_nav_places_nav_at_edge() {
        let mut blueprint = LayoutBlueprint::plan();
        blueprint
            .try_place_block_at_gutter(LayoutBlock::Nav, LayoutGutter::FarLeft)
            .unwrap();
        assert_eq!(blueprint.outer_left_order, OuterStripOrder::NavFirst);
    }

    #[test]
    fn threads_before_editor_is_already_there_when_on_left() {
        let blueprint = LayoutBlueprint::plan();
        assert_eq!(
            LayoutBlueprint::preview_placement(
                &blueprint,
                LayoutBlock::Threads,
                BlockPlacementAction::Gutter(LayoutGutter::BeforeEditor),
            ),
            PlacementOutcome::AlreadyThere("Threads is already on the left edge")
        );
    }

    #[test]
    fn apply_placement_threads_right_changes_classic() {
        let mut blueprint = LayoutBlueprint::classic();
        assert_eq!(
            blueprint.apply_placement(
                LayoutBlock::Threads,
                BlockPlacementAction::Gutter(LayoutGutter::FarRight),
            ),
            PlacementOutcome::Changed
        );
        assert_eq!(blueprint.threads, ThreadsPlacement::Right);
    }

    #[test]
    fn apply_placement_threads_left_is_already_there_on_classic() {
        let mut blueprint = LayoutBlueprint::classic();
        assert_eq!(
            blueprint.apply_placement(
                LayoutBlock::Threads,
                BlockPlacementAction::Gutter(LayoutGutter::FarLeft),
            ),
            PlacementOutcome::AlreadyThere("Threads is already on the left edge")
        );
        assert_eq!(blueprint.threads, ThreadsPlacement::Left);
    }

    #[test]
    fn classic_outer_strip_edge_vs_inside_active_state() {
        let blueprint = LayoutBlueprint::classic();
        assert_eq!(blueprint.block_status_short(LayoutBlock::Threads), "Edge L");
        assert_eq!(blueprint.block_status_short(LayoutBlock::Nav), "Inside L");
        assert!(blueprint.is_placement_active(
            LayoutBlock::Threads,
            BlockPlacementAction::Gutter(LayoutGutter::FarLeft),
        ));
        assert!(!blueprint.is_placement_active(
            LayoutBlock::Threads,
            BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterLeft),
        ));
        assert!(!blueprint.is_placement_active(
            LayoutBlock::Nav,
            BlockPlacementAction::Gutter(LayoutGutter::FarLeft),
        ));
        assert!(blueprint.is_placement_active(
            LayoutBlock::Nav,
            BlockPlacementAction::Gutter(LayoutGutter::BetweenOuterLeft),
        ));
    }

    #[test]
    fn classic_nav_inside_steps_to_right_in_one_click() {
        let mut blueprint = LayoutBlueprint::classic();
        let outcome = blueprint.apply_placement_step(
            LayoutBlock::Nav,
            PlacementStepDirection::Next,
        );
        assert_eq!(outcome, PlacementOutcome::Changed);
        assert_eq!(blueprint.nav, NavPlacement::Right);
        assert_eq!(blueprint.block_status_short(LayoutBlock::Nav), "Right");
    }

    #[test]
    fn side_columns_left_nav_threads_agent_order() {
        let mut blueprint = LayoutBlueprint::classic();
        blueprint
            .try_place_block_at_gutter(LayoutBlock::Nav, LayoutGutter::FarLeft)
            .unwrap();
        blueprint.agent = AgentHost::Column {
            side: DockSide::Left,
            width: LayoutBlueprint::DEFAULT_AGENT_WIDTH,
        };
        // Edge→editor: Nav at the edge, Threads inside, Agent adjacent to
        // the editor — mirrors the right-side order.
        assert_eq!(
            blueprint.side_columns(DockSide::Left),
            vec![
                SideColumnKind::NavDock,
                SideColumnKind::ThreadsSidebar,
                SideColumnKind::AgentPanel,
            ]
        );
        assert!(blueprint.uses_side_column_row(DockSide::Left));
        assert!(blueprint.editor_crowding_warning().is_some());
    }

    #[test]
    fn side_columns_right_threads_before_agent() {
        let mut blueprint = LayoutBlueprint::classic();
        blueprint.nav = NavPlacement::Right;
        blueprint.agent = AgentHost::Column {
            side: DockSide::Right,
            width: LayoutBlueprint::DEFAULT_AGENT_WIDTH,
        };
        blueprint.threads = ThreadsPlacement::Right;
        assert_eq!(
            blueprint.side_columns(DockSide::Right),
            vec![
                SideColumnKind::NavDock,
                SideColumnKind::ThreadsSidebar,
                SideColumnKind::AgentPanel,
            ]
        );
    }

    #[test]
    fn side_columns_single_nav_no_row() {
        let blueprint = LayoutBlueprint::classic();
        assert_eq!(
            blueprint.side_columns(DockSide::Left),
            vec![SideColumnKind::NavDock]
        );
        assert!(!blueprint.uses_side_column_row(DockSide::Left));
    }

    #[test]
    fn classic_threads_sidebar_outer_nav_first_inner() {
        let classic = LayoutBlueprint::classic();
        assert_eq!(
            classic.threads_sidebar_slot(DockSide::Left),
            ThreadsSidebarSlot::Outer
        );

        let mut nav_first = LayoutBlueprint::classic();
        nav_first
            .try_place_block_at_gutter(LayoutBlock::Nav, LayoutGutter::FarLeft)
            .unwrap();
        assert_eq!(nav_first.outer_left_order, OuterStripOrder::NavFirst);
        assert_eq!(
            nav_first.threads_sidebar_slot(DockSide::Left),
            ThreadsSidebarSlot::Inner
        );
    }

    #[test]
    fn agentic_threads_sidebar_outer_on_right() {
        let agentic = LayoutBlueprint::agentic();
        assert_eq!(
            agentic.threads_sidebar_slot(DockSide::Left),
            ThreadsSidebarSlot::Outer
        );
        assert_eq!(
            agentic.threads_sidebar_slot(DockSide::Right),
            ThreadsSidebarSlot::Absent
        );
    }

    #[test]
    fn agent_nav_share_side_agent_inner_on_right() {
        let mut blueprint = LayoutBlueprint::classic();
        blueprint.nav = NavPlacement::Right;
        assert!(blueprint.agent_and_nav_share_side());
        assert_eq!(
            blueprint.agent_panel_slot(DockSide::Right),
            AgentPanelSlot::Inner
        );
        assert_eq!(
            blueprint.agent_panel_slot(DockSide::Left),
            AgentPanelSlot::Absent
        );
    }

    #[test]
    fn agent_nav_opposite_sides_agent_outer() {
        let classic = LayoutBlueprint::classic();
        assert!(!classic.agent_and_nav_share_side());
        assert_eq!(
            classic.agent_panel_slot(DockSide::Right),
            AgentPanelSlot::Outer
        );
    }

    #[test]
    fn agent_nav_share_side_agent_inner_on_left() {
        let mut blueprint = LayoutBlueprint::agentic();
        blueprint.nav = NavPlacement::Left;
        blueprint.agent = AgentHost::Column {
            side: DockSide::Left,
            width: LayoutBlueprint::DEFAULT_AGENT_WIDTH,
        };
        assert!(blueprint.agent_and_nav_share_side());
        assert_eq!(
            blueprint.agent_panel_slot(DockSide::Left),
            AgentPanelSlot::Inner
        );
    }

    #[test]
    fn write_settings_keeps_nav_on_agent_side_when_split() {
        use settings::DockPosition;

        let mut blueprint = LayoutBlueprint::classic();
        blueprint.nav = NavPlacement::Right;
        let merged = PanelLayout {
            agent_dock: Some(DockPosition::Right),
            project_panel_dock: Some(DockSide::Left),
            outline_panel_dock: Some(DockSide::Left),
            collaboration_panel_dock: Some(DockPosition::Left),
            git_panel_dock: Some(DockPosition::Left),
        };
        let mut settings = SettingsContent::default();
        blueprint.write_settings(&merged, &mut settings);
        assert_eq!(
            settings.project_panel.as_ref().and_then(|p| p.dock),
            Some(DockSide::Right)
        );
        assert_eq!(
            settings.outline_panel.as_ref().and_then(|p| p.dock),
            Some(DockSide::Right)
        );
    }

    #[test]
    fn drag_gutters_exclude_noop_before_editor_for_threads_on_left() {
        let blueprint = LayoutBlueprint::plan();
        let gutters = blueprint.drag_gutters_for(LayoutBlock::Threads);
        assert!(!gutters.contains(&LayoutGutter::BeforeEditor));
        assert!(!gutters.contains(&LayoutGutter::FarLeft));
        assert!(gutters.contains(&LayoutGutter::AfterEditor));
    }

    #[test]
    fn drag_gutters_include_inside_left_when_nav_at_edge() {
        let mut blueprint = LayoutBlueprint::plan();
        blueprint.outer_left_order = OuterStripOrder::NavFirst;
        let gutters = blueprint.drag_gutters_for(LayoutBlock::Nav);
        assert!(gutters.contains(&LayoutGutter::BetweenOuterLeft));
    }

    #[test]
    fn agent_left_column_invalid_when_plan_on_left() {
        let blueprint = LayoutBlueprint::plan();
        assert_eq!(
            LayoutBlueprint::preview_placement(
                &blueprint,
                LayoutBlock::Agent,
                BlockPlacementAction::Gutter(LayoutGutter::LeftColumn),
            ),
            PlacementOutcome::Invalid(LayoutValidationError::PlanAgentSameColumn)
        );
    }

    #[test]
    fn placement_matrix_threads_from_plan_preset() {
        let blueprint = LayoutBlueprint::plan();
        assert!(matches!(
            LayoutBlueprint::preview_placement(
                &blueprint,
                LayoutBlock::Threads,
                BlockPlacementAction::Gutter(LayoutGutter::FarRight),
            ),
            PlacementOutcome::Changed
        ));
        assert!(matches!(
            LayoutBlueprint::preview_placement(
                &blueprint,
                LayoutBlock::Threads,
                BlockPlacementAction::HideThreads,
            ),
            PlacementOutcome::Changed
        ));
    }

    #[test]
    fn menu_options_include_active_and_hide_redundant_noops() {
        let blueprint = LayoutBlueprint::classic();
        let options = blueprint.meaningful_placement_actions_for(LayoutBlock::Threads);
        assert!(options.iter().any(|option| {
            option.label == "Left edge"
                && blueprint.is_placement_active(LayoutBlock::Threads, option.action)
        }));
        assert!(options.iter().any(|option| option.label == "Right edge"));
        assert!(!options.iter().any(|option| {
            matches!(
                option.action,
                BlockPlacementAction::Gutter(LayoutGutter::BeforeEditor)
            )
        }));
    }

    #[test]
    fn plan_column_is_not_ghost() {
        let blueprint = LayoutBlueprint::plan();
        assert!(!blueprint.block_is_ghost(LayoutBlock::Plan));
        let classic = LayoutBlueprint::classic();
        assert!(classic.block_is_ghost(LayoutBlock::Plan));
    }

    #[test]
    fn apply_summary_classic() {
        assert_eq!(
            LayoutBlueprint::classic().apply_summary(),
            "Agent docked right · Plan tab focused · Nav visible"
        );
    }

    #[test]
    fn apply_summary_dual() {
        let summary = LayoutBlueprint::dual().apply_summary();
        assert!(summary.contains("Plan opened separately"));
    }

    #[test]
    fn focus_slider_width_is_not_unsaved_change() {
        let saved = LayoutBlueprint::plan();
        let mut edited = saved.clone();
        edited.focus_slider = 0.9;
        edited.apply_focus_slider();
        assert!(!edited.has_unsaved_changes(&saved));
    }

    #[test]
    fn threads_move_is_unsaved_change() {
        let saved = LayoutBlueprint::classic();
        let mut edited = saved.clone();
        edited.threads = ThreadsPlacement::Right;
        assert!(edited.has_unsaved_changes(&saved));
    }

    mod track5 {
        use super::*;
        use settings::{SettingsContent, SidebarDockPosition};

        fn editor_layout_settings() -> SettingsContent {
            let mut settings = SettingsContent::default();
            let merged = PanelLayout::from_merged_settings(&settings);
            LayoutBlueprint::classic().write_settings(&merged, &mut settings);
            settings
        }

        fn settings_after_apply(
            blueprint: &LayoutBlueprint,
            base: &SettingsContent,
        ) -> SettingsContent {
            let merged = PanelLayout::from_merged_settings(base);
            let mut settings = base.clone();
            blueprint.write_settings(&merged, &mut settings);
            settings
        }

        fn blueprint_after_apply(
            blueprint: &LayoutBlueprint,
            base: &SettingsContent,
        ) -> LayoutBlueprint {
            LayoutBlueprint::from_merged_settings(&settings_after_apply(blueprint, base))
        }

        #[test]
        fn all_presets_validate() {
            for preset in [
                LayoutBlueprint::plan(),
                LayoutBlueprint::implement(),
                LayoutBlueprint::classic(),
                LayoutBlueprint::agentic(),
                LayoutBlueprint::dual(),
            ] {
                preset.validate().expect("preset should validate");
            }
        }

        #[test]
        fn classic_preset_roundtrips_persisted_fields() {
            let base = editor_layout_settings();
            let blueprint = LayoutBlueprint::classic();
            let loaded = blueprint_after_apply(&blueprint, &base);

            assert_eq!(loaded.preset, LayoutPreset::Custom);
            assert_eq!(loaded.threads, blueprint.threads);
            assert!(matches!(
                loaded.agent,
                AgentHost::Column {
                    side: DockSide::Right,
                    ..
                }
            ));
            assert_eq!(loaded.nav, NavPlacement::Left);
            assert!(matches!(loaded.plan, PlanHost::TabInAgent));
        }

        #[test]
        fn agentic_preset_roundtrips_persisted_fields() {
            let base = editor_layout_settings();
            let blueprint = LayoutBlueprint::agentic();
            let loaded = blueprint_after_apply(&blueprint, &base);

            assert_eq!(loaded.preset, LayoutPreset::Custom);
            assert!(matches!(
                loaded.agent,
                AgentHost::Column {
                    side: DockSide::Left,
                    ..
                }
            ));
            assert_eq!(loaded.nav, NavPlacement::Right);
        }

        #[test]
        fn plan_column_roundtrips() {
            let base = editor_layout_settings();
            let blueprint = LayoutBlueprint::plan();
            assert!(matches!(blueprint.plan, PlanHost::Column { .. }));

            let loaded = blueprint_after_apply(&blueprint, &base);
            assert!(matches!(
                loaded.plan,
                PlanHost::Column {
                    side: DockSide::Left,
                    ..
                }
            ));
            assert!(loaded.placement_equivalent(&blueprint));
        }

        #[test]
        fn detached_plan_roundtrips() {
            let base = editor_layout_settings();
            let blueprint = LayoutBlueprint::dual();
            assert!(blueprint.plan_detached());

            let loaded = blueprint_after_apply(&blueprint, &base);
            assert!(loaded.plan_detached());
            assert!(loaded.placement_equivalent(&blueprint));
        }

        #[test]
        fn outer_strip_order_roundtrips() {
            let base = editor_layout_settings();
            let mut blueprint = LayoutBlueprint::classic();
            blueprint
                .try_place_block_at_gutter(LayoutBlock::Nav, LayoutGutter::FarLeft)
                .unwrap();
            assert_eq!(blueprint.outer_left_order, OuterStripOrder::NavFirst);

            let loaded = blueprint_after_apply(&blueprint, &base);
            assert_eq!(loaded.outer_left_order, OuterStripOrder::NavFirst);
            assert_eq!(
                loaded.outer_left_blocks(),
                vec![LayoutBlock::Nav, LayoutBlock::Threads]
            );
            assert!(loaded.placement_equivalent(&blueprint));
        }

        #[test]
        fn nav_hidden_roundtrips() {
            let base = editor_layout_settings();
            let blueprint = LayoutBlueprint::implement();
            assert_eq!(blueprint.nav, NavPlacement::Hidden);

            let loaded = blueprint_after_apply(&blueprint, &base);
            assert_eq!(loaded.nav, NavPlacement::Hidden);
            assert!(loaded.placement_equivalent(&blueprint));
        }

        #[test]
        fn plan_strip_only_roundtrips() {
            let base = editor_layout_settings();
            let blueprint = LayoutBlueprint::implement();
            assert!(blueprint.plan_strip_only);

            let loaded = blueprint_after_apply(&blueprint, &base);
            assert!(loaded.plan_strip_only);
            assert!(loaded.placement_equivalent(&blueprint));
        }

        #[test]
        fn hidden_threads_steps_to_visible() {
            let mut blueprint = LayoutBlueprint::classic();
            blueprint.apply_placement(
                LayoutBlock::Threads,
                BlockPlacementAction::HideThreads,
            );
            assert_eq!(blueprint.threads, ThreadsPlacement::Hidden);

            let outcome = blueprint.apply_placement_step(
                LayoutBlock::Threads,
                PlacementStepDirection::Next,
            );
            assert_eq!(outcome, PlacementOutcome::Changed);
            assert_ne!(blueprint.threads, ThreadsPlacement::Hidden);
        }

        #[test]
        fn placement_ring_includes_hide_and_unhide() {
            let blueprint = LayoutBlueprint::classic();
            let ring = blueprint.placement_ring(LayoutBlock::Threads);
            assert!(ring.contains(&BlockPlacementAction::HideThreads));
            assert!(ring.contains(&BlockPlacementAction::Gutter(LayoutGutter::FarLeft)));

            let mut hidden = blueprint.clone();
            hidden.threads = ThreadsPlacement::Hidden;
            let hidden_ring = hidden.placement_ring(LayoutBlock::Threads);
            assert!(hidden_ring.contains(&BlockPlacementAction::HideThreads));
            assert!(hidden.is_placement_active(
                LayoutBlock::Threads,
                BlockPlacementAction::HideThreads
            ));

            let outcome = hidden.apply_placement_step(
                LayoutBlock::Threads,
                PlacementStepDirection::Next,
            );
            assert_eq!(outcome, PlacementOutcome::Changed);
            assert_ne!(hidden.threads, ThreadsPlacement::Hidden);
        }

        #[test]
        fn control_panel_segments_reflect_hidden_and_plan_tab() {
            let classic = LayoutBlueprint::classic();
            let segments = classic.control_panel_segments();
            assert!(!segments.iter().any(|segment| {
                matches!(segment, ControlPanelSegment::Block(LayoutBlock::Plan))
            }));
            assert!(classic.shows_plan_badge_on_agent());

            let implement = LayoutBlueprint::implement();
            let implement_segments = implement.control_panel_segments();
            assert!(implement_segments.contains(&ControlPanelSegment::OffChip(
                LayoutBlock::Nav
            )));
        }

        #[test]
        fn threads_sidebar_side_persists_on_apply() {
            let base = editor_layout_settings();
            let mut blueprint = LayoutBlueprint::classic();
            blueprint.threads = ThreadsPlacement::Right;
            blueprint.preset = LayoutPreset::Custom;

            let settings = settings_after_apply(&blueprint, &base);
            let loaded = LayoutBlueprint::from_merged_settings(&settings);
            assert_eq!(loaded.threads, ThreadsPlacement::Right);
            assert_eq!(
                settings
                    .agent
                    .as_ref()
                    .and_then(|agent| agent.sidebar_side),
                Some(SidebarDockPosition::Right)
            );
        }

        #[test]
        fn placement_matrix() {
            struct Case {
                name: &'static str,
                blueprint: fn() -> LayoutBlueprint,
                block: LayoutBlock,
                action: BlockPlacementAction,
                changed: bool,
                invalid: bool,
            }

            let cases = [
                Case {
                    name: "classic threads right",
                    blueprint: LayoutBlueprint::classic,
                    block: LayoutBlock::Threads,
                    action: BlockPlacementAction::Gutter(LayoutGutter::FarRight),
                    changed: true,
                    invalid: false,
                },
                Case {
                    name: "classic threads left noop",
                    blueprint: LayoutBlueprint::classic,
                    block: LayoutBlock::Threads,
                    action: BlockPlacementAction::Gutter(LayoutGutter::FarLeft),
                    changed: false,
                    invalid: false,
                },
                Case {
                    name: "classic before editor noop",
                    blueprint: LayoutBlueprint::classic,
                    block: LayoutBlock::Threads,
                    action: BlockPlacementAction::Gutter(LayoutGutter::BeforeEditor),
                    changed: false,
                    invalid: false,
                },
                Case {
                    name: "plan agent left collision",
                    blueprint: LayoutBlueprint::plan,
                    block: LayoutBlock::Agent,
                    action: BlockPlacementAction::Gutter(LayoutGutter::LeftColumn),
                    changed: false,
                    invalid: true,
                },
                Case {
                    name: "plan threads hide",
                    blueprint: LayoutBlueprint::plan,
                    block: LayoutBlock::Threads,
                    action: BlockPlacementAction::HideThreads,
                    changed: true,
                    invalid: false,
                },
                Case {
                    name: "classic plan tab already there",
                    blueprint: LayoutBlueprint::classic,
                    block: LayoutBlock::Plan,
                    action: BlockPlacementAction::PlanTabInAgent,
                    changed: false,
                    invalid: false,
                },
                Case {
                    name: "classic plan left column changes",
                    blueprint: LayoutBlueprint::classic,
                    block: LayoutBlock::Plan,
                    action: BlockPlacementAction::Gutter(LayoutGutter::LeftColumn),
                    changed: true,
                    invalid: false,
                },
                Case {
                    name: "implement agent bottom invalid for plan",
                    blueprint: LayoutBlueprint::implement,
                    block: LayoutBlock::Plan,
                    action: BlockPlacementAction::Gutter(LayoutGutter::Bottom),
                    changed: false,
                    invalid: true,
                },
                Case {
                    name: "dual plan detached already there",
                    blueprint: LayoutBlueprint::dual,
                    block: LayoutBlock::Plan,
                    action: BlockPlacementAction::Gutter(LayoutGutter::Detached),
                    changed: false,
                    invalid: false,
                },
                Case {
                    name: "agentic nav left edge",
                    blueprint: LayoutBlueprint::agentic,
                    block: LayoutBlock::Nav,
                    action: BlockPlacementAction::Gutter(LayoutGutter::FarLeft),
                    changed: true,
                    invalid: false,
                },
            ];

            for case in cases {
                let blueprint = (case.blueprint)();
                let outcome =
                    LayoutBlueprint::preview_placement(&blueprint, case.block, case.action);
                if case.invalid {
                    assert!(
                        matches!(outcome, PlacementOutcome::Invalid(_)),
                        "{} expected Invalid, got {:?}",
                        case.name,
                        outcome
                    );
                } else if case.changed {
                    assert_eq!(
                        outcome,
                        PlacementOutcome::Changed,
                        "{} expected Changed",
                        case.name
                    );
                } else {
                    assert!(
                        matches!(outcome, PlacementOutcome::AlreadyThere(_)),
                        "{} expected AlreadyThere, got {:?}",
                        case.name,
                        outcome
                    );
                }
            }
        }

        #[test]
        fn block_is_ghost_when_plan_in_agent_tab() {
            let blueprint = LayoutBlueprint::classic();
            assert!(blueprint.block_is_ghost(LayoutBlock::Plan));
            assert!(!blueprint.block_is_ghost(LayoutBlock::Editor));
        }

        #[test]
        fn apply_summary_implement_includes_strip() {
            let summary = LayoutBlueprint::implement().apply_summary();
            assert!(summary.contains("Plan strip active"));
            assert!(summary.contains("Nav hidden"));
        }
    }
}
