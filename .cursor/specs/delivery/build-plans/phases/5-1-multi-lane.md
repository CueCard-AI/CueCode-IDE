# Build phase 5.1 — Multi-lane model {#phase-5-1}

> **Invoke:** `Build phase 5.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 2–3 weeks |
| **Track** | 5 — Multi-lane & polish |
| **Roadmap** | [Phase 5](../07-implementation-roadmap#phase-5) |
| **QA script** | QA-P5 steps 2–4 |

## Deliverable {#phase-5-1-deliverable}

Parallel agent lanes with Explorer + Implementer presets, reviewer read-only lane, write conflict detection.

## Depends / blocks {#phase-5-1-deps}

| | Phase |
|---|-------|
| **Depends on** | 3.2, 3b.2 |
| **Blocks** | 5.2 |

## Out of scope {#phase-5-1-out-of-scope}

- Composer-first layout preset (5.2)
- Visual regression baselines (5.2)

---

## Tasks {#phase-5-1-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 5.1.1 | Lane tabs or split in agent panel | `crates/agent_ui/` | `[ ]` |
| 5.1.2 | Explorer + Implementer presets | `crates/cuecode_sandbox/src/builtin_agents.rs` | `[ ]` |
| 5.1.3 | Reviewer lane (read-only intent) | `crates/agent_ui/`, `crates/cuecode_sandbox/` | `[ ]` |
| 5.1.4 | Write conflict detection + banner | `crates/agent_ui/` | `[ ]` |
| 5.1.5 | `parent_session_id` / thread grouping | `crates/acp_thread/` | `[ ]` |

---

## Implementation notes {#phase-5-1-impl}

- Lane tabs or split view in agent panel; each lane = separate `AcpThread`.
- Explorer preset: Explore intent; Implementer: Fix/Ship intent.
- Reviewer lane: Review intent — write tools denied.
- Same-file edit from two lanes → conflict banner; no silent overwrite.
- `parent_session_id` groups coordinator + worker threads.

---

## Verify {#phase-5-1-verify}

```bash
cd apps/CueCode-IDE
cargo build -p agent_ui -p acp_thread
```

---

## Exit criteria {#phase-5-1-exit}

- [ ] Two lanes active without silent write conflicts — [07 §phase-5-acceptance](../07-implementation-roadmap.md#phase-5-acceptance) rows 1, 3

---

## QA {#phase-5-1-qa}

Manual steps before marking **Status** `[x]`:

1. Open Explorer + Implementer lanes — shared spec in header
2. Trigger same-file edit conflict — banner appears
3. Reviewer lane — write tool denied

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P5 steps 2–4

---

## PR checklist {#phase-5-1-pr}

- [ ] PR title/body cites **Build phase 5.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-5-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Multi-lane | ../../core/05-innovations.md#multi-lane |
| Coordinator | ../../parity/18-teams-and-tasks.md#coordinator |
| Flow C | ../../parity/16-end-to-end-flows.md#flow-c-coordinator |

---

## Changelog {#phase-5-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
