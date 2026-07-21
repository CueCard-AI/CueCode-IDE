# Build phase 1.4m — Layout stabilization recovery {#phase-1-4m}

> **Invoke:** `Build phase 1.4m` — open **this file only**; sub-phases **0 → 4 completed 2026-07-02**; [1.5](./1-5-spec-roots-pin-modes.md) unblocked.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done — PulseBoard Q1–Q12 passed 2026-07-02 |
| **Last verified** | 2026-07-02 (PulseBoard manual QA + `cargo test -p workspace split_column`) |
| **Duration** | 2–4 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [07 §phase-1](../07-implementation-roadmap.md#phase-1-4m-layout-stabilization) |
| **QA fixture** | `qa-fixture/pulseboard` (`cargo run -p cuecode`) |
| **Layout preset** | PulseBoard / NavFirst — `[Nav \| Threads] \| [Editor + Terminal] \| [Agent]` |

## Deliverable {#phase-1-4m-deliverable}

PulseBoard layout is **stable and usable**: center editor shows open files, bottom terminal docks at sane height without killing the editor, agent panel shows thread messages with composer pinned at bottom, all column gutters resize the correct column, Plan ↔ Threads tab switch does not corrupt layout.

## Depends / blocks {#phase-1-4m-deps}

| | Phase |
|---|-------|
| **Depends on** | [1.4l](./1-4l-side-column-layout-engine.md) (engine + QA complete 2026-07-02) |
| **Blocks** | ~~[1.5 spec roots](./1-5-spec-roots-pin-modes.md)~~ — **unblocked 2026-07-02** |
| **Reopens** | [1.4l QA](./1-4l-side-column-layout-engine.md#phase-1-4l-qa) — **closed 2026-07-02** (Q1–Q12 pass) |

## Why this phase exists {#phase-1-4m-why}

[1.4l](./1-4l-side-column-layout-engine.md) shipped `LayoutResolver` + `ColumnShell` + `render_side_layout` (cargo check + unit tests pass). **PulseBoard manual QA failed** after follow-up patches (2026-07-01):

| Symptom | Severity | User impact |
|---------|----------|-------------|
| Center editor **blank** (gray band, no tabs/content) while nav selection works | **P0** | Cannot edit files |
| Agent panel title OK but **empty thread body** / composer wrong | **P0** | Cannot use agent |
| Bottom terminal **sliver → clamp** partially fixed; now competes with editor vertically | **P1** | Terminal OK but editor regressed |
| Agent ↔ editor resize invisible / broken | **P2** | Cannot resize agent column |
| Editor diff gutter **paint bleed** at agent seam | **P2** | Visual corruption |

**Root cause:** incremental patches stacked **four layout systems** without one flex contract:

```text
Left ColumnRow     → column_seam + DraggedColumn + column_shell (OK)
Center stack       → flex_col editor + bottom dock + clip wrapper (BROKEN)
Right outer agent  → classic_dock hybrid + hidden agent_panel_dock width (FRAGILE)
Agent UI           → many flex wrappers + window.refresh() on set_base_view (FRAGILE)
```

## Architecture target {#phase-1-4m-target}

One rule (from [1.4l §rule](./1-4l-side-column-layout-engine.md#phase-1-4l-rule)):

> Blueprint → `LayoutResolver` → fixed-pixel columns. **Never** `render_dock` flex inside a column row.

### Target tree (PulseBoard)

```text
MultiWorkspace row
├── Left strip (ColumnRow: Nav, Threads)
│   seam | nav_shell | seam | threads_shell | seam
├── Center stack (v_flex flex_col flex_1 min_h_0)
│   ├── editor_slot (flex_1 min_h_0 flex_shrink_1 overflow_hidden size_full)
│   │   └── PaneGroup → Editor
│   └── bottom_dock_slot (flex_shrink_0 h=clamped)
└── Right strip (outer agent)
    seam | agent_shell | seam   ← all DraggedColumn, no DraggedDock overlap
```

### Key files (edit map)

| Area | Primary file(s) |
|------|-----------------|
| Side columns + seams | `crates/workspace/src/layout_engine.rs` |
| Center + bottom dock | `crates/workspace/src/workspace.rs` (`render_center_editor`, `render_dock`, `center_stack`, `normalize_bottom_dock_panel_sizes`) |
| Agent panel mount | `crates/agent_ui/src/agent_panel.rs` |
| Thread scroll + composer | `crates/agent_ui/src/conversation_view/thread_view.rs` |
| Conversation wrapper | `crates/agent_ui/src/conversation_view.rs` |
| Plan surface styling | `crates/agent_ui/src/planning_hub.rs` |

---

## Sub-phase 0 — Stop the bleeding (P0 unblock) {#phase-1-4m-0}

**Goal:** Files and thread messages visible again. **No new architecture** — smallest diffs only.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| **1.4m.0.1** | Center pane clip wrapper: add `.size_full()` (or `.h_full().w_full()`) to the div wrapping `self.center.render(...)` | `workspace.rs` `render_center_editor` | `[x]` |
| **1.4m.0.2** | `center_stack`: wrap `render_center_editor()` in explicit `div().flex_1().min_h_0().flex_shrink_1().overflow_hidden().h_full()` before bottom dock sibling | `workspace.rs` ~`center_stack` closure | `[x]` |
| **1.4m.0.3** | Remove global `window.refresh()` from `AgentPanel::set_base_view` (causes prepaint cache bust + nested update risk); scope refresh to Plan↔Threads only if still needed | `agent_panel.rs` | `[x]` |
| **1.4m.0.4** | Verify thread list: when sidebar thread selected and title shown, `list_state.item_count() > 0` → entries visible; if count 0, fix activation not layout | `thread_view.rs`, `agent_panel.rs` | `[x]` |

**Verify 0:**

```bash
cd apps/CueCode-IDE
CARGO_TARGET_DIR=$PWD/target cargo check -p workspace -p agent_ui
cargo run -p cuecode   # open qa-fixture/pulseboard
```

**Exit 0:**

- [x] Click file in nav → editor tab + content visible (not blank gray)
- [x] Select thread in Threads column → agent panel shows message history (if thread has messages)
- [x] App starts without `cannot update Workspace while it is already being updated` panic

---

## Sub-phase 1 — Center column contract {#phase-1-4m-1}

**Goal:** Editor + bottom terminal coexist predictably.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| **1.4m.1.1** | Extract `render_center_stack(with_bottom_dock)` helper with documented flex tree (see §target) | `workspace.rs` | `[x]` |
| **1.4m.1.2** | Single clip boundary at `editor_slot` — remove redundant inner clip in `render_center_editor` if duplicated | `workspace.rs` | `[x]` |
| **1.4m.1.3** | Bottom dock height: `MIN_BOTTOM_DOCK_HEIGHT` (200px) as **floor** only; default 320px; **cap** at ~45% of center stack height so editor never collapses to 0 | `layout_engine.rs`, `workspace.rs` `render_dock`, `normalize_bottom_dock_panel_sizes` | `[x]` |
| **1.4m.1.4** | On workspace load, normalize stored bottom dock sizes **below** min (not only missing/zero) | `normalize_bottom_dock_panel_sizes` | `[x]` |
| **1.4m.1.5** | `resize_bottom_dock`: clamp min + max vs center bounds | `workspace.rs` | `[x]` |

**Exit 1:**

- [x] Status bar `>_` opens terminal ≥200px, editor still shows open file above
- [x] Drag bottom dock divider — editor shrinks/grows, content stays visible
- [x] `bottom_dock_layout: contained` (default) — terminal only under editor column

---

## Sub-phase 2 — Agent column + thread UI contract {#phase-1-4m-2}

**Goal:** Agent panel body stable across Threads / Plan / Terminal tabs.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| **1.4m.2.1** | Agent panel body: one `flex_1 min_h_0 overflow_hidden` child keyed by surface id (`agent-surface-threads` / `-plan` / `-terminal`) | `agent_panel.rs` | `[x]` |
| **1.4m.2.2** | ThreadView two-zone layout: `[flex_1 scroll]` (messages + callouts **inside** scroll) + `[flex_shrink_0 composer]` — callouts must not sit between list and composer as flex siblings | `thread_view.rs` | `[x]` |
| **1.4m.2.3** | ConversationView: one `flex_1 min_h_0` wrapper around active thread child | `conversation_view.rs` | `[x]` |
| **1.4m.2.4** | Plan tab in panel: **no** `elevation_3` (panel host uses `panel_background` only); detached window keeps elevation | `planning_hub.rs` | `[x]` |
| **1.4m.2.5** | Remove extra flex wrappers on `panel.to_any()` unless measured necessary | `layout_engine.rs` `agent_panel_column` | `[x]` |
| **1.4m.2.6** | Re-verify agent panel WARNING checklist in `AgentPanel::render` comment (composer expand, font size, scroll, drop) | manual QA | `[x]` |

**Exit 2:**

- [x] Threads tab: message list fills panel; composer pinned bottom; toolbar buttons visible
- [x] Plan ↔ Threads ×5 — no floating composer, no collapsed list, no editor gutter bleed in agent column
- [x] **AP1** Cmd-Option-Esc expand composer still works
- [x] **AP2** Expanded composer: bottom buttons display correctly
- [x] **AP3** Font size Cmd-+ / Cmd-- work
- [x] **AP4** Scrolling works in message list and Plan tab
- [x] **AP5** Files can be dropped into agent panel

---

## Sub-phase 3 — Unified resize model {#phase-1-4m-3}

**Goal:** All PulseBoard gutters use `DraggedColumn` + `apply_column_width` — no invisible deferred `DraggedDock` on split outer agent.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| **1.4m.3.1** | Outer agent: `render_outer_agent_column` (in-flow seams) — **done in code**; verify drag hits editor-facing seam | `layout_engine.rs` | `[x]` |
| **1.4m.3.2** | Remove / stop using `classic_dock_editor_gutter` + `DraggedDock` for `is_split_outer_agent` paths | `layout_engine.rs` | `[x]` |
| **1.4m.3.3** | Agent width: single source — `apply_column_width(AgentPanel)` → `resize_inner_agent_panel`; no parallel `resize_right_dock` for outer agent | `layout_engine.rs`, `workspace.rs` | `[x]` |
| **1.4m.3.4** | Double-click seam → `reset_column(AgentPanel)` | already in `column_seam` | `[x]` |
| **1.4m.3.5** | Nav + Threads gutters still resize correct column (regression check) | manual QA | `[x]` |

**Exit 3:**

- [x] Visible 14px seam between editor and agent; cursor `col-resize` on hover
- [x] Drag resizes agent column only; editor keeps `MIN_EDITOR_WIDTH`
- [x] Double-click seam resets agent default width

---

## Sub-phase 4 — Regression gate (re-close 1.4l QA) {#phase-1-4m-4}

**Goal:** Honest sign-off. Update [1.4l](./1-4l-side-column-layout-engine.md) status only when all pass.

### QA script (PulseBoard)

Fixture: `cd apps/CueCode-IDE && cargo run -p cuecode` → open `qa-fixture/pulseboard`

| # | Check | Pass |
|---|-------|------|
| Q1 | Nav tree visible; project files listed | `[x]` |
| Q2 | Threads column lists threads; selection syncs agent header title | `[x]` |
| Q3 | Open file from nav → **editor shows content** (tabs + text) | `[x]` |
| Q4 | Status bar terminal toggle → dock ≥200px; editor still usable above | `[x]` |
| Q5 | Nav gutter resizes nav only | `[x]` |
| Q6 | Threads gutter resizes threads only | `[x]` |
| Q7 | Editor↔agent gutter resizes agent only | `[x]` |
| Q8 | Agent Threads: messages scroll; composer at bottom | `[x]` |
| Q9 | Agent Plan tab → back to Threads ×5 — no layout corruption | `[x]` |
| Q10 | Window shrink — columns clamp; editor ≥ `MIN_EDITOR_WIDTH` | `[x]` |
| Q11 | No editor diff gutter paint in agent column | `[x]` |
| Q12 | Classic / Agentic layout presets unchanged (smoke) | `[x]` |

### Automated verify

```bash
cd apps/CueCode-IDE
CARGO_TARGET_DIR=$PWD/target cargo test -p workspace split_column
CARGO_TARGET_DIR=$PWD/target cargo check -p cuecode
./script/clippy -p workspace -p agent_ui
```

| ID | Task | Done |
|----|------|------|
| **1.4m.4.1** | All Q1–Q12 pass on pulseboard | `[x]` |
| **1.4m.4.2** | Mark [1.4l](./1-4l-side-column-layout-engine.md) `[x]` with Last verified date | `[x]` |
| **1.4m.4.3** | Update [07 §progress](../07-implementation-roadmap.md#progress) — 1.4m `[x]`, unblock 1.5 | `[x]` |

---

## Out of scope {#phase-1-4m-out-of-scope}

- Plan hub manifest / Implement ticket session ([1.4](./1-4-planning-hub-manifest.md) content) — layout only here
- Spec roots + pin modes ([1.5](./1-5-spec-roots-pin-modes.md))
- New layout presets or Layout Studio features ([1.4f](./1-4f-layout-studio.md))
- Agent ↔ editor **classic dock** path when `split_layout_active` is false

---

## Changelog {#phase-1-4m-changelog}

| Date | Change |
|------|--------|
| 2026-07-02 | Phase complete — PulseBoard Q1–Q12 pass; 1.4l re-closed; 1.5 unblocked |
| 2026-07-01 | Phase opened — PulseBoard QA failed after 1.4l + patch stack; documents sub-phases 0–4 |
