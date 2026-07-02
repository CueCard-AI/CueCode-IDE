# Build phase 1.4i — Split-column resize UX {#phase-1-4i}

> **Invoke:** `Build phase 1.4i` — polish pass after [1.4h Agent / Nav column split](./1-4h-agent-nav-column-split.md).

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done |
| **Last verified** | 2026-06-19 |
| **Duration** | 1 day |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) |

## Problem {#phase-1-4i-problem}

[1.4h](./1-4h-agent-nav-column-split.md) made Agent + Nav visible side-by-side, but resize UX was broken in practice:

| Symptom | Cause |
|---------|-------|
| Nav grows **over** agent | Right-dock clamp ignored inner agent/threads on the same side |
| “Can't resize agent” | Editor↔agent handle hard to hit; agent↔nav border only resizes nav |
| Nav text breaks when narrow | Project tree uses horizontal overflow, not ellipsis |
| No sensible limits | `ProjectPanel` had no `min_size`; no editor floor in clamp |

## UX principles {#phase-1-4i-principles}

1. **Tiled, never stacked** — columns share space; nothing paints over a neighbor.
2. **One handle per gutter** — every column border is draggable (`col-resize`).
3. **Resize the column you touch** — editor↔agent adjusts agent; agent↔nav adjusts nav.
4. **Hard mins** — editor ≥ 400px, agent ≥ 300px, nav ≥ 180px.
5. **Content adapts** — nav filenames ellipsis when narrow.
6. **Double-click reset** — each gutter resets that column to default width.

## Layout model {#phase-1-4i-layout}

### Right-side split (Agent Right col + Nav Right)

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│                              WORKSPACE ROW (flex, overflow hidden)           │
│  ┌──────────────────────────┐ ║ ┌─────────────┐ ║ ┌──────────────┐        │
│  │        EDITOR            │H1│    AGENT     │H2│  Nav (dock)  │        │
│  │      (flex: 1)           │ ║ │ inner strip │ ║ │ outer dock   │        │
│  └──────────────────────────┘ ║ └─────────────┘ ║ └──────────────┘        │
└─────────────────────────────────────────────────────────────────────────────┘
         screen left                                              screen right
```

| Gutter | Resizes | Double-click resets |
|--------|---------|---------------------|
| **H1** Editor ↔ Agent | Agent width | Agent default (640px) |
| **H2** Agent ↔ Nav | Nav width | Nav default (240px) |

With threads inner on the same side: `[Editor] | [Threads inner] | [Agent inner] | [Nav outer]`.

## Constraint math {#phase-1-4i-math}

For workspace width `W`:

```text
max_dock(side) = W − usage(opposite_side) − inner_strips(same_side) − MIN_EDITOR_WIDTH
max_agent(side) = W − usage(opposite_side) − dock(same_side) − threads(same_side) − MIN_EDITOR_WIDTH
```

Constants: `MIN_EDITOR_WIDTH = 400` (`LayoutBlueprint`), agent min `300`, nav min `180`.

Editor is the shock absorber (`flex: 1`); fixed columns clamp before overlap.

## Depends / blocks {#phase-1-4i-deps}

| | Phase |
|---|------|
| **Depends on** | 1.4h (inner agent strip + dock suppress) |
| **Blocks** | Honest Custom / Plan presets with Agent + Nav same side |

## Out of scope {#phase-1-4i-out-of-scope}

| Item | Notes |
|------|-------|
| Agent↔nav gutter resizing agent | Standard IDE: that gutter resizes nav |
| Layout Studio live resize | Apply-only |
| Plan pinned column | 1.5x |

---

## Tasks {#phase-1-4i-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4i.1 | Split-aware clamp helpers + fix `resize_left/right_dock` | `workspace.rs` | `[x]` |
| 1.4i.2 | Clamp inner agent drag + bounds-change split layout clamp | `workspace.rs` | `[x]` |
| 1.4i.3 | `ProjectPanel::min_size` (180px) | `project_panel.rs` | `[x]` |
| 1.4i.4 | Nav label ellipsis (`truncate`) | `project_panel.rs` | `[x]` |
| 1.4i.5 | Unit tests for clamp math + manual QA checklist | `workspace.rs`, this doc | `[x]` |

---

## QA {#phase-1-4i-qa}

Fixture: `qa-fixture/pulseboard` · Custom: Agent **Right col**, Nav **Right**, Apply Layout.

1. Drag nav to max — agent never covered by nav.
2. **H1** editor↔agent — only agent width changes.
3. **H2** agent↔nav — only nav width changes.
4. Nav max drag stops when editor ≈ 400px wide.
5. Agent min ~300px; nav min ~180px with ellipsis labels.
6. Double-click H1 / H2 resets agent / nav defaults.
7. Reload — widths persist.
8. Mirror on left side; Classic preset unchanged.

---

## Changelog {#phase-1-4i-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial spec + split-aware clamp, nav min/truncate, agent drag max |
| 2026-06-19 | Agent inner drag still broken with `flexible: true` → fixed in [1.4j](./1-4j-inner-column-fixed-pixel-sizing.md) |
| 2026-06-19 | Three ad-hoc resize strips in crowded layouts → [1.4k side column row](./1-4k-side-column-row.md) |
