use agent_settings::{
    AgentPanelSlot, LayoutBlueprint, SideColumnKind, ThreadsSidebarSlot,
};
use gpui::{
    App, Axis, Context, InteractiveElement, MouseButton, Pixels, Render,
    WeakEntity, Window, deferred, div, px,
};
use settings::{DockSide, SettingsStore};
use ui::prelude::*;

use crate::{
    Workspace, AGENT_PANEL_DOCK_KEY,
    dock::{DockPosition, PanelResizeMode},
};

use super::DraggedDock;

use super::MIN_INNER_AGENT_WIDTH;

/// Hit target width for split-layout column gutters (wider than legacy 6px dock handles).
pub const COLUMN_GUTTER_HIT_SIZE: Pixels = px(14.);

/// Minimum rendered height for the bottom dock (terminal / debug).
pub const MIN_BOTTOM_DOCK_HEIGHT: Pixels = px(200.);

/// Column identity in a resolved side layout (screen edge → editor).
pub type ColumnId = SideColumnKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColumnSlot {
    pub id: ColumnId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SideLayout {
    ClassicDock,
    ColumnRow(Vec<ColumnSlot>),
}

pub struct LayoutResolver;

impl LayoutResolver {
    pub fn resolve(workspace: &Workspace, side: DockSide, cx: &App) -> SideLayout {
        let columns: Vec<ColumnSlot> = workspace
            .layout_blueprint(cx)
            .side_columns(side)
            .into_iter()
            .filter(|kind| column_available(workspace, *kind, cx))
            .map(|id| ColumnSlot { id })
            .collect();

        match columns.len() {
            0 => {
                // Nav on the other side (e.g. outer agent dock) — keep classic flex path.
                SideLayout::ClassicDock
            }
            1 => SideLayout::ClassicDock,
            _ => SideLayout::ColumnRow(columns),
        }
    }
}

fn column_available(workspace: &Workspace, kind: SideColumnKind, cx: &App) -> bool {
    match kind {
        SideColumnKind::NavDock => true,
        SideColumnKind::AgentPanel => workspace.agent_panel_handle(cx).is_some(),
        SideColumnKind::ThreadsSidebar => workspace.threads_sidebar_available(cx),
    }
}

/// Drag payload: resize `column_id` on `side`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DraggedColumn {
    pub side: DockSide,
    pub column_id: ColumnId,
}

impl Render for DraggedColumn {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

pub(crate) fn dock_position_for_side(side: DockSide) -> DockPosition {
    match side {
        DockSide::Left => DockPosition::Left,
        DockSide::Right => DockPosition::Right,
    }
}

impl Workspace {
    pub(crate) fn layout_blueprint(&self, cx: &App) -> LayoutBlueprint {
        LayoutBlueprint::from_merged_settings(cx.global::<SettingsStore>().merged_settings())
    }

    pub(crate) fn is_inner_agent(&self, side: DockSide, cx: &App) -> bool {
        matches!(
            self.layout_blueprint(cx).agent_panel_slot(side),
            AgentPanelSlot::Inner
        )
    }

    pub(crate) fn is_inner_threads(&self, side: DockSide, cx: &App) -> bool {
        matches!(
            self.layout_blueprint(cx).threads_sidebar_slot(side),
            ThreadsSidebarSlot::Inner
        ) && self.threads_sidebar_available(cx)
    }

    pub(crate) fn is_outer_threads(&self, side: DockSide, cx: &App) -> bool {
        matches!(
            self.layout_blueprint(cx).threads_sidebar_slot(side),
            ThreadsSidebarSlot::Outer
        ) && self.threads_sidebar_available(cx)
    }

    pub(crate) fn threads_column_on_side(&self, side: DockSide, cx: &App) -> bool {
        self.is_inner_threads(side, cx) || self.is_outer_threads(side, cx)
    }

    pub(crate) fn threads_column_width(&self, side: DockSide, cx: &App) -> Pixels {
        if !self.threads_column_on_side(side, cx) {
            return Pixels::ZERO;
        }
        self.multi_workspace
            .as_ref()
            .and_then(|multi_workspace| multi_workspace.upgrade())
            .map(|multi_workspace| multi_workspace.read(cx).threads_sidebar_width(cx))
            .unwrap_or(Pixels::ZERO)
    }

