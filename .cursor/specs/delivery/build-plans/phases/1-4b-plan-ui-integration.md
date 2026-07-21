# Build phase 1.4b — Plan UI integration (P-H1b) {#phase-1-4b}

> **Invoke:** `Build phase 1.4b` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Complete — Plan tab, detached window, implement collapse |
| **Last verified** | 2026-06-23 |
| **Duration** | 4–6 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [16-planning-hub §P-H1b](../../design/16-planning-hub.md#delivery-phases) |
| **QA script** | QA-P1 (extended) |

## Deliverable {#phase-1-4b-deliverable}

Refactor **Planning Hub modal** into host-agnostic **`PlanningHubView`** + shared **`PlanStore`**. Ship **Plan tab** in agent panel (default home) and **detached Plan window**. Demote modal to Organize empty state + quick picker only.

## Depends / blocks {#phase-1-4b-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.4 (manifest + Implement ticket session) |
| **Blocks** | 1.5 (`@phase` UX polish), 1.6 (Organize board in Plan chrome) |

## Out of scope {#phase-1-4b-out-of-scope}

- `refs[]` schema + Implement bundle (1.4)
- `@phase` mention type (1.5)
- Organize kanban columns (1.6)
- Left-dock Plan panel (settings later)

---

## Tasks {#phase-1-4b-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4b.0 | Build track status chrome (done green outline, in-progress border) | `crates/agent_ui/src/planning_hub.rs` | `[x]` |
| 1.4b.0b | Implement → **Write** profile auto-select; stub → `verify-all.sh` | `plan_session.rs`, `cuecode_plans/resolve.rs` | `[x]` |
| 1.4b.1 | Extract `PlanningHubView` from modal; shared `PlanStore` entity keyed by workspace | `crates/agent_ui/src/planning_hub.rs`, `plan_store.rs` | `[x]` |
| 1.4b.2 | **Plan tab** — `BaseView::Plan` in `AgentPanel`; tab chrome `[Threads] [Plan] [Terminal]` | `crates/agent_ui/src/agent_panel.rs` | `[x]` |
| 1.4b.3 | Palette: **CueCode: Focus Plan** → focus Plan tab (replace modal as default entry) | `crates/cuecode_actions/` | `[x]` |
| 1.4b.4 | Pin chip → focus Plan tab + select ticket (not modal) | `crates/agent_ui/src/spec_pin_chip.rs` | `[x]` |
| 1.4b.5 | **Open in New Window** — detached GPUI window, same `PlanStore`, selection sync | `crates/agent_ui/src/planning_hub.rs`, `crates/cuecode/` | `[x]` |
| 1.4b.6 | **Dock back** closes detached window; focuses Plan tab in main workspace | `crates/agent_ui/src/planning_hub.rs` | `[x]` |
| 1.4b.7 | Implement collapse: Plan → strip + thread composer expand (see design wireframe) | `crates/agent_ui/src/agent_panel.rs`, `plan_store.rs`, `plan_session.rs` | `[x]` |
| 1.4b.8 | Build track UI from manifest in Plan tab (depends 1.4.3) | `crates/agent_ui/src/planning_hub.rs` | `[x]` |
| 1.4b.9 | Demote modal: only Organize empty state + optional quick picker | `crates/agent_ui/src/planning_hub.rs` | `[x]` |

---

## Implementation notes {#phase-1-4b-impl}

- Design wireframes: [16 §plan-ui](../../design/16-planning-hub.md#plan-ui).
- Reuse modal list/preview code inside `PlanningHubView::render`.
- `PlanStore` holds: selected ticket id, active tab, search filter, detached window handle.
- Detached window: minimal shell (title + `PlanningHubView`); subscribe to `PlanStore` for sync.
- Product copy: **Plan** in UI; keep `OpenPlanningHub` action alias until keymap migration.

---

## Verify {#phase-1-4b-verify}

```bash
cd apps/CueCode-IDE
cargo build -p agent_ui -p cuecode
# Manual: Agent panel → Plan tab → Build track + preview
# Manual: Open in New Window → second monitor; select ticket syncs to main
# Manual: Implement → main thread focused; Plan strip visible; detached stays on ticket
# Manual: Pin chip → Plan tab, ticket selected (no modal)
# Manual: Palette Focus Plan → Plan tab (not modal)
```

---

## Exit criteria {#phase-1-4b-exit}

- [x] [16-planning-hub E5, E8](../../design/16-planning-hub.md#exit-criteria)
- [x] Plan tab is default planning home; modal not used for daily browse
- [x] Detached window usable on second monitor for dogfood

---

## QA {#phase-1-4b-qa}

1. Plan tab matches [16 §plan-ui-agent-tab](../../design/16-planning-hub.md#plan-ui-agent-tab) layout (list + preview + footer actions).
2. Detached window matches [16 §plan-ui-detached](../../design/16-planning-hub.md#plan-ui-detached).
3. No agent chat bleed-through; `elevation_3` shell on all Plan hosts.
4. Open in editor does not dismiss Plan surface.

---

## Deep specs {#phase-1-4b-specs}

| Topic | Doc |
|-------|-----|
| Plan UI wireframes | [16 §plan-ui](../../design/16-planning-hub.md#plan-ui) |
| E2E journey | [16 §e2e-journey](../../design/16-planning-hub.md#e2e-journey) |
| Copy deck | [09 §plan-surface](../../design/09-ui-ux-spec.md#plan-surface) |

---

## Changelog {#phase-1-4b-changelog}

| Date | Change |
|------|--------|
| 2026-06-23 | 1.4b.7: Implement collapse strip + composer expand after ticket session |
| 2026-06-23 | 1.4b.1–6, 8–9: Plan tab, FocusPlan, PlanStore, detached window, modal demoted |
| 2026-06-23 | 1.4b.0/0b: green done outline, Implement→Write, stub verify-all |
| 2026-06-22 | Initial sub-phase — Agent Linear UI integration |
