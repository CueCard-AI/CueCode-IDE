# Build phase 3b.2 — Notification rail + VERDICT {#phase-3b-2}

> **Invoke:** `Build phase 3b.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 7–10 days |
| **Track** | 3b — Active / Async harness (local) |
| **Roadmap** | [Phase 3b](../07-implementation-roadmap#phase-3b) |
| **QA script** | QA-P3b full |

## Deliverable {#phase-3b-2-deliverable}

Notification rail for async completions; VERDICT parse; FAIL blocks Ship complete; stop hooks v1 stub.

## Depends / blocks {#phase-3b-2-deps}

| | Phase |
|---|-------|
| **Depends on** | 3b.1 (3.2 recommended) |
| **Blocks** | 5.1, C.3 |

## Out of scope {#phase-3b-2-out-of-scope}

- Cloud VERDICT service (C.3)
- Multi-lane coordinator (5.1)

---

## Tasks {#phase-3b-2-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 3b.2.1 | Notification rail UI | `crates/agent_ui/` | `[ ]` |
| 3b.2.2 | Payload schema (XML/JSON) | `crates/cuecode_sandbox/src/execution.rs` | `[ ]` |
| 3b.2.3 | Verification agent + VERDICT parse | `crates/cuecode_sandbox/src/builtin_agents.rs` | `[ ]` |
| 3b.2.4 | FAIL blocks Ship complete (override audit) | `crates/agent_ui/`, `crates/cuecode_sandbox/` | `[ ]` |
| 3b.2.5 | Stop hooks v1 (memory extract stub) | `crates/agent/` | `[ ]` |
| 3b.2.6 | Analytics: harness events | `crates/telemetry/` | `[ ]` |

---

## Implementation notes {#phase-3b-2-impl}

- Notification rail: task pills + completion cards; click opens review Terminal tab on FAIL.
- VERDICT format: structured PASS/FAIL/PARTIAL — not prose inference.
- FAIL unacknowledged blocks session complete (EC-16); override writes audit entry.
- Stop hooks v1: stub memory extract on session end.

---

## Verify {#phase-3b-2-verify}

```bash
cd CueCode-IDE
cargo build -p agent_ui -p cuecode_sandbox
# Manual: QA-P3b full
```

---

## Exit criteria {#phase-3b-2-exit}

- [ ] [07 §phase-3b-exit](../07-implementation-roadmap.md#phase-3b-exit) + acceptance rows
- [ ] [15 §P2](../../parity/15-competitive-parity.md#phases) deliverables checked

---

## QA {#phase-3b-2-qa}

Manual steps before marking **Status** `[x]`:

1. Spawn background verification on small test crate
2. Wait for VERDICT — notification appears in rail
3. Click Review on FAIL — lands in review Terminal tab
4. FAIL blocks Ship complete until override
5. Run full QA-P3b — steps 3, 5 green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P3b full

---

## PR checklist {#phase-3b-2-pr}

- [ ] PR title/body cites **Build phase 3b.2** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-3b-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Notification rail | ../../harness/local/01-agent-harness.md#notification-rail-ui |
| Verification prompt | ../../harness/local/01-agent-harness.md#verification-prompt-outline |
| Stop hooks | ../../parity/17-memory-and-context.md#stop-hooks |

---

## Changelog {#phase-3b-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
