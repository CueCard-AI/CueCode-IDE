# Build phase 1.4l — Side Column Layout Engine {#phase-1-4l}

> **Invoke:** `Build phase 1.4l` — comprehensive fix after [1.4k partial](./1-4k-side-column-row.md).

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done — PulseBoard QA passed 2026-07-02 via [1.4m](./1-4m-layout-stabilization-recovery.md) |
| **Last verified** | 2026-07-02 (PulseBoard Q1–Q12 manual QA) |
| **Duration** | 2–3 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) |

## Problem {#phase-1-4l-problem}

[1.4k](./1-4k-side-column-row.md) shipped split rows but PulseBoard QA failed:

1. **Nav missing** — `render_dock` flex (`flex_shrink: 1.0`) inside split row crushed nav to 0px when agent/threads had explicit widths.
2. **Nothing resizable** — gutters were flex **siblings** of columns (not inside `.relative()` shells); dock handles hidden without working replacement.
3. **Three conflicting sources** — blueprint `side_columns()`, runtime `inner_*` flags from `MultiWorkspace`, and `render_dock` flex mixed into one row.

## Rule {#phase-1-4l-rule}

> **Blueprint + panel availability → `LayoutResolver` → `SideLayout`.** Split rows use **fixed-pixel `ColumnShell`s only** — never `render_dock` flex inside a column row.

```text
LayoutBlueprint + availability → LayoutResolver → SideLayout
  Empty | ClassicDock (flex, 6px handle) | ColumnRow (FixedPixels, 14px gutters in-shell)
```

## Design {#phase-1-4l-design}

### Types (`layout_engine.rs`)

- `ColumnId` — alias for `SideColumnKind`
- `ColumnSlot { id }` — resolved column in row order (screen edge → editor)
- `SideLayout` — `Empty` | `ClassicDock` | `ColumnRow(Vec<ColumnSlot>)`
- `LayoutResolver::resolve(workspace, side, cx) -> SideLayout`
- `DraggedColumn { side, column_id }` — single resize drag type
- `ColumnShell` — `.relative().w(width).flex_shrink_0()` with **trailing gutter inside** the shell
- `render_side_layout()` — entry for left/right workspace sides
- `render_nav_column()` — nav dock entity at fixed width (**no** `render_dock` flex wrapper)

### Removed

- `inner_agent_left/right`, `inner_threads_left/right` on `Workspace`
- `set_layout_inner_slots()` from `MultiWorkspace` render
- `side_column_row.rs` (logic moved to `layout_engine.rs`)

### Unchanged

- Gutter semantics from 1.4k (14px hit, editor-facing column resize)
- 1.4i clamp math (`split_column_max_*`)
- `PanelResizeMode::FixedPixels` for split columns
- Outer threads sidebar tiling in `MultiWorkspace` (outer slot only)
- Classic / Agentic single-dock flex path

## Tasks {#phase-1-4l-tasks}

| ID | Task | Done |
|----|------|------|
| 1.4l.1 | `LayoutResolver` + `SideLayout` + `ColumnSlot` from blueprint | `[x]` |
| 1.4l.2 | `ColumnShell` + gutter-in-column + `DraggedColumn` | `[x]` |
| 1.4l.3 | `render_side_layout()` + `render_nav_column()` (no flex in split rows) | `[x]` |
| 1.4l.4 | Remove `inner_*` flags; blueprint-only resolver; gut `side_column_row` | `[x]` |
| 1.4l.5 | Layout Studio apply invariants (existing crowding warning) | `[x]` |
| 1.4l.6 | Tests + PulseBoard QA checklist | `[x]` |

## QA {#phase-1-4l-qa}

Fixture: `qa-fixture/pulseboard`

1. Nav + Agent + Threads **left**: three visible columns; nav tree present; gutters resize correct column
2. Agent + Nav **right**: agent inner column resizes; nav visible on right
3. Classic / Agentic presets: single dock flex unchanged
4. Window shrink: columns clamp; editor keeps `MIN_EDITOR_WIDTH`
5. Double-click gutter resets column default

> **2026-07-02:** PulseBoard manual QA **passed** via [1.4m §4 QA](./1-4m-layout-stabilization-recovery.md#phase-1-4m-4).

## Changelog {#phase-1-4l-changelog}

| Date | Change |
|------|--------|
| 2026-07-02 | Closed — PulseBoard Q1–Q12 passed via [1.4m](./1-4m-layout-stabilization-recovery.md) |
| 2026-07-01 | Reopened — PulseBoard QA failed; recovery tracked in [1.4m](./1-4m-layout-stabilization-recovery.md) |
| 2026-06-19 | Layout engine replaces broken 1.4k split-row render path |
