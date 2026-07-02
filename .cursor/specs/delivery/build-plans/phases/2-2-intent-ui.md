# Build phase 2.2 — Intent UI + sandbox badge {#phase-2-2}

> **Invoke:** `Build phase 2.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 4–5 days |
| **Track** | 2 — Intent profiles |
| **Roadmap** | [Phase 2](../07-implementation-roadmap#phase-2) |
| **QA script** | QA-P2 full |

## Deliverable {#phase-2-2-deliverable}

Intent switcher in agent header, sandbox badge popover, keyboard cycle, settings stub. **Phase 2 complete** — unlocks 3.x, 3b.x, 4.x, C.0.

## Depends / blocks {#phase-2-2-deps}

| | Phase |
|---|-------|
| **Depends on** | 2.1 |
| **Blocks** | 3.1, 3b.1, 4.1, C.0 |

## Out of scope {#phase-2-2-out-of-scope}

- Checkpoint store (3.1)
- Multi-lane tabs (5.1)

---

## Tasks {#phase-2-2-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 2.2.1 | Intent switcher in agent header | `crates/agent_ui/` | `[ ]` |
| 2.2.2 | Sandbox badge + popover | `crates/agent_ui/` | `[ ]` |
| 2.2.3 | Hide write UI in Explore | `crates/agent_ui/` | `[ ]` |
| 2.2.4 | `cmd-shift-i` cycle intent | `crates/workspace/src/keymap.rs`, `crates/agent_ui/` | `[ ]` |
| 2.2.5 | Settings UI stub: Agent → Intent Profiles | `crates/settings_ui/` or settings panel | `[ ]` |
| 2.2.6 | Analytics: `cuecode.intent.*`, `cuecode.sandbox.*` | `crates/telemetry/` | `[ ]` |

---

## Implementation notes {#phase-2-2-impl}

- Intent switcher: segmented control or dropdown in agent panel header.
- Sandbox badge shows active policy; popover lists network + FS write rules.
- Explore: hide "Apply patch" and write affordances in composer toolbar.
- `cmd-shift-i` cycles Explore → Fix → Ship → Review with accessibility announcement.

---

## Verify {#phase-2-2-verify}

```bash
cd CueCode-IDE
cargo build -p agent_ui
# Manual: cmd-shift-i cycle, sandbox badge click
```

---

## Exit criteria {#phase-2-2-exit}

- [ ] [07 §phase-2-exit](../07-implementation-roadmap.md#phase-2-exit) + all [phase-2-acceptance](../07-implementation-roadmap.md#phase-2-acceptance) rows
- [ ] **Phase 2 complete** — unlocks 3.x, 3b.x, 4.x, C.0

---

## QA {#phase-2-2-qa}

Manual steps before marking **Status** `[x]`:

1. Set Explore — ask agent to edit `README.md` — verify deny in tool card
2. Switch Fix — repeat — verify confirm or execute path
3. Click sandbox badge — popover shows network + FS policy
4. Press `cmd-shift-i` four times — cycle all intents
5. Explore — verify no "Apply patch" in composer toolbar
6. Run full QA-P2 — all pass steps green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P2 full

---

## PR checklist {#phase-2-2-pr}

- [ ] PR title/body cites **Build phase 2.2** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-2-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Intent UI | ../../design/09-ui-ux-spec.md#intent-ui |
| Intent switcher innovation | ../../core/05-innovations.md#intent-switcher |

---

## Changelog {#phase-2-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
