# Build phase 1.4h — Agent / Nav column split at runtime {#phase-1-4h}

> **Invoke:** `Build phase 1.4h` — companion to [1.4f Layout Studio](./1-4f-layout-studio.md) (threads inner strip = 1.4g pattern in changelog).

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done |
| **Last verified** | 2026-06-19 |
| **Duration** | 1–2 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) |

## Problem {#phase-1-4h-problem}

Layout Studio schematic shows **Agent** (`Right col` / `Left col`) and **Nav** (`Right` / `Left` edge) as **adjacent columns** when both share a side:

```
[Editor] | [Agent column] | [Nav at screen edge]
```

Runtime mapped both into the **same workspace side dock**. Zed docks show **one active panel at a time**, so Apply + reveal Nav covered Agent entirely.

After the first runtime pass, both columns were visible but **resize was broken**: dock drag resized the active panel (Agent) while the UI displayed the visible panel (Nav).

## Deliverable {#phase-1-4h-deliverable}

When blueprint has Agent column + Nav on the **same** side:

| Block | Runtime slot |
|-------|----------------|
| **Nav** (Project / Outline / Git) | Outer — existing `left_dock` / `right_dock` at screen edge |
| **Agent** | Inner — injected between editor and outer dock (same mechanism as threads inner strip in 1.4g) |

Both visible side-by-side after **Apply Layout**, with **independent horizontal resize** for each column.

## Depends / blocks {#phase-1-4h-deps}

| | Phase |
|---|------|
| **Depends on** | 1.4f (Layout Studio), 1.4g (threads inner strip pattern) |
| **Blocks** | Honest Agent+Nav-right presets in Plan / Custom layouts |

## Out of scope {#phase-1-4h-out-of-scope}

| Item | Notes |
|------|-------|
| Plan column + Agent same side | Still invalid (`PlanAgentSameColumn`) |
| Three-way inner strip ordering (Threads + Agent + Nav same side) | Best-effort order: dock → threads inner → agent inner → editor |
| Split bottom dock | Agent `Bottom` unchanged |
| Live Layout Studio focus slider drag in main window | Apply-only; separate from dock resize |

---

## Design {#phase-1-4h-design}

### Blueprint API

```rust
pub enum AgentPanelSlot { Absent, Outer, Inner }

impl LayoutBlueprint {
    pub fn agent_panel_slot(&self, side: DockSide) -> AgentPanelSlot;
    pub fn agent_and_nav_share_side(&self) -> bool;
}
```

- **Inner** when `AgentHost::Column { side }` and `NavPlacement` matches `side`.
- **Outer** when agent on side but nav hidden or on opposite side.
- Nav is always **outer** (dock) when sharing; Agent is always **inner**.

### Workspace render

1. `inner_agent_left` / `inner_agent_right` flags on `Workspace`.
2. `append_inner_*_agent_strip` injects Agent panel element between dock and center.
3. `Dock::display_suppressed_panel_keys` hides Agent from dock tab stack so dock shows Nav while Agent renders in inner strip.

### Split-aware resize {#phase-1-4h-resize}

> **Superseded by [1.4i](./1-4i-split-column-resize-ux.md)** for clamp math, nav min/ellipsis, and overlap fix.

| Edge | Resizes | Mechanism |
|------|---------|-----------|
| Editor ↔ Agent | Agent (inner strip) | `DraggedInnerAgent` → `resize_panel_by_key(agent_panel)` |
| Agent ↔ Nav | Nav (outer dock) | Standard dock handle → `resize_displayed_panel()` (visible panel, not active) |
| Double-click handle | Resets that column to default width | Agent inner or nav dock |

### Settings / Apply

- `write_settings`: **skip** `reconcile_side_panels_for_agent_dock` when `agent_and_nav_share_side()`.
- `apply_blueprint_to_workspace`: reveal Nav panels; Agent focus does not break nav resize (displayed-panel resize path).

---

## Tasks {#phase-1-4h-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4h.1 | `AgentPanelSlot` + `agent_panel_slot` + share-side helper | `layout_blueprint.rs` | `[x]` |
| 1.4h.2 | Dock suppress + inner agent strip render | `dock.rs`, `workspace.rs` | `[x]` |
| 1.4h.3 | Sync slots from blueprint each frame | `multi_workspace.rs`, `workspace.rs` | `[x]` |
| 1.4h.4 | Apply + write_settings reconcile skip | `layout_studio_apply.rs`, `layout_blueprint.rs` | `[x]` |
| 1.4h.5 | Unit tests (blueprint slot + settings round-trip) | `layout_blueprint.rs` | `[x]` |
| 1.4h.6 | Split-aware resize: `resize_displayed_panel`, layout math, inner agent drag | `dock.rs`, `workspace.rs`, `multi_workspace.rs` | `[x]` |

---

## QA {#phase-1-4h-qa}

Fixture: `qa-fixture/pulseboard`

1. **Arrange Workspace** → Custom: Threads left, Agent **Right col**, Nav **Right**.
2. **Apply Layout** → Project tree at **right screen edge**; Agent chat **between editor and project tree**; both visible.
3. Drag **editor ↔ agent** border (`col-resize` cursor) → agent width changes; nav unchanged.
4. Drag **agent ↔ nav** border → nav (project tree) width changes; agent unchanged.
5. Double-click either resize handle → that column resets to default width.
6. Reload app → widths persist.
7. Repeat with Agent **Left col** + Nav **Left**.
8. **Classic** / **Agentic** (opposite sides) → unchanged single-dock resize behavior.

---

## Changelog {#phase-1-4h-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial spec + runtime inner agent strip + dock suppress + reconcile skip |
| 2026-06-19 | 1.4h.6: split-aware resize — `resize_displayed_panel`, `horizontal_usage_on_side`, inner agent drag + double-click reset |
| 2026-06-19 | Follow-on resize overlap + limits → [1.4i split-column resize UX](./1-4i-split-column-resize-ux.md) |
| 2026-06-19 | Inner agent drag with `flexible: true` → [1.4j fixed-pixel inner columns](./1-4j-inner-column-fixed-pixel-sizing.md) |
