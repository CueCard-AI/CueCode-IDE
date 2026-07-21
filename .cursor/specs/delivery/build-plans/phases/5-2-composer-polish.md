# Build phase 5.2 — Composer-first layout + polish {#phase-5-2}

> **Invoke:** `Build phase 5.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 2–3 weeks |
| **Track** | 5 — Multi-lane & polish |
| **Roadmap** | [Phase 5](../07-implementation-roadmap#phase-5) |
| **QA script** | QA-P5 full |

## Deliverable {#phase-5-2-deliverable}

Composer-first layout, terminal replay, context budget UI optional, notification polish, visual baselines. **Beta milestone.**

## Depends / blocks {#phase-5-2-deps}

| | Phase |
|---|-------|
| **Depends on** | 5.1 |
| **Blocks** | 6.1 |

## Out of scope {#phase-5-2-out-of-scope}

- Release DMG, docs site (6.1)
- Competitive 1.0 gate (6.2)

---

## Tasks {#phase-5-2-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 5.2.1 | Composer-first layout preset | `crates/workspace/`, `crates/agent_ui/` | `[ ]` |
| 5.2.2 | Project panel collapsed default | `crates/project_panel/` | `[ ]` |
| 5.2.3 | Terminal replay (basic re-run) | `crates/agent_ui/`, `crates/terminal/` | `[ ]` |
| 5.2.4 | Context budget UI (optional, **additive**) — do not replace split I/O rings | `crates/agent_ui/` | `[ ]` |
| 5.2.5 | Notification rail polish | `crates/agent_ui/` | `[ ]` |
| 5.2.6 | Visual regression baselines | `crates/agent_ui/tests/` or screenshot CI | `[ ]` |

---

## Implementation notes {#phase-5-2-impl}

- Composer-first: agent panel ≥60% width on relaunch when preset enabled.
- Project panel collapsed by default in composer-first mode.
- Terminal replay: re-run captured commands from review Terminal tab.
- Context budget UI: optional category breakdown in header/popover — **must not remove** composer footer split I/O rings ([14 §split-token-usage](../../agent/14-agent-modes-and-builder.md#split-token-usage)).

---

## Verify {#phase-5-2-verify}

```bash
cd apps/CueCode-IDE
cargo build -p agent_ui -p workspace
# Manual: QA-P5 full
```

---

## Exit criteria {#phase-5-2-exit}

- [ ] [07 §phase-5-exit](../07-implementation-roadmap.md#phase-5-exit) all rows
- [ ] **Beta milestone** — [07 §beta-milestone](../07-implementation-roadmap.md#beta-milestone)

---

## QA {#phase-5-2-qa}

Manual steps before marking **Status** `[x]`:

1. Enable composer-first — relaunch — agent panel ≥60% width
2. Terminal replay — re-run command from review tab
3. Run full QA-P5 — all pass steps green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P5 full

---

## PR checklist {#phase-5-2-pr}

- [ ] PR title/body cites **Build phase 5.2** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-5-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Composer-first layout | ../../design/09-ui-ux-spec.md#composer-first-layout |
| Context budget | ../../core/05-innovations.md#context-budget |

---

## Changelog {#phase-5-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