    pub(crate) fn threads_sidebar_available(&self, cx: &App) -> bool {
        self.multi_workspace
            .as_ref()
            .and_then(|multi_workspace| multi_workspace.upgrade())
            .and_then(|multi_workspace| multi_workspace.read(cx).sidebar().map(|_| ()))
            .is_some()
    }

    pub(crate) fn threads_sidebar_open(&self, cx: &App) -> bool {
        self.multi_workspace
            .as_ref()
            .and_then(|multi_workspace| multi_workspace.upgrade())
            .is_some_and(|multi_workspace| multi_workspace.read(cx).sidebar_open())
    }

    /// A resolved column that renders zero-width (nav dock closed, threads
    /// sidebar toggled off). The column stays in the resolved layout — so the
    /// split-layout state, dock suppression keys, and resize modes never flip
    /// with panel visibility — but the strip skips its width and resize seams.
    pub(crate) fn column_collapsed(&self, side: DockSide, kind: ColumnId, cx: &App) -> bool {
        match kind {
            SideColumnKind::NavDock => {
                let dock = self.dock_at_position(dock_position_for_side(side));
                dock.read(cx).visible_panel().is_none()
            }
            SideColumnKind::ThreadsSidebar => !self.threads_sidebar_open(cx),
            SideColumnKind::AgentPanel => false,
        }
    }

    pub(crate) fn resolved_side_columns(&self, side: DockSide, cx: &App) -> Vec<ColumnSlot> {
        match LayoutResolver::resolve(self, side, cx) {
            SideLayout::ColumnRow(columns) => columns,
            _ => Vec::new(),
        }
    }

    pub(crate) fn side_uses_column_row(&self, side: DockSide, cx: &App) -> bool {
        matches!(
            LayoutResolver::resolve(self, side, cx),
            SideLayout::ColumnRow(_)
        )
    }

    pub(crate) fn is_split_outer_agent(&self, side: DockSide, cx: &App) -> bool {
        matches!(
            self.layout_blueprint(cx).agent_panel_slot(side),
            AgentPanelSlot::Outer
        ) && self.agent_panel_handle(cx).is_some()
            && (self.side_uses_column_row(DockSide::Left, cx)
                || self.side_uses_column_row(DockSide::Right, cx))
    }

    pub(crate) fn split_layout_active(&self, cx: &App) -> bool {
        self.side_uses_column_row(DockSide::Left, cx)
            || self.side_uses_column_row(DockSide::Right, cx)
            || self.is_split_outer_agent(DockSide::Left, cx)
            || self.is_split_outer_agent(DockSide::Right, cx)
    }

    pub(crate) fn classic_dock_uses_editor_gutter(&self, side: DockSide, cx: &App) -> bool {
        self.split_layout_active(cx) && !self.side_uses_column_row(side, cx)
    }

    pub(crate) fn dock_panel_resize_mode(
        &self,
        position: DockPosition,
        cx: &App,
    ) -> PanelResizeMode {
        if position.axis() != Axis::Horizontal {
            return PanelResizeMode::Dock;
        }
        let dock_side = match position {
            DockPosition::Left => DockSide::Left,
            DockPosition::Right => DockSide::Right,
            DockPosition::Bottom => return PanelResizeMode::Dock,
        };
        if self.side_uses_column_row(dock_side, cx) || self.classic_dock_uses_editor_gutter(dock_side, cx)
        {
            PanelResizeMode::FixedPixels
        } else {
            PanelResizeMode::Dock
        }
    }

    pub(crate) fn sync_side_column_dock_handles(&mut self, cx: &mut Context<Self>) {
        for side in [DockSide::Left, DockSide::Right] {
            let hide = self.side_uses_column_row(side, cx)
                || self.classic_dock_uses_editor_gutter(side, cx);
            let dock = self.dock_at_position(dock_position_for_side(side));
            dock.update(cx, |dock, cx| {
                dock.set_hide_builtin_resize_handle(hide, cx);
            });
        }
    }

