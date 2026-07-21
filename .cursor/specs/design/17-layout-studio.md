# Layout Studio {#layout-studio}

> **Status:** Draft — spec 1.4f  
> **Related:** [09-ui-ux-spec §global-layout](./09-ui-ux-spec.md#global-layout) · [16-planning-hub §plan-ui](./16-planning-hub.md#plan-ui) · [1-4f build plan](../delivery/build-plans/phases/1-4f-layout-studio.md) · [1-4e multi-window](../delivery/build-plans/phases/1-4e-agent-layout-multi-window.md)

| Field | Value |
|-------|-------|
| **Product name** | **Layout Studio** |
| **One-liner** | macOS Displays energy for the IDE — a virtualized dollhouse workspace you drag into shape until Plan, Code, and Agent all breathe. |
| **Supersedes (UX)** | Raw “Dock Left/Right/Bottom” as primary layout control; flat title-bar “Agentic / Classic” as sole discovery path |

---

## Problem {#layout-studio-problem}

| Pain | Root cause |
|------|------------|
| “Dock Right doesn’t reshuffle” | Users expect **column layout**; product exposes **dock tab strips** (one visible panel per edge) |
| Outline/project vanish with agent on right | Agent + nav panels **share a dock** — tab hostage, not spatial layout |
| Plan buried in agent tabs | Plan is **Agent Linear backlog**, not a mode inside chat |
| Cursor is annoying to dock agents | Opaque, agent-centric, no visual layout model |
| 1.4e detach is powerful but hidden | ↗ buttons with no **spatial mental model** |

**Agent Linear** needs three surfaces visible without tab archaeology:

```
Plan (backlog + ticket)  |  Editor (work)  |  Agent (Implement)
```

Layout Studio makes that **literal, draggable, and preset-able**.

---

## Principles {#layout-studio-principles}

1. **Pictures before JSON** — every layout change has a thumbnail and a dollhouse preview.
2. **Workflows before edges** — presets named Plan / Implement / Classic, not “dock right.”
3. **Physics, not surprises** — invalid drops are rejected visually; nav auto-moves with explanation.
4. **Apply is explicit** — preview ≠ surprise mutation (macOS Displays pattern).
5. **Multi-monitor is first-class** — detach is drag-to-display, not an Easter egg.
6. **Session-loop aware** — layouts tie to Plan → Implement → Review journeys ([16 §journey-daily](./16-planning-hub.md#journey-daily)).

---

## Layout blocks {#layout-studio-surfaces}

Six draggable **Layout Blocks** in v1:

| Block ID | User label | Implementation | Detachable |
|----------|------------|----------------|------------|
| `threads` | **Threads** | Multi-workspace sidebar | v2 |
| `plan` | **Plan** | `PlanningHubView` — column, agent tab, or OS window | Yes |
| `editor` | **Editor** | Center `PaneGroup` | No (always present) |
| `agent` | **Agent** | `AgentPanel` | Yes |
| `nav` | **Project & Outline** | Project + Outline panels (grouped in dollhouse) | No |
| `terminal` | **Terminal** | Bottom dock band (Advanced only in v1) | No |

**v1 honesty:** The dollhouse **always shows Plan as its own column** when the preset demands it. Apply maps to the best available host (pinned column when engine supports it; else Plan tab + toast; detach when Dual preset).

---

## Entry points {#layout-studio-entry}

| Entry | Action |
|-------|--------|
| Title bar | **Layout** button (`LayoutGrid` icon) → Layout Studio modal |
| Palette | `CueCode: Arrange Workspace…` |
| Agent `⋯` | **Arrange Workspace…** (above deprecated Panel Position) |
| Settings | **Window & Layout → Open Layout Studio** |

### Contextual prompts

| Trigger | Prompt | Default ghost |
|---------|--------|---------------|
| First Plan tab | “Set up your workspace for planning?” | **Plan** |
| First Implement | “Switch to Implement layout?” | **Implement** |
| First detach ↗ success | “Save this as Dual Display?” | **Dual display** |
| Width &lt; 1024px | Banner: layouts work best at ≥1280px | — |

### Deprecate as primary UX

- Agent `⋯` → Panel Position → **Advanced ▾** inside Layout Studio only
- Title bar → Panel Layout → Agentic/Classic → **preset cards** in studio

---

## Modal anatomy {#layout-studio-anatomy}

**Presentation:** Modal layer 720–880px wide, max 90vh; backdrop 40% dim.

```
┌─ Layout Studio ─────────────────────────────────────────────── [×] ─┐
│ Arrange Workspace                                                    │
│ Drag blocks to set up Plan, Editor, and Agent for how you work.     │
├──────────────────────────────────────────────────────────────────────┤
│  ┌─ VIRTUAL WORKSPACE ──────────────────────────────────────────┐  │
│  │  [ Single display | Multiple displays ]                        │  │
│  │  ┌──────────────────────────────────────────────────────────┐  │  │
│  │  │ ░ Threads │ ██ Plan │ ████████ Editor │ ███ Agent │ Nav│  │  │
│  │  └──────────────────────────────────────────────────────────┘  │  │
│  │  ◀──────── Focus: Plan ────●──────── Agent ──────────▶         │  │
│  └────────────────────────────────────────────────────────────────┘  │
│  Presets: [Plan][Implement][Classic][Agentic][Dual] … scroll        │
│  ▾ Advanced                                                          │
├──────────────────────────────────────────────────────────────────────┤
│  Custom · unsaved          [ Reset preset ]  [ Cancel ] [ Apply ]   │
└──────────────────────────────────────────────────────────────────────┘
```

---

## Virtual workspace (hero) {#layout-studio-hero}

### Dollhouse fidelity (~65%)

| Block | Mini content |
|-------|----------------|
| **Threads** | 3 skeleton thread rows + search pill |
| **Plan** | Build track (2.1 ◐, 2.2 ○) + preview lines |
| **Editor** | Tab `project.yaml` + monospace lines + gutter |
| **Agent** | Tabs `Threads · Plan · Terminal` + bubbles + composer |
| **Nav** | Tree chevrons + outline symbols |

**Style:** live theme colors; 6px window chrome radius; 4px blocks; 1px `border_variant` dividers; 2px accent on selected/dragging block; virtual title bar 24px.

### Slot geometry (v1 — discrete)

```
┌────────┬────────┬─────────────────┬────────┐
│ SLOT_L1│ SLOT_L2│   SLOT_CENTER   │ SLOT_R1│
│ threads│ plan   │     editor      │ agent  │
│  or nav│  or nav│                 │ or nav │
└────────┴────────┴─────────────────┴────────┘
════════════════ SLOT_BOTTOM: agent or terminal ═══
```

| Rule | UX |
|------|-----|
| Plan + Agent same vertical slot | Zones gray; bounce animation |
| Agent bottom + Agent side | Side slot clears |
| Nav collides with Plan | Nav shown as tab chip on block |
| Editor | Cannot drag; flexes |

### Drop zones

- **Valid:** accent 8% fill + dashed 2px accent border + label (“Left column”, “Display 2”)
- **Invalid:** error 6% fill + 🚫 cursor

---

## Drag & drop {#layout-studio-dnd}

- **Handle:** 28px block header; cursor `grab` → `grabbing`
- **Lift:** Y −4px, shadow, scale 1.02
- **Snap:** 12px magnetic threshold; ghost at 50% opacity before release
- **Invalid release:** spring bounce 200ms

### Focus slider

Plan ↔ Editor ↔ Agent (macOS “Larger Text ↔ More Space” analog). Live-updates dollhouse widths; Apply commits.

| Position | Plan | Agent | Editor |
|----------|------|-------|--------|
| Plan-heavy | 380px | 340px | flex |
| Balanced | 320px | 420px | flex |
| Agent-heavy | strip/hidden | 560px | flex |

### Resize dividers

Vertical handles between blocks; mins: Threads 200, Plan 280, Agent 320, Nav 200, Editor 400 (flex).

---

## Multi-display mode {#layout-studio-multi-display}

Toggle: **Single display** | **Multiple displays**

```
┌─ Display 1 ──────────────────┐      ┌─ Display 2 ────────┐
│ Threads │ Editor │ Agent     │      │ Plan               │
└──────────────────────────────┘      └────────────────────┘
```

- Drag displays to swap spatial relationship (v1 horizontal only)
- Drag Plan onto Display 2 → `plan.host = detached(display:2)`
- One physical monitor: Display 2 labeled **Second Window** with caption *Opens as separate window*

---

## Preset carousel {#layout-studio-presets}

Cards 120×140px; selected = 2px accent ring. Click → dollhouse morph 280ms.

### Plan (Agent Linear default)

```
threads: left | plan: column-left 320px | editor: flex | agent: column-right 420px | nav: left or right (auto)
```

**Apply:** open Plan + nav (outline or project); focus Plan once. Toast: *Plan layout applied.*

### Implement

```
plan: strip | agent: right 480px | editor: flex priority | nav: collapsed
```

**Apply:** matches 1.4c post-Implement strip; focus composer.

### Classic

`PanelLayout::EDITOR` — nav left, agent right. Open project panel.

### Agentic

`PanelLayout::AGENT` — threads+agent left, nav right.

### Dual display

Display 1: threads | editor | agent. Display 2: Plan detached. Calls `Open Plan in New Window`.

### Review (stub)

Editor diff + narrow agent — wire in Phase 3.

### Focus (deferred)

Composer-first — Phase 5; hidden or feature-flagged.

---

## Advanced section {#layout-studio-advanced}

```
▾ Advanced
  Agent position      [ Left | Right | Bottom | Detached ]
  Plan host           [ Column | Tab in Agent | Detached ]
  Navigation          [ Left | Right | Hidden ]
  Threads sidebar     [ Left | Right | Hidden ]
  Terminal            [ Bottom | Hidden ]
  Save as workspace default [toggle]
```

---

## Apply pipeline {#layout-studio-apply}

1. Validate `LayoutBlueprint`
2. Write `PanelLayout` + host settings via `AgentSettings::set_layout`
3. Run collision resolver (`reconcile_side_panels_for_agent_dock` + extensions)
4. Move panels (settings observers on docks)
5. Open required panels; detach if blueprint says so
6. Serialize workspace; optional 200ms crossfade
7. Toast + close modal

### Blueprint (conceptual)

```rust
struct LayoutBlueprint {
    threads: ThreadsPlacement,
    plan: PlanHost,       // Column { side, width } | TabInAgent | Detached { display }
    agent: AgentHost,     // Column { side, width } | Bottom { height } | Detached
    nav: NavPlacement,
    editor: EditorFlex,
    focus_slider: f32,    // 0.0 plan-heavy .. 1.0 agent-heavy
}
```

Auto-move toast: *Moved Project & Outline to the left to make room for Agent.*

---

## Motion {#layout-studio-motion}

| Event | Duration |
|-------|----------|
| Modal open | 180ms fade + scale 0.96→1 |
| Preset morph | 280ms |
| Invalid drop bounce | 200ms |
| Apply crossfade | 200ms optional |
| Conflict pulse | 150ms |

Respect reduced-motion: instant snap, no morph.

---

## Copy deck {#layout-studio-copy}

| Element | String |
|---------|--------|
| Modal title | `Arrange Workspace` |
| Subtitle | `Drag blocks to set up Plan, Editor, and Agent for how you work.` |
| Title bar tooltip | `Arrange Workspace` |
| Palette | `CueCode: Arrange Workspace…` |
| Apply | `Apply Layout` |
| Focus slider ends | `Plan` / `Agent` |
| Multi-display | `Multiple Displays` |
| Display 1 / 2 | `Main Workspace` / `Second Window` |
| Invalid drop | `Plan and Agent can't share the same side` |
| Auto-move toast | `Moved {panel} to {side} to avoid overlap.` |
| First Plan prompt | `Set up your workspace for planning?` |

---

## Accessibility {#layout-studio-a11y}

- Blocks: focus ring + `aria-label`
- v1.1: keyboard slot editing (Tab, arrows, Enter)
- Preset carousel: arrow keys
- Invalid state announced to screen reader

---

## Technical map {#layout-studio-tech}

| Piece | Location (proposed) |
|-------|---------------------|
| `LayoutStudioView` | `crates/agent_ui/src/layout_studio.rs` |
| `LayoutBlueprint` | `crates/agent_settings/src/layout_blueprint.rs` |
| Apply | `layout_studio.rs` → `AgentSettings::set_layout`, detach APIs |
| Modal | workspace modal layer |
| Title bar | `crates/title_bar/src/title_bar.rs` |
| Action | `cuecode::ArrangeWorkspace` |
| Storybook | `component_preview` |

---

## Delivery ladder {#layout-studio-phases}

| Phase | Scope |
|-------|--------|
| **1.4f v1** | Modal, dollhouse, presets, slot drag, focus slider, Apply |
| **1.4g** | Morph polish, display arrangement, keyboard editing |
| **1.5x** | Plan pinned column (dollhouse = reality) |
| **5.x** | Review + Focus presets |

---

## Acceptance (dogfood) {#layout-studio-acceptance}

- [ ] Plan preset: Plan + Agent + Editor visible without dock-tab hunting
- [ ] Implement preset: strip + wide editor + agent
- [ ] Dual preset: Plan detached; main keeps editor + agent
- [ ] Invalid drop: bounce + message, no silent collision
- [ ] Dollhouse matches applied layout within one slot

---

## Changelog {#layout-studio-changelog}

| Date | Change |
|------|--------|
| 2026-06-23 | Initial — Layout Studio design spec (1.4f) |
