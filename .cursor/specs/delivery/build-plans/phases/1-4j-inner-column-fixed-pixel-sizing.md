# Build phase 1.4j — Inner column fixed-pixel sizing {#phase-1-4j}

> **Invoke:** `Build phase 1.4j` — fundamental fix after [1.4i split-column resize UX](./1-4i-split-column-resize-ux.md).

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done |
| **Last verified** | 2026-06-19 |
| **Duration** | 0.5–1 day |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) |

## Problem {#phase-1-4j-problem}

Agent panel defaults to **`agent.flexible: true`**. Dock resize writes **flex**; inner strip renders **pixel width** from `size_state.size`.

Inner agent drag calls `resize_panel_by_key(size, flex=None)` → `resize_panel_entry` **discards pixels** when flexible → UI never updates.

```text
OUTER DOCK                    INNER STRIP (1.4h split)
render_dock → flex OR size    agent_panel_strip → w(size) only
resize → flex path            resize → flex path (BUG: ignores size)
```

## Rule {#phase-1-4j-rule}

> **Panels rendered outside the dock wrapper use pixel sizing only.**

Flexible sizing remains a **dock-row** feature (agent shares flex budget with editor). Inner strips are **fixed columns** in a tiled row — same model as threads sidebar.

## Design {#phase-1-4j-design}

### `PanelResizeMode`

```rust
pub enum PanelResizeMode {
    /// Respect `has_flexible_size()` (existing dock behavior).
    Dock,
    /// Inner / split column: write `size`, clear `flex`.
    FixedPixels,
}
```

`resize_panel_by_key(..., mode)` routes through `resize_panel_entry`.

### Width resolver (inner strip)

```text
width = size_state.size ?? default_size(panel)
```

Never reads flex.

### Normalize on inner slot

When `AgentPanelSlot::Inner` becomes active (render sync or Apply Layout):

1. If agent size state is flex-only → convert effective width (`dock_size` or default) to pixels.
2. Clear `flex`; persist pixel `size`.

### Unchanged

- Outer dock agent resize (flex path) when nav not on same side.
- 1.4i clamp math (`split_column_max_*`).

## Depends / blocks {#phase-1-4j-deps}

| | Phase |
|---|------|
| **Depends on** | 1.4h (inner strip), 1.4i (clamp) |
| **Blocks** | Reliable Custom Agent+Nav-right layouts |

---

## Tasks {#phase-1-4j-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4j.1 | `PanelResizeMode` + `resize_panel_entry` / `resize_panel_by_key` | `dock.rs` | `[x]` |
| 1.4j.2 | Inner agent resize/reset uses `FixedPixels`; shared width resolver | `workspace.rs` | `[x]` |
| 1.4j.3 | Normalize flex→pixels when inner slot active | `workspace.rs` | `[x]` |
| 1.4j.4 | Unit test: `FixedPixels` writes size when panel is flexible | `dock.rs` or `workspace.rs` | `[x]` |
| 1.4j.5 | QA: `flexible=true` + Agent Right + Nav Right → agent drag works | this doc | `[x]` |

---

## QA {#phase-1-4j-qa}

Fixture: `qa-fixture/pulseboard` · Custom: Agent **Right col**, Nav **Right**, Apply Layout.

1. With default `agent.flexible: true`, drag **editor ↔ agent** → agent width changes visibly.
2. Nav width unchanged during agent drag; agent unchanged during nav drag.
3. Reload → agent pixel width persists.
4. Classic / Agentic (outer dock) → flex dock resize still works.

---

## Changelog {#phase-1-4j-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial spec + `PanelResizeMode::FixedPixels` + inner normalize |
| 2026-06-19 | Crowded multi-column resize → [1.4k side column row](./1-4k-side-column-row.md) |