    pub(crate) fn normalize_side_column_nav_sizes(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        for side in [DockSide::Left, DockSide::Right] {
            if !self.side_uses_column_row(side, cx) {
                continue;
            }
            let dock = self.dock_at_position(dock_position_for_side(side));
            let Some(panel) = dock.read(cx).visible_panel() else {
                continue;
            };
            let size_state = dock
                .read(cx)
                .stored_panel_size_state(panel.as_ref())
                .unwrap_or_default();
            if size_state.flex.is_none() && size_state.size.is_some() {
                continue;
            }
            let width = size_state
                .size
                .or_else(|| self.dock_size(dock.read(cx), window, cx))
                .unwrap_or_else(|| panel.default_size(window, cx));
            dock.update(cx, |dock, cx| {
                dock.resize_displayed_panel_with_mode(
                    Some(width),
                    None,
                    PanelResizeMode::FixedPixels,
                    window,
                    cx,
                );
            });
        }
    }

    fn column_min_width_on_side(
        &self,
        side: DockSide,
        kind: ColumnId,
        window: &Window,
        cx: &App,
    ) -> Option<Pixels> {
        match kind {
            SideColumnKind::NavDock => {
                let dock = self.dock_at_position(dock_position_for_side(side));
                dock.read(cx)
                    .visible_panel()
                    .and_then(|panel| panel.min_size(window, cx))
            }
            SideColumnKind::AgentPanel => self
                .agent_panel_handle(cx)
                .and_then(|panel| panel.min_size(window, cx))
                .or(self.is_split_outer_agent(side, cx).then_some(MIN_INNER_AGENT_WIDTH)),
            SideColumnKind::ThreadsSidebar => Some(px(160.)),
        }
    }

    pub(crate) fn column_width(
        &self,
        side: DockSide,
        kind: ColumnId,
        window: &Window,
        cx: &App,
    ) -> Pixels {
        match kind {
            SideColumnKind::NavDock => self.nav_column_width(side, window, cx),
            SideColumnKind::AgentPanel => {
                if self.is_inner_agent(side, cx) {
                    self.inner_agent_panel_width(side, window, cx)
                } else if self.is_split_outer_agent(side, cx) {
                    self.agent_panel_column_width(window, cx)
                } else {
                    Pixels::ZERO
                }
            }
            SideColumnKind::ThreadsSidebar => self.threads_column_width(side, cx),
        }
    }

    fn nav_column_width(&self, side: DockSide, window: &Window, cx: &App) -> Pixels {
        let dock = self.dock_at_position(dock_position_for_side(side));
        let dock = dock.read(cx);
        if dock.visible_panel().is_none() {
            return Pixels::ZERO;
        }
        dock.stored_visible_panel_size(window, cx)
            .or_else(|| {
                dock.visible_panel().map(|panel| {
                    dock.stored_panel_size_state(panel.as_ref())
                        .and_then(|state| state.size)
                        .unwrap_or_else(|| panel.default_size(window, cx))
                })
            })
            .unwrap_or(px(180.))
    }

    pub(crate) fn outer_threads_width_from_drag(
        &self,
        side: DockSide,
        position_x: Pixels,
        strip_origin_x: Pixels,
        strip_width: Pixels,
        window: &Window,
        cx: &App,
    ) -> Pixels {
        let _ = strip_width;
        // Outer strip layout: seam | threads | seam (reversed on right).
        let leading_seam = COLUMN_GUTTER_HIT_SIZE;
        let raw = match side {
            DockSide::Left => position_x - strip_origin_x - leading_seam,
            DockSide::Right => strip_origin_x + strip_width - leading_seam - position_x,
        };
        self.clamp_threads_column_width(side, raw, window, cx, true)
    }

    fn clamp_threads_column_width(
        &self,
        side: DockSide,
        width: Pixels,
        window: &Window,
        cx: &App,
        outer: bool,
    ) -> Pixels {
        let min = px(160.);
        let max = if outer {
            self.max_outer_threads_width(side, window, cx)
        } else {
            let used_elsewhere = self.horizontal_usage_on_side(side, window, cx)
                - self.threads_column_width(side, cx);
            let opposite = match side {
                DockSide::Left => self.horizontal_usage_on_side(DockSide::Right, window, cx),
                DockSide::Right => self.horizontal_usage_on_side(DockSide::Left, window, cx),
            };
            (self.bounds.size.width - opposite - used_elsewhere
                - LayoutBlueprint::MIN_EDITOR_WIDTH)
                .max(min)
        };
        width.clamp(min, max)
    }

