# Build phase 3b.1 — `run_in_background` + builtin agents {#phase-3b-1}

> **Invoke:** `Build phase 3b.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 7–10 days |
| **Track** | 3b — Active / Async harness (local) |
| **Roadmap** | [Phase 3b](../07-implementation-roadmap#phase-3b) |
| **QA script** | QA-P3b step 5 |

## Deliverable {#phase-3b-1-deliverable}

Background agent spawn with `ExecutionContext` Active/Async/Hybrid; builtin explore + verification agents; sidechain JSONL on disk.

## Depends / blocks {#phase-3b-1-deps}

| | Phase |
|---|-------|
| **Depends on** | 2.2 |
| **Blocks** | 3b.2, C.2 |

## Out of scope {#phase-3b-1-out-of-scope}

- Notification rail UI (3b.2)
- VERDICT parse + Ship block (3b.2)
- Cloud CHP spawn (C.2)

---

## Tasks {#phase-3b-1-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 3b.1.1 | `ExecutionContext` Active/Async/Hybrid | `crates/cuecode_sandbox/src/execution.rs` | `[ ]` |
| 3b.1.2 | `spawn_agent` + `run_in_background: bool` | `crates/agent/` tools, `crates/cuecode_sandbox/` | `[ ]` |
| 3b.1.3 | Builtin defs: explore, verification (tool walls) | `crates/cuecode_sandbox/src/builtin_agents.rs` | `[ ]` |
| 3b.1.4 | Sidechain JSONL under session dir | `crates/cuecode_sandbox/src/execution.rs` | `[ ]` |
| 3b.1.5 | Resume via `session_id` | `crates/acp_thread/`, `crates/agent_ui/` | `[ ]` |

---

## Implementation notes {#phase-3b-1-impl}

- `ExecutionContext`: Active (blocks composer optional), Async (background), Hybrid (handoff artifacts).
- Builtin explore agent: read-only tool wall — no `edit_file`.
- Builtin verification agent: test/run tools only.
- Sidechain transcripts: JSONL under session dir per [local §B.5](../../harness/local/01-agent-harness.md#b-5-async-artifacts-on-disk).
- Resume child thread by `session_id` without duplicate.

---

## Verify {#phase-3b-1-verify}

```bash
cd apps/CueCode-IDE
cargo test -p cuecode_sandbox -- execution
cargo build -p agent -p cuecode_sandbox
```

---

## Exit criteria {#phase-3b-1-exit}

- [ ] Explore subagent cannot call `edit_file` — [07 §phase-3b-acceptance](../07-implementation-roadmap.md#phase-3b-acceptance) row 1

---

## QA {#phase-3b-1-qa}

Manual steps before marking **Status** `[x]`:

1. Spawn background explore — completes without blocking main composer (per setting)
2. Attempt explore subagent `edit_file` — blocked
3. Resume via `session_id` — no duplicate thread

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P3b step 5

---

## PR checklist {#phase-3b-1-pr}

- [ ] PR title/body cites **Build phase 3b.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-3b-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Three contexts | ../../harness/local/01-agent-harness.md#three-contexts |
| Part B async | ../../harness/local/01-agent-harness.md#part-b-async |
| Spawn tool | ../../agent/08-agent-tools-and-skills.md |

---

## Changelog {#phase-3b-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
