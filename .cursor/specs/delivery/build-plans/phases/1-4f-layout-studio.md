# Build phase 1.4f — Layout Studio {#phase-1-4f}

> **Invoke:** `Build phase 1.4f` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done |
| **Last verified** | 2026-06-19 |
| **Duration** | 5–8 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [17 Layout Studio](../../design/17-layout-studio.md) |

## Deliverable {#phase-1-4f-deliverable}

**Layout Studio** — a modal with a virtualized dollhouse IDE, drag-to-slot layout blocks, workflow presets (Plan / Implement / Classic / Agentic / Dual), and **Apply Layout** that maps to `PanelLayout` + panel open + detach — without users touching dock JSON or tab-strip archaeology.

## Depends / blocks {#phase-1-4f-deps}

| | Phase |
|---|---|
| **Depends on** | 1.4e (detach, bottom dock, `PanelLayout`, collision reconcile) |
| **Blocks** | 1.5 (pin UX benefits from Plan preset), Phase 5 composer-first preset |

## Out of scope {#phase-1-4f-out-of-scope}

| Item | Owner |
|------|-------|
| Plan **pinned column** outside agent tab (dollhouse = pixel-perfect) | 1.4g / 1.5x |
| Review / Focus presets (functional) | Phase 3 / 5 |
| Split docks (two visible panels same edge) | Workspace engine — future |
| Threads sidebar detach | v2 |
| OS display window placement API | 1.4g polish |

---

## Tasks {#phase-1-4f-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/`.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4f.1 | `LayoutBlueprint` + validate + map to `PanelLayout` | `crates/agent_settings/src/layout_blueprint.rs` | `[x]` |
| 1.4f.2 | Preset definitions (Plan, Implement, Classic, Agentic, Dual) | `layout_blueprint.rs` | `[x]` |
| 1.4f.3 | `LayoutStudioView` — modal shell + dollhouse blocks | `layout_studio.rs`, `layout_studio_editor.rs` | `[x]` |
| 1.4f.4 | Slot drag-drop + invalid bounce + drop zones | `layout_studio_editor.rs` | `[x]` |
| 1.4f.5 | Preset carousel + morph preview (280ms) | `layout_studio_editor.rs` | `[x]` |
| 1.4f.6 | Focus slider (Plan ↔ Agent widths) | `layout_studio.rs` | `[x]` |
| 1.4f.7 | Apply pipeline: settings + open panels + detach + toasts | `layout_studio.rs`, `agent_panel.rs`, `planning_hub.rs` | `[x]` |
| 1.4f.8 | `cuecode::ArrangeWorkspace` + title bar Layout button | `cuecode_actions`, `title_bar.rs` | `[x]` |
| 1.4f.9 | Demote Panel Position to Advanced; link from agent `⋯` | `agent_panel.rs` | `[x]` |
| 1.4f.10 | Multi-display toggle + Display 2 frame (detach Plan) | `layout_studio.rs` | `[x]` |
| 1.4f.11 | First-run prompts (Plan tab, Implement) | `agent_panel.rs`, `plan_session.rs` | `[x]` |
| 1.4f.12 | Copy deck + design spec cross-refs | `17-layout-studio.md`, `09-ui-ux-spec.md` | `[x]` specs pre-linked |
| 1.4f.13 | Storybook / component preview for Layout Studio | `layout_studio_preview.rs`, `layout_studio_editor.rs` | `[x]` |

---

## Implementation notes {#phase-1-4f-impl}

### `LayoutBlueprint`

```rust
pub enum PlanHost {
    Column { side: DockSide, width: Pixels },
    TabInAgent,
    Detached { display: DisplayTarget },
}
pub enum AgentHost {
    Column { side: DockSide, width: Pixels },
    Bottom { height: Pixels },
    Detached { display: DisplayTarget },
}
```

- `from_merged_settings()` / `to_panel_layout()` bridge existing `PanelLayout`
- `validate()` enforces: Plan + Agent not same vertical slot unless one is Bottom/Detached
- `apply(fs, workspace, cx)` orchestrates settings write + panel focus