    fn max_outer_threads_width(
        &self,
        side: DockSide,
        window: &Window,
        cx: &App,
    ) -> Pixels {
        let min = px(160.);
        let window_width = window.window_bounds().get_bounds().size.width;
        let opposite = match side {
            DockSide::Left => self.horizontal_usage_on_side(DockSide::Right, window, cx),
            DockSide::Right => self.horizontal_usage_on_side(DockSide::Left, window, cx),
        };
        let same_side_workspace = self.horizontal_usage_on_side(side, window, cx);
        (window_width - opposite - same_side_workspace - LayoutBlueprint::MIN_EDITOR_WIDTH).max(min)
    }

    fn can_resize_column(&self, side: DockSide, column_id: ColumnId, cx: &App) -> bool {
        match column_id {
            SideColumnKind::ThreadsSidebar => self.threads_column_on_side(side, cx),
            SideColumnKind::AgentPanel => {
                self.resolved_side_columns(side, cx)
                    .iter()
                    .any(|slot| slot.id == column_id)
                    || self.is_split_outer_agent(side, cx)
            }
            _ => self
                .resolved_side_columns(side, cx)
                .iter()
                .any(|slot| slot.id == column_id),
        }
    }

    pub(crate) fn apply_column_width(
        &mut self,
        side: DockSide,
        column_id: ColumnId,
        new_width: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_resize_column(side, column_id, cx) {
            return;
        }

        let outer_threads = column_id == SideColumnKind::ThreadsSidebar
            && self.is_outer_threads(side, cx);
        let new_width = if column_id == SideColumnKind::ThreadsSidebar {
            self.clamp_threads_column_width(side, new_width, window, cx, outer_threads)
        } else {
            self.clamp_column_width(side, column_id, new_width, window, cx)
        };

        match column_id {
            SideColumnKind::NavDock => {
                let dock = self.dock_at_position(dock_position_for_side(side));
                dock.update(cx, |dock, cx| {
                    dock.resize_displayed_panel_with_mode(
                        Some(new_width),
                        None,
                        PanelResizeMode::FixedPixels,
                        window,
                        cx,
                    );
                });
            }
            SideColumnKind::AgentPanel => {
                self.resize_inner_agent_panel(side, new_width, window, cx);
            }
            SideColumnKind::ThreadsSidebar => {
                self.set_threads_column_width(side, Some(new_width), cx);
            }
        }
        self.finish_column_resize(column_id, window, cx);
    }

    fn set_threads_column_width(
        &mut self,
        _side: DockSide,
        width: Option<Pixels>,
        cx: &mut Context<Self>,
    ) {
        let Some(multi_workspace) = self
            .multi_workspace
            .as_ref()
            .and_then(|entity| entity.upgrade())
        else {
            return;
        };
        multi_workspace.update(cx, |multi_workspace, cx| {
            if let Some(sidebar) = multi_workspace.sidebar() {
                sidebar.set_width(width, cx);
            }
            multi_workspace.serialize(cx);
        });
    }

    fn finish_column_resize(
        &mut self,
        column_id: ColumnId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.serialize_workspace(window, cx);
        if column_id == SideColumnKind::ThreadsSidebar {
            if let Some(multi_workspace) = self
                .multi_workspace
                .as_ref()
                .and_then(|entity| entity.upgrade())
            {
                multi_workspace.update(cx, |_, cx| cx.notify());
            }
        }
        cx.notify();
    }

    pub(crate) fn reset_column_width(
        &mut self,
        side: DockSide,
        column_id: ColumnId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_resize_column(side, column_id, cx) {
            return;
        }

        match column_id {
            SideColumnKind::NavDock => {
                let dock = match side {
                    DockSide::Left => &self.left_dock,
                    DockSide::Right => &self.right_dock,
                };
                dock.update(cx, |dock, cx| {
                    dock.resize_displayed_panel_with_mode(
                        None,
                        None,
                        PanelResizeMode::FixedPixels,
                        window,
                        cx,
                    );
                });
            }
            SideColumnKind::AgentPanel => self.reset_inner_agent_panel(side, window, cx),
            SideColumnKind::ThreadsSidebar => {
                self.set_threads_column_width(side, None, cx);
            }
        }
        self.finish_column_resize(column_id, window, cx);
    }

