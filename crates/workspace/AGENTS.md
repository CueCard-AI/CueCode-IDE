# `workspace` — agent notes

Layout, docks, panes, window/workspace state. Load `ui-ux-gpui` + `rust-quality`.

## Traps

- **Split columns use fixed-pixel `ColumnShell` only.** Never put `render_dock` flex inside a column row — it crushes columns to 0 and breaks gutters. The 1.4k/1.4l regression was exactly this.
- **Blueprint → `LayoutResolver` → `SideLayout` is the single source of truth** for side columns. Don't reintroduce `inner_agent_left/right` / `inner_threads_left/right` flags on `Workspace` — they were removed in 1.4l because three sources conflicted.
- **Gutters resize the editor-facing column** (14px hit, hover line), not the column at the screen edge. `DraggedColumn` is the single resize drag type for split columns.
- **Editor keeps `MIN_EDITOR_WIDTH`.** Window shrink / crowded layouts must clamp, not collapse the editor.
- **Bottom dock:** `MIN_BOTTOM_DOCK_HEIGHT` (200px) is a floor; cap ~45% of center height. Normalize stored heights below min on load (`normalize_bottom_dock_panel_sizes`). `bottom_dock_layout: contained` = terminal only under the editor column.
- **No global `window.refresh()` from layout transitions.** Scope notifications to the affected view; a global refresh busts prepaint caches and risks nested-update panics.

## Verify

```bash
cargo test -p workspace split_column        # layout engine tests
./script/clippy -p workspace
```

## Where to look

`workspace.rs` (`render_center_editor`, `render_dock`, `center_stack`, `resize_bottom_dock`), `dock.rs`, `layout_engine.rs` (`LayoutResolver`, `ColumnShell`, `render_side_layout`, `render_nav_column`), `layout_blueprint.rs` (`side_columns`, `editor_crowding_warning`).