### Dollhouse blocks

- Render with theme tokens; no live editor data required
- Drag updates in-memory blueprint only until Apply
- Footer: `Custom · unsaved changes` when blueprint ≠ saved

### Layout Studio v3 UX (Phase A + B)

**Fixed canonical row:** Threads | Plan | Editor | Agent | Nav — always visible; ghost blocks (dashed, dimmed) when a panel is hidden or hosted elsewhere; status subtitle shows real placement (e.g. "Right edge", "In Agent tab").

**Primary placement path:** per-block chips (`Move X to:`) with active state filled; invalid options disabled with tooltip reason (Plan+Agent same column). Nav/Threads get **Inside left/right** chips when both share an edge (`BetweenOuterLeft` / `BetweenOuterRight`).

**Drag:** auto-selects dragged block; only valid gutters highlight during drag; no red invalid lines. Second-window strip appears when multi-display is on, Plan/Agent already detached, or dragging Plan/Agent with a valid Detached gutter.

**Discoverability:** `?` info tooltips on each block header; modal subtitle: *Click a panel, then pick where it goes — or drag to a highlighted zone.*

**Preview chrome:** `layout_studio_chrome.rs` mirrors fixed row + ghosts for Component Preview snapshots.

### Layout Studio v3 UX (Phase C — Delight)

**Click-to-move primary:** selecting a panel opens an inline placement menu inside the block (stacked buttons). Bottom hint reinforces click-first; drag is optional.

**Drag secondary:** muted ⋮⋮ handle with tooltip *"Optional — drag to a highlighted zone"*; gutters only appear during drag.

**Custom reset ramp:** when preset is Custom, quick *Start from Plan / Classic / Agentic / Dual* chips appear above presets.

**Apply gating:** Apply stays enabled when blueprint validates — placement errors are informational, not blockers (still blocked during active drag).

**Advanced demoted:** duplicate Panel Position dock toggles removed; Agent placement lives in the Agent block menu.

### Layout Studio v4 Track 1 — PlacementOutcome engine

**API:** `preview_placement` / `apply_placement` return `PlacementOutcome::{Changed, AlreadyThere, Invalid}` — no silent no-ops.

**Semantic equivalence:** `placement_equivalent` compares host sides and outer-strip order, ignoring width tweaks from `apply_focus_slider`.

**Drag targets:** `drag_gutters_for` only lists gutters where preview is `Changed` (no misleading BeforeEditor no-ops for Threads on left).

**UI feedback:** editor shows muted notice on `AlreadyThere`, error copy on `Invalid`.

**Tests:** 39+ blueprint tests — see Track 5 gate.

### Layout Studio v4 Track 2 — Interaction state machine

**Apply gating:** `is_interacting` = active drag only — hover preview no longer disables Apply.

**Drag lifecycle:** preview cleared each drag-move tick (gutters re-apply on hover); mouse-up always clears drag state; invalid gutter drag-over clears stale preview.

**Block UX:** select on header click only (placement buttons isolated); status subtitle hidden while block menu is open.

### Layout Studio v4 Track 3 — UI simplification

**One interaction model:** click panel header → arrow toolbar (◀ ▶ etc.) with tooltips. Gutter drag and text chip menus removed.

**Control panel spacing:** `gap_2` between cards; narrow blocks widen when selected so controls don't overlap neighbors.

**Menu filtering:** `meaningful_placement_actions_for` drives which arrows appear; active direction filled, invalid arrows disabled.

**Plan honesty:** banner when Plan is a column — *Plan column applies for this session — pinned column coming in 1.4g. Plan tab will open on Apply.*

### Layout Studio v4 Track 4 — Apply pipeline

**Dirty detection:** `has_unsaved_changes` uses `placement_equivalent` (not full struct equality or preset label alone). Status shows `{preset} · unsaved changes`.

**Apply gating:** Apply disabled when layout matches saved blueprint (no no-op applies).

**Post-Apply toast:** `apply_summary()` — e.g. *Agent docked right · Plan tab focused · Nav visible*.