    pub(crate) fn column_width_from_drag(
        &self,
        side: DockSide,
        column_id: ColumnId,
        position_x: Pixels,
        strip_bounds: gpui::Bounds<Pixels>,
        window: &Window,
        cx: &App,
    ) -> Pixels {
        if column_id == SideColumnKind::AgentPanel && self.is_split_outer_agent(side, cx) {
            let width = match side {
                DockSide::Left => {
                    position_x - strip_bounds.origin.x - COLUMN_GUTTER_HIT_SIZE
                }
                DockSide::Right => strip_bounds.origin.x + strip_bounds.size.width
                    - position_x
                    - COLUMN_GUTTER_HIT_SIZE,
            };
            return self.clamp_column_width(side, column_id, width, window, cx);
        }

        // Collapsed columns render no width and no seams, so exclude them from
        // the seam/offset math below.
        let columns: Vec<ColumnSlot> = self
            .resolved_side_columns(side, cx)
            .into_iter()
            .filter(|slot| !self.column_collapsed(side, slot.id, cx))
            .collect();
        let Some(column_index) = columns.iter().position(|slot| slot.id == column_id) else {
            return Pixels::ZERO;
        };

        let leading_columns: Pixels = columns
            .iter()
            .take(column_index)
            .map(|slot| self.column_width(side, slot.id, window, cx))
            .sum();
        let leading_seams = column_seams_before_index(column_index);

        let width = match side {
            DockSide::Left => {
                position_x - strip_bounds.origin.x - leading_columns - leading_seams
            }
            DockSide::Right => {
                let trailing_columns: Pixels = columns
                    .iter()
                    .skip(column_index + 1)
                    .map(|slot| self.column_width(side, slot.id, window, cx))
                    .sum();
                let trailing_seams =
                    column_seams_after_index(column_index, columns.len());
                strip_bounds.origin.x + strip_bounds.size.width
                    - position_x
                    - trailing_columns
                    - trailing_seams
            }
        };

        self.clamp_column_width(side, column_id, width, window, cx)
    }

    pub(crate) fn handle_dragged_column_move(
        &mut self,
        drag: DraggedColumn,
        position: gpui::Point<Pixels>,
        strip_bounds: gpui::Bounds<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.previous_dock_drag_coordinates == Some(position) {
            return;
        }
        self.previous_dock_drag_coordinates = Some(position);
        let new_width = if drag.column_id == SideColumnKind::ThreadsSidebar
            && self.is_outer_threads(drag.side, cx)
        {
            self.outer_threads_width_from_drag(
                drag.side,
                position.x,
                strip_bounds.origin.x,
                strip_bounds.size.width,
                window,
                cx,
            )
        } else {
            self.column_width_from_drag(
                drag.side,
                drag.column_id,
                position.x,
                strip_bounds,
                window,
                cx,
            )
        };
        self.resize_column(drag.side, drag.column_id, new_width, window, cx);
    }

    fn clamp_column_width(
        &self,
        side: DockSide,
        column_id: ColumnId,
        width: Pixels,
        window: &Window,
        cx: &App,
    ) -> Pixels {
        let min = match column_id {
            SideColumnKind::NavDock => px(180.),
            SideColumnKind::AgentPanel => MIN_INNER_AGENT_WIDTH,
            SideColumnKind::ThreadsSidebar => px(160.),
        };

        let max = match column_id {
            SideColumnKind::NavDock => self.max_dock_width_on_side(side, window, cx),
            SideColumnKind::AgentPanel => self.max_inner_agent_width_on_side(side, window, cx),
            SideColumnKind::ThreadsSidebar => {
                return self.clamp_threads_column_width(side, width, window, cx, false);
            }
        };

        width.clamp(min, max)
    }

    pub(crate) fn resize_column(
        &mut self,
        side: DockSide,
        column_id: ColumnId,
        new_width: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.apply_column_width(side, column_id, new_width, window, cx);
    }

    pub(crate) fn reset_column(
        &mut self,
        side: DockSide,
        column_id: ColumnId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.reset_column_width(side, column_id, window, cx);
    }

    pub(crate) fn agent_panel_column_width(&self, window: &Window, cx: &App) -> Pixels {
        let Some(dock) = self.agent_panel_dock(cx) else {
            return LayoutBlueprint::DEFAULT_AGENT_WIDTH;
        };
        let Some(panel) = dock.read(cx).panel_for_key(AGENT_PANEL_DOCK_KEY) else {
            return LayoutBlueprint::DEFAULT_AGENT_WIDTH;
        };
        Self::fixed_column_panel_width(
            panel.as_ref(),
            dock.read(cx)
                .stored_panel_size_state(panel.as_ref()),
            window,
            cx,
        )
    }

