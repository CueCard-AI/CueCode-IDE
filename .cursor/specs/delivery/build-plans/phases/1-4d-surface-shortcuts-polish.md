# Build phase 1.4d — Surface shortcuts & chrome polish {#phase-1-4d}

> **Invoke:** `Build phase 1.4d` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Complete |
| **Last verified** | 2026-06-19 |
| **Duration** | ~0.5 day |
| **Track** | 1 — Planning Hub / Agent Linear |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [1.4c](./1-4c-plan-navigation-polish.md) |

## Deliverable {#phase-1-4d-deliverable}

Agent panel surfaces use a **segmented control**, palette shortcuts for **Focus Agent Chat** / **Focus Plan** / **Focus Terminal**, and **Cmd/Ctrl+1/2/3** when the agent panel is focused.

## Depends / blocks {#phase-1-4d-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.4c (Plan ↔ Chat navigation) |
| **Blocks** | — |

---

## Tasks {#phase-1-4d-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4d.1 | `ToggleButtonGroup` segmented control for surface tabs | `crates/agent_ui/src/agent_panel.rs` | `[x]` |
| 1.4d.2 | `cuecode::FocusAgentChat` + `cuecode::FocusTerminal` actions | `crates/cuecode_actions/src/lib.rs` | `[x]` |
| 1.4d.3 | Workspace handlers + register actions | `agent_panel.rs`, `planning_hub.rs` | `[x]` |
| 1.4d.4 | Keymaps (macOS cmd-1/2/3; Linux/Windows ctrl-1/2/3) | `assets/keymaps/default-*.json` | `[x]` |
| 1.4d.5 | Copy deck + 1.4c out-of-scope closure | `09-ui-ux-spec.md`, `16-planning-hub.md` | `[x]` |

---

## Verify {#phase-1-4d-verify}

```bash
cd apps/CueCode-IDE
cargo check -p agent_ui -p cuecode_actions -p cuecode
# Manual:
# 1. Agent panel → segmented Threads | Plan | Terminal bar
# 2. On Plan → Cmd/Ctrl+1 → chat restored
# 3. Cmd/Ctrl+2 → Plan; Cmd/Ctrl+3 → Terminal
# 4. Palette: CueCode: Focus Agent Chat / Focus Plan / Focus Terminal
```

---

## Exit criteria {#phase-1-4d-exit}

- [x] Surface tabs render as one outlined segmented control
- [x] Palette symmetric: Focus Agent Chat + Focus Plan + Focus Terminal
- [x] Keybindings scoped to `AgentPanel` context only

---

## Changelog {#phase-1-4d-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial — closes 1.4c out-of-scope polish items |