**Plan tab honesty:** Plan block stays ghosted when `TabInAgent`; menu shows *Plan opens in the Agent tab on Apply*.

### Layout Studio v4 Track 5 — Test gate

**Placement matrix:** table-driven `(preset, block, action) → PlacementOutcome` for no-ops, moves, and collisions.

**Persist round-trip:** `write_settings` → `from_merged_settings` documents what survives Apply today (classic/agentic panel docks, threads sidebar) vs known gaps (Plan column, outer strip order, detached Plan, nav hidden, implement strip flag).

**Regression helpers:** test-only `settings_after_apply` simulates the Apply settings path without GPUI.

**Coverage:** 42+ `layout_blueprint` tests — run `cargo test -p agent_settings layout_blueprint` before merging Layout Studio changes.

### Layout Studio v5 — Fundamental UX + persist (PR1–PR3)

**PR1 — Placement ring (`layout_blueprint.rs`):**

- `placement_ring(block)` — ordered valid actions (excludes `Invalid` previews).
- `apply_placement_step(block, Prev|Next)` — wrap index; **unhide** is stepping off `Hidden` (no one-way 👁 trap).
- `block_status_short()` — compact card labels (`Left`, `Hidden`, `In tab`, …).
- `control_panel_segments()` — honest schematic row (outer strip order, columns, off-chips).

**PR2 — Honest control panel (`layout_studio_editor.rs`):**

- **Always-visible ◀ ▶** on every card and off-chip — no select-to-reveal.
- Row built from `control_panel_segments()` — Plan tab = badge on Agent, not fake Plan column.
- Hidden panels = dashed **off-chip** at row end (still steppable).
- Per-card feedback (notice/error on the card, not only modal footer).
- Label: *Control panel — schematic*; session banners removed.

**PR3 — Settings persist (`settings_content/agent.rs` + apply):**

- New agent settings: `layout_plan_host`, `layout_plan_column_width`, `layout_outer_left_order`, `layout_outer_right_order`, `layout_threads_hidden`, `layout_nav_hidden`, `layout_plan_strip_only`.
- `from_merged_settings` / `write_settings` round-trip all of the above.
- Apply closes Project/Outline when `nav == Hidden` (not just skip reveal).
- Track 5 gap tests flipped to **round-trip pass** (`plan_column_roundtrips`, etc.).

**PR4 — Apply matches schematic (`layout_studio_apply.rs`):**

- **Threads:** `Hidden` → close multi-workspace sidebar; `Left`/`Right` → open sidebar (side from persisted settings).
- **Nav:** `Hidden` → close Project, Outline, Git; visible → reveal all three.
- **Agent:** column/bottom → always reveal + focus Plan tab (Plan preset / plan column) or Agent chat (Classic, Custom, Implement, Agentic, Dual); detached → close agent on main window.

### Apply side effects (Plan preset)

1. `PanelLayout::reconcile_side_panels_for_agent_dock`
2. `workspace.focus_panel::<AgentPanel>` + `show_plan` / Plan tab
3. `workspace.reveal_panel` project or outline (remember last in user settings)
4. Toast via `StatusToast`

### Dual preset

- If Plan detached closed → `open_detached_plan_window`
- If chat should stay main → no agent detach

---

## Verify {#phase-1-4f-verify}

```bash
cd apps/CueCode-IDE
cargo check -p agent_ui -p agent_settings -p cuecode_actions -p cuecode
cargo test -p agent_settings layout_blueprint   # 44 tests — Layout Studio regression gate
# Component Preview: OpenComponentPreview → search "Layout Studio"
```

---

## Exit criteria {#phase-1-4f-exit}

- [x] Layout Studio opens from title bar icon, user menu, palette, agent menu
- [x] Plan preset applies without dock-tab hunting for Plan + Agent on opposite sides
- [x] Implement preset maps to strip layout; Apply collapses plan strip when implement ticket active
- [x] Dual preset opens detached Plan window
- [x] Invalid placement (Plan + Agent same column) shows error copy
- [x] Panel Position demoted to Advanced inside studio