    fn render_split_outer_agent_side(
        &self,
        side: DockSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let width = self.agent_panel_column_width(window, cx);
        let min_width = self
            .agent_panel_handle(cx)
            .and_then(|panel| panel.min_size(window, cx))
            .unwrap_or(MIN_INNER_AGENT_WIDTH);
        let content = self
            .agent_panel_column(side, window, cx)
            .unwrap_or_else(empty_column);
        div().flex_shrink_0().child(render_outer_agent_column(
            side,
            width,
            min_width,
            content,
            cx.weak_entity(),
            cx,
        ))
    }

    pub(crate) fn render_side_layout(
        &self,
        side: DockSide,
        mut row: gpui::Div,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        match LayoutResolver::resolve(self, side, cx) {
            SideLayout::ClassicDock => {
                if self.split_layout_active(cx)
                    && self.layout_blueprint(cx).agent_panel_slot(side)
                        == AgentPanelSlot::Outer
                    && self.agent_panel_handle(cx).is_some()
                {
                    return row.child(self.render_split_outer_agent_side(side, window, cx));
                }

                let position = dock_position_for_side(side);
                let dock = match side {
                    DockSide::Left => &self.left_dock,
                    DockSide::Right => &self.right_dock,
                };
                if let Some(dock_element) = self.render_dock(position, dock, window, cx) {
                    let show_editor_gutter = self.classic_dock_uses_editor_gutter(side, cx)
                        && (dock.read(cx).visible_panel().is_some()
                            || self.agent_panel_handle(cx).is_some());
                    if show_editor_gutter {
                        let gutter = classic_dock_editor_gutter(side, cx.weak_entity(), cx);
                        row = row.child(classic_dock_shell(dock_element, gutter));
                    } else {
                        row = row.child(dock_element);
                    }
                }
                row
            }
            SideLayout::ColumnRow(columns) => {
                self.append_column_row(side, columns, row, window, cx)
            }
        }
    }

    fn append_column_row(
        &self,
        side: DockSide,
        columns: Vec<ColumnSlot>,
        row: gpui::Div,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        row.child(self.column_row_strip(side, &columns, window, cx))
    }

