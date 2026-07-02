# Build phase 1.4e — Agent layout & multi-window {#phase-1-4e}

> **Invoke:** `Build phase 1.4e` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done |
| **Last verified** | 2026-06-19 — `cargo check -p agent_ui -p cuecode_actions -p cuecode` |
| **Duration** | 3–5 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [16 §plan-ui](../../design/16-planning-hub.md#plan-ui) |

## Deliverable {#phase-1-4e-deliverable}

Agent panel is **dockable left/right/bottom** (Cursor-style), and **Threads + Plan** can each live in **detached OS windows** for multi-monitor workflows.

## Depends / blocks {#phase-1-4e-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.4d (surface tabs, Focus Agent Chat) |
| **Blocks** | 1.4f (Layout Studio), 1.5 polish, composer-first preset (Phase 5) |

---

## Design contract {#phase-1-4e-design}

### Docked layout

- `agent.dock`: `left` | `right` | `bottom` — all three valid
- Discover via agent `⋯` → **Panel Position** and dock tab right-click
- When Threads detached, main panel shows placeholder + **Dock chat** CTA

### Multi-window

```
Monitor 1: Editor (+ sidebar)     Monitor 2: Plan (detached)     Monitor 3: Chat (detached)
```

| Window | Host | Sync |
|--------|------|------|
| Plan detached | `PlanningHubView` + `PlanStore` | Selection, implement ticket |
| Threads detached | `DetachedAgentChatView` + `PlanStore.detached_threads_window` | Active `ConversationView` via `AgentPanel` |

**Implement phase:** focuses Threads host (main or detached); Plan window unchanged.

---

## Tasks {#phase-1-4e-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4e.1 | Enable bottom dock (`position_is_valid`) | `agent_panel.rs` | `[x]` |
| 1.4e.2 | Panel Position submenu in agent `⋯` menu | `agent_panel.rs` | `[x]` |
| 1.4e.3 | `DetachedAgentChatView` + store handle | `detached_agent_chat.rs`, `plan_store.rs` | `[x]` |
| 1.4e.4 | ↗ Open chat in window + Dock + main placeholder | `agent_panel.rs`, `detached_agent_chat.rs` | `[x]` |
| 1.4e.5 | `cuecode::OpenAgentChatInNewWindow` action | `cuecode_actions`, `planning_hub.rs` | `[x]` |
| 1.4e.6 | Focus Agent Chat activates detached window when open | `agent_panel.rs` | `[x]` |
| 1.4e.7 | Spec cross-refs in 16 + 09 copy deck | design specs | `[x]` |

---

## Verify {#phase-1-4e-verify}

```bash
cd CueCode-IDE
cargo check -p agent_ui -p cuecode_actions -p cuecode
# Manual:
# 1. agent.dock bottom — usable composer + tabs
# 2. ⋯ → Panel Position → Left/Right/Bottom
# 3. ↗ on Threads → detached chat; main panel placeholder
# 4. Plan ↗ + Threads ↗ simultaneously on two monitors
# 5. Implement → focuses detached chat if open
# 6. Dock on each window restores main panel tab
```

---

## Exit criteria {#phase-1-4e-exit}

- [x] Agent docks left, right, and bottom without layout breakage
- [x] Threads and Plan detachable at the same time
- [x] Dock restores correct surface in main window
- [x] Focus Agent Chat / palette opens or focuses detached chat

---

## Changelog {#phase-1-4e-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial — dock flexibility + dual detach |
| 2026-06-19 | Shipped: bottom dock, Panel Position menu, `DetachedAgentChatView`, palette action |