---

## QA {#phase-1-4f-qa}

Fixture: `qa-fixture/pulseboard`

1. Title bar **Split** icon or **Arrange Workspace…** → studio opens with current layout reflected in dollhouse
2. Click **Plan** preset → **Apply** → Plan tab + agent opposite; outline/project reachable
3. Drag blocks between slots (or use slot buttons) → collision shows error copy
4. **Implement** on ticket → prompt → Apply → agent-right layout + plan strip when ticket active
5. **Dual** → Apply → Plan detached; editor + agent on main window
6. Dirty close → discard confirm when blueprint changed
7. `Cmd-Shift-P` → `Arrange Workspace` works

**Script:** [07 §QA-P1](../07-implementation-roadmap#manual-qa-scripts)

---

## PR checklist {#phase-1-4f-pr}

- [ ] PR links **Build phase 1.4f** and this file
- [ ] Tasks `1.4f.1–1.4f.13` all `[x]` above
- [ ] Exit criteria checked
- [ ] Update [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress)

---

## Deep specs (reference) {#phase-1-4f-specs}

| Topic | Doc |
|-------|-----|
| Full UX | [17-layout-studio](../../design/17-layout-studio.md) |
| Plan hosts | [16 §plan-ui](../../design/16-planning-hub.md#plan-ui) |
| Detach APIs | [1-4e](./1-4e-agent-layout-multi-window.md) |
| Panel layouts | `PanelLayout` in `agent_settings.rs` |

---

## Changelog {#phase-1-4f-changelog}

| Date | Change |
|------|--------|
| 2026-06-23 | Initial — Layout Studio build phase |
| 2026-06-19 | Core implementation: blueprint, modal, presets, apply pipeline, prompts |
| 2026-06-19 | 1.4f deferrals: block drag, 280ms morph, title bar icon, interactive preview, dirty-close confirm, implement strip on Apply |
| 2026-06-19 | v3 Phase B: chip-primary UX, disabled collision chips, Inside left/right, block tooltips, chrome sync |
| 2026-06-19 | v3 Phase C: inline click-to-move menus, drag demoted, custom reset ramp, Apply not blocked by errors |
| 2026-06-19 | v4 Track 1: PlacementOutcome API, placement_equivalent, meaningful drag targets, matrix tests |
| 2026-06-19 | v4 Track 2: Apply not blocked by hover preview; drag lifecycle cleanup; header-only select |
| 2026-06-19 | v4 Track 3: menus-only placement; control panel label; meaningful menu options; Plan session banner |
| 2026-06-19 | v4 Track 4: semantic dirty detection; Apply disabled when clean; apply_summary toast; Plan tab honesty |
| 2026-06-19 | v4 Track 5: placement matrix + persist round-trip tests; documents Plan column / strip order gaps |
| 2026-06-19 | v4 Track 3b: arrow placement controls + card spacing (no text menu overlap) |
| 2026-06-19 | v5 PR1: placement_ring + apply_placement_step; block_status_short; control_panel_segments |
| 2026-06-19 | v5 PR2: always-visible ◀▶; honest schematic row; off-chips; per-card feedback; Plan tab badge |
| 2026-06-19 | v5 PR3: layout_* settings persist; nav hide closes panels; round-trip tests pass (42 total) |
| 2026-06-19 | v5 PR4: `apply_blueprint_to_workspace` — reveal/close threads, nav, agent to match schematic |
| 2026-06-19 | v5 PR5: fix outer-strip edge vs inside ring index; status labels Edge L / Inside L |
| 2026-06-19 | 1.4h: agent/nav column split at runtime — inner agent strip + dock suppress when sharing a side |
| 2026-06-19 | 1.4h.6: split-aware resize — nav dock resizes visible panel; inner agent drag independent |
| 2026-06-19 | 1.4i: split-column resize UX — clamp math, nav min/ellipsis, no overlap |
| 2026-06-19 | 1.4j: inner column fixed-pixel sizing — agent inner drag with `flexible: true` |