    /// Fixed-pixel columns with explicit flex seam siblings (screen edge → editor).
    fn column_row_strip(
        &self,
        side: DockSide,
        columns: &[ColumnSlot],
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Div {
        let weak = cx.weak_entity();
        let mut parts: Vec<AnyElement> = Vec::new();

        let mut pushed_leading_seam = false;
        for slot in columns.iter() {
            if self.column_collapsed(side, slot.id, cx) {
                // Keep the column's entity mounted (so toggles/focus still work)
                // but render no width and no resize seams.
                parts.push(collapsed_column_shell(
                    self.render_column(slot.id, side, window, cx),
                ));
                continue;
            }

            if !pushed_leading_seam {
                parts.push(column_seam(side, slot.id, weak.clone(), cx));
                pushed_leading_seam = true;
            }

            let width = self.column_width(side, slot.id, window, cx);
            let min_width = self.column_min_width_on_side(side, slot.id, window, cx);
            parts.push(column_shell(
                width,
                min_width,
                self.render_column(slot.id, side, window, cx),
            ));

            // Seam to the right of this column resizes this column (not the next one).
            parts.push(column_seam(side, slot.id, weak.clone(), cx));
        }

        if side == DockSide::Right {
            parts.reverse();
        }

        let mut strip = bind_column_strip_drag(
            div().h_full().flex_shrink_0().flex().flex_row(),
            side,
            weak,
        );
        for part in parts {
            strip = strip.child(part);
        }
        strip
    }

    fn render_column(
        &self,
        kind: ColumnId,
        side: DockSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match kind {
            SideColumnKind::NavDock => self.render_nav_column(side, window, cx),
            SideColumnKind::AgentPanel => self
                .agent_panel_column(side, window, cx)
                .unwrap_or_else(empty_column),
            SideColumnKind::ThreadsSidebar => self
                .threads_column(side, cx)
                .unwrap_or_else(empty_column),
        }
    }

    fn render_nav_column(
        &self,
        side: DockSide,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> AnyElement {
        let position = dock_position_for_side(side);
        if self.zoomed_position == Some(position) {
            return div().into_any_element();
        }

        let dock = self.dock_at_position(position);
        // Always mount the dock entity (matches `render_dock`) so panel toggles and
        // focus work when the dock is closed. Width is applied on the column shell.
        div()
            .id(format!("nav-column-{side:?}"))
            .size_full()
            .overflow_hidden()
            .child(dock.clone())
            .into_any_element()
    }

    fn agent_panel_column(
        &self,
        _side: DockSide,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let panel = self.agent_panel_handle(cx)?;

        Some(
            div()
                .id("side-column-agent")
                .size_full()
                .overflow_hidden()
                .bg(cx.theme().colors().panel_background)
                .border_l_1()
                .border_color(cx.theme().colors().border)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .size_full()
                        .overflow_hidden()
                        .child(panel.to_any()),
                )
                .into_any_element(),
        )
    }

    fn threads_column(&self, _side: DockSide, cx: &mut Context<Self>) -> Option<AnyElement> {
        let multi_workspace = self
            .multi_workspace
            .as_ref()
            .and_then(|entity| entity.upgrade())?;
        let sidebar = multi_workspace.read(cx).sidebar()?;

        Some(
            div()
                .id("side-column-threads")
                .size_full()
                .overflow_hidden()
                .child(sidebar.to_any())
                .into_any_element(),
        )
    }
}

/// Outer threads column at the MultiWorkspace window edge (same flex seam model as inner columns).
pub(crate) fn render_outer_threads_column(
    side: DockSide,
    width: Pixels,
    content: AnyElement,
    weak: WeakEntity<Workspace>,
    cx: &App,
) -> AnyElement {
    let column_id = SideColumnKind::ThreadsSidebar;
    let mut parts = vec![
        column_seam(side, column_id, weak.clone(), cx),
        column_shell(width, Some(px(160.)), content),
        column_seam(side, column_id, weak.clone(), cx),
    ];
    if side == DockSide::Right {
        parts.reverse();
    }

    let mut strip = bind_column_strip_drag(
        div().h_full().flex_shrink_0().flex().flex_row(),
        side,
        weak.clone(),
    );
    for part in parts {
        strip = strip.child(part);
    }
    strip.into_any_element()
}

/// Outer agent column (PulseBoard split layout): in-flow seam + fixed-width panel.
pub(crate) fn render_outer_agent_column(
    side: DockSide,
    width: Pixels,
    min_width: Pixels,
    content: AnyElement,
    weak: WeakEntity<Workspace>,
    cx: &App,
) -> AnyElement {
    let column_id = SideColumnKind::AgentPanel;
    let mut parts = vec![
        column_seam(side, column_id, weak.clone(), cx),
        column_shell(width, Some(min_width), content),
        column_seam(side, column_id, weak.clone(), cx),
    ];
    if side == DockSide::Right {
        parts.reverse();
    }

    let mut strip = bind_column_strip_drag(
        div().h_full().flex_shrink_0().flex().flex_row(),
        side,
        weak,
    );
    for part in parts {
        strip = strip.child(part);
    }
    strip.into_any_element()
}

/// Seam slots before column `index` in edge→editor strip order (leading edge + inter-column seams).
fn column_seams_before_index(index: usize) -> Pixels {
    COLUMN_GUTTER_HIT_SIZE * (index as f32 + 1.)
}

/// Seam slots after column `index` through the editor-facing edge.
fn column_seams_after_index(index: usize, column_count: usize) -> Pixels {
    COLUMN_GUTTER_HIT_SIZE * (column_count - index) as f32
}

fn bind_column_strip_drag(
    strip: gpui::Div,
    strip_side: DockSide,
    weak: WeakEntity<Workspace>,
) -> gpui::Div {
    strip
        .on_drag_move::<DraggedColumn>({
            let weak = weak.clone();
            move |event, window, cx| {
                let drag = *event.drag(cx);
                if drag.side != strip_side {
                    return;
                }
                let position = event.event.position;
                let strip_bounds = event.bounds;
                weak.update(cx, |workspace, cx| {
                    workspace.handle_dragged_column_move(drag, position, strip_bounds, window, cx);
                })
                .ok();
            }
        })
        .on_drop::<DraggedColumn>(move |_, _, cx| {
            weak.update(cx, |workspace, _| {
                workspace.previous_dock_drag_coordinates = None;
            })
            .ok();
        })
}

fn classic_dock_shell(
    content: impl IntoElement,
    editor_gutter: AnyElement,
) -> impl IntoElement {
    div()
        .relative()
        .h_full()
        .flex_shrink_0()
        .child(content)
        .child(editor_gutter)
}

fn classic_dock_editor_gutter(
    side: DockSide,
    weak: WeakEntity<Workspace>,
    cx: &App,
) -> AnyElement {
    let position = dock_position_for_side(side);
    let border_color = cx.theme().colors().border;

    let handle = div()
        .id(format!("classic-dock-editor-gutter-{side:?}"))
        .on_drag(DraggedDock(position), |payload, _, _, cx| {
            cx.stop_propagation();
            cx.new(|_| payload.clone())
        })
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
        .on_mouse_up(
            MouseButton::Left,
            move |event, window, cx| {
                if event.click_count == 2 {
                    weak.update(cx, |workspace, cx| {
                        if workspace.is_split_outer_agent(side, cx) {
                            workspace.reset_column(side, SideColumnKind::AgentPanel, window, cx);
                        } else {
                            let dock = workspace.dock_at_position(dock_position_for_side(side));
                            dock.update(cx, |dock, cx| {
                                dock.resize_displayed_panel(None, None, window, cx);
                            });
                            workspace.serialize_workspace(window, cx);
                        }
                    })
                    .ok();
                }
                cx.stop_propagation();
            },
        );

    let positioned = match side {
        DockSide::Left => handle
            .absolute()
            .top(px(0.))
            .h_full()
            .w(COLUMN_GUTTER_HIT_SIZE)
            .right(-COLUMN_GUTTER_HIT_SIZE / 2.),
        DockSide::Right => handle
            .absolute()
            .top(px(0.))
            .h_full()
            .w(COLUMN_GUTTER_HIT_SIZE)
            .left(-COLUMN_GUTTER_HIT_SIZE / 2.),
    };

    deferred(
        positioned
            .cursor_col_resize()
            .hover(|style| style.bg(border_color.opacity(0.35)))
            .occlude(),
    )
    .into_any()
}

fn column_shell(
    width: Pixels,
    min_width: Option<Pixels>,
    content: AnyElement,
) -> AnyElement {
    div()
        .h_full()
        .flex_shrink_0()
        .overflow_hidden()
        .when(width > Pixels::ZERO, |this| this.w(width))
        .when_some(min_width, |this, min| this.min_w(min))
        .child(content)
        .into_any_element()
}

fn empty_column() -> AnyElement {
    div().into_any_element()
}

/// Zero-width shell that keeps a collapsed column's entity mounted.
fn collapsed_column_shell(content: AnyElement) -> AnyElement {
    div()
        .w(Pixels::ZERO)
        .h_full()
        .flex_shrink_0()
        .overflow_hidden()
        .child(content)
        .into_any_element()
}

/// In-flow resize seam between columns (or column | editor). Never overlaps siblings.
fn column_seam(
    side: DockSide,
    column_id: ColumnId,
    weak: WeakEntity<Workspace>,
    cx: &App,
) -> AnyElement {
    let drag = DraggedColumn { side, column_id };
    let border_color = cx.theme().colors().border;

    div()
        .id(format!("side-column-seam-{side:?}-{column_id:?}"))
        .w(COLUMN_GUTTER_HIT_SIZE)
        .h_full()
        .flex_shrink_0()
        // Paint the seam as chrome (panel background + centered divider line)
        // instead of a transparent gap that exposes the window background.
        .bg(cx.theme().colors().panel_background)
        .flex()
        .justify_center()
        .child(div().w_px().h_full().bg(border_color))
        .cursor_col_resize()
        .hover(|style| style.bg(border_color.opacity(0.35)))
        .occlude()
        .on_drag(drag, |_, _, _, cx| {
            cx.stop_propagation();
            cx.new(|_| gpui::Empty)
        })
        .on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
        .on_mouse_up(
            MouseButton::Left,
            move |event, window, cx| {
                if event.click_count == 2 {
                    weak.update(cx, |workspace, cx| {
                        workspace.reset_column(side, column_id, window, cx);
                    })
                    .ok();
                }
                cx.stop_propagation();
            },
        )
        .into_any()
}
