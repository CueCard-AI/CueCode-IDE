# Build phase 1.4k — Side column row {#phase-1-4k}

> **Invoke:** `Build phase 1.4k` — unified column + gutter model after [1.4j](./1-4j-inner-column-fixed-pixel-sizing.md).

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done — superseded recovery in [1.4l](./1-4l-side-column-layout-engine.md) / [1.4m](./1-4m-layout-stabilization-recovery.md) |
| **Last verified** | 2026-07-02 (PulseBoard Q1–Q12 via 1.4m) |
| **Duration** | 2–3 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) |

## Problem {#phase-1-4k-problem}

Split layouts (Nav | Agent | Threads | Editor) used **three ad-hoc strips**, **three drag types**, and **6px handles**. Gutters resized the wrong column; crowded layouts were unusable.

## Rule {#phase-1-4k-rule}

> Each side of the editor is an ordered list of **fixed-pixel columns** (screen edge → editor). **Uniform gutters** (14px hit, hover line) resize the column at the gutter's **editor-facing** index.

### Gutter semantics (left side, columns `[Nav, Agent, Threads]`)

| Gutter | Resizes |
|--------|---------|
| Screen edge (leading, col 0) | Nav |
| Nav \| Agent | Agent |
| Agent \| Threads | Threads |
| Threads \| Editor | Threads |

Right side: columns ordered screen→editor `[Nav, Threads?, Agent?]`; flex row renders editor→screen.

## Design {#phase-1-4k-design}

- `SideColumnKind` + `LayoutBlueprint::side_columns(side)` in `layout_blueprint.rs`
- `side_column_row.rs`: `DraggedSideColumn`, `COLUMN_GUTTER_HIT_SIZE` (14px), gutter UI, `resize_side_column()`
- When `side_columns.len() > 1`: render `SideColumnRow`, hide dock builtin handle, `PanelResizeMode::FixedPixels`
- Single `on_drag_move(DraggedSideColumn)` on workspace
- `LayoutBlueprint::editor_crowding_warning()` for Layout Studio apply toast

## Tasks {#phase-1-4k-tasks}

| ID | Task | Done |
|----|------|------|
| 1.4k.1 | `SideColumnKind` + `side_columns()` | `[x]` |
| 1.4k.2 | `ColumnGutter` + `DraggedSideColumn` | `[x]` |
| 1.4k.3 | `SideColumnRow` render + `resize_side_column()` | `[x]` |
| 1.4k.4 | Migrate workspace row; remove `DraggedInnerAgent` | `[x]` |
| 1.4k.5 | Layout Studio crowding warning | `[x]` |
| 1.4k.6 | Tests + QA checklist | `[x]` PulseBoard Q1–Q12 via [1.4m](./1-4m-layout-stabilization-recovery.md) |

## Follow-up {#phase-1-4k-follow-up}

Split-row render bugs (nav crushed, gutters non-functional) fixed in [1.4l — Side Column Layout Engine](./1-4l-side-column-layout-engine.md).

## QA {#phase-1-4k-qa}

Fixture: `qa-fixture/pulseboard`

1. Nav + Agent + Threads **left**: three gutters; nav\|agent resizes **Agent**; no overlap
2. Agent + Nav **right**: agent drag with `flexible: true`
3. Classic / Agentic: single dock flex unchanged
4. 3+ columns same side → crowding warning on Apply

## Changelog {#phase-1-4k-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial SideColumnRow + unified gutters |
| 2026-06-19 | Reopened partial — nav/gutter bugs → 1.4l layout engine |
