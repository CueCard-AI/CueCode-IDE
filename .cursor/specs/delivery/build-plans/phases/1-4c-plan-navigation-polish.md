# Build phase 1.4c — Plan ↔ Chat navigation polish {#phase-1-4c}

> **Invoke:** `Build phase 1.4c` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Complete |
| **Last verified** | 2026-06-19 |
| **Duration** | 1–2 days |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [16 §plan-ui-agent-tab](../../design/16-planning-hub.md#plan-ui-agent-tab) |

## Deliverable {#phase-1-4c-deliverable}

Agent panel **Threads | Plan | Terminal** navigation is always visible, never traps the user on Plan, and sidebar thread clicks always return to chat when appropriate.

## Depends / blocks {#phase-1-4c-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.4b (Plan tab, implement collapse) |
| **Blocks** | 1.5 (`@phase` UX — assumes reliable Plan ↔ Threads switching) |

## Out of scope {#phase-1-4c-out-of-scope}

Moved to [1.4d — Surface shortcuts & chrome polish](./1-4d-surface-shortcuts-polish.md) (complete).

- ~~Segmented control visual redesign beyond filled/transparent buttons~~
- ~~Keybindings `Cmd+1/2/3` for surfaces~~
- ~~Palette `CueCode: Focus Agent Chat` alias~~
- Default landing on Plan for first manifest open (optional product call — defer)

---

## Design contract {#phase-1-4c-design}

### Chrome layout (two rows)

```
ROW 1  Intent ▾  Sandbox ▾  Model ▾     title / pin     [+] [↗] [⋯]
ROW 2  [ Threads ] [ Plan ● ] [ Terminal ]
       (optional ROW 2b — plan strip when collapsed after Implement)
CONTENT …
```

**Rules:**
- Surface tabs live on **their own row** below the session toolbar — never inline with title/+.
- Plan strip (post-Implement) sits **between tab row and content**, not inside tab row.
- Active surface uses filled button style; inactive uses transparent.

### State sync

| User action | Panel result |
|-------------|--------------|
| Sidebar thread click | **Threads** + `load_agent_thread` (even if sidebar row already highlighted) |
| **Threads** tab from Plan/Terminal | Restore implement/retained thread, not empty draft |
| **Plan** tab | Full Plan view; ticket from strip or selection |
| Implement phase | **Threads ●** + strip visible |
| Pin chip / Focus Plan | **Plan ●** + ticket selected |

**Sidebar early-return rule:** only skip `load_agent_thread` when sidebar thread **and** panel `active_thread_id` both match.

---

## Tasks {#phase-1-4c-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4c.1 | Two-row chrome: toolbar row + surface tab row | `crates/agent_ui/src/agent_panel.rs` | `[x]` |
| 1.4c.2 | Plan strip on separate row between tabs and content | `crates/agent_ui/src/agent_panel.rs` | `[x]` |
| 1.4c.3 | `show_threads_surface` restores implement/retained thread | `crates/agent_ui/src/agent_panel.rs` | `[x]` |
| 1.4c.4 | Sidebar click loads thread when panel not showing that thread | `crates/sidebar/src/sidebar.rs` | `[x]` |
| 1.4c.5 | Update design spec cross-ref in 16 §plan-ui-agent-tab | `design/16-planning-hub.md` | `[x]` |

---

## Verify {#phase-1-4c-verify}

```bash
cd apps/CueCode-IDE
cargo check -p agent_ui -p sidebar -p cuecode
# Manual (pulseboard fixture):
# 1. Plan tab → Threads/Plan/Terminal visible at 420px panel width
# 2. Plan tab → click highlighted sidebar thread → chat appears
# 3. Implement phase → Threads selected, strip visible, Plan one click away
# 4. Plan tab → Threads tab → same Implement conversation (not empty draft)
```

---

## Exit criteria {#phase-1-4c-exit}

- [x] Surface tabs visible without horizontal scroll at default agent panel width
- [x] No “click a different thread first” workaround to reach chat
- [x] Threads tab after Implement restores implement session
- [x] Sidebar + panel surface stay logically in sync

---

## QA {#phase-1-4c-qa}

1. Open Plan → confirm **Threads | Plan | Terminal** on row below title bar.
2. Click sidebar thread (including already-highlighted) → chat + composer.
3. Implement phase → strip between tabs and chat; **Threads ●** active.
4. Plan tab → full Plan; Threads tab → back to Implement thread.
5. Pin chip → Plan; Threads → same thread.

---

## Deep specs {#phase-1-4c-specs}

| Topic | Doc |
|-------|-----|
| Plan UI wireframes | [16 §plan-ui-agent-tab](../../design/16-planning-hub.md#plan-ui-agent-tab) |
| Copy deck | [09 §plan-surface](../../design/09-ui-ux-spec.md#plan-surface-copy) |
| Parent phase | [1-4b-plan-ui-integration](./1-4b-plan-ui-integration.md) |

---

## Changelog {#phase-1-4c-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial sub-phase — navigation polish after 1.4b dogfood trap |
