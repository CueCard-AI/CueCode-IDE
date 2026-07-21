# Build phase 3.2 — Unified review panel {#phase-3-2}

> **Invoke:** `Build phase 3.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 7–10 days |
| **Track** | 3 — Review & checkpoints |
| **Roadmap** | [Phase 3](../07-implementation-roadmap#phase-3) |
| **QA script** | QA-P3 full |

## Deliverable {#phase-3-2-deliverable}

Review panel with Plan | Diffs | Terminal | Spec tabs; accept/reject partial; triggers on turn complete and `cmd-shift-r`. **Alpha milestone** reachable.

## Depends / blocks {#phase-3-2-deps}

| | Phase |
|---|-------|
| **Depends on** | 3.1 |
| **Blocks** | 5.1 |

## Out of scope {#phase-3-2-out-of-scope}

- Notification rail, VERDICT (3b.2)
- Multi-lane (5.1)

---

## Tasks {#phase-3-2-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 3.2.1 | review_panel.rs — Plan / Diffs / Terminal / Spec tabs | `crates/agent_ui/src/review_panel.rs` | `[ ]` |
| 3.2.2 | Accept all / reject all / partial | `crates/agent_ui/src/review_panel.rs` | `[ ]` |
| 3.2.3 | Triggers: turn complete, `cmd-shift-r`, notification | `crates/agent_ui/` | `[ ]` |
| 3.2.4 | Plan ↔ spec checkbox mapping (v1, confirm) | `crates/cuecode_specs/src/sync.rs` | `[ ]` |
| 3.2.5 | Analytics: `cuecode.review.*`, `cuecode.checkpoint.*` | `crates/telemetry/` | `[ ]` |

---

## Implementation notes {#phase-3-2-impl}

- Four tabs: Plan (ACP plan entries), Diffs (action_log hunks), Terminal (commands run), Spec (linked spec excerpt).
- Partial accept: per-file or per-hunk where supported.
- `cmd-shift-r` opens review when pending edits exist.
- Plan ↔ spec sync v1: propose checkbox toggles; user must confirm before apply (Q6).

---

## Verify {#phase-3-2-verify}

```bash
cd apps/CueCode-IDE
cargo build -p agent_ui
# Manual: QA-P3 full script
```

---

## Exit criteria {#phase-3-2-exit}

- [ ] [07 §phase-3-exit](../07-implementation-roadmap.md#phase-3-exit) + all acceptance rows
- [ ] **Alpha milestone** reachable — [07 §alpha-milestone](../07-implementation-roadmap.md#alpha-milestone)

---

## QA {#phase-3-2-qa}

Manual steps before marking **Status** `[x]`:

1. Fix intent — agent changes two files — review panel opens or badge shows count
2. Plan tab — verify entries; Terminal tab — commands listed
3. Accept one file, reject one — partial state correct
4. Reject all — files restored
5. `cmd-shift-r` opens review when pending edits exist
6. Run full QA-P3 — pass steps 2, 4, 5, 7 green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P3 full

---

## PR checklist {#phase-3-2-pr}

- [ ] PR title/body cites **Build phase 3.2** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-3-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Review panel UI | ../../design/09-ui-ux-spec.md#review-panel |
| Flow G security review | ../../parity/16-end-to-end-flows.md#flow-g-security-review |

---

## Changelog {#phase-3-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
