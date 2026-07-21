# Build phase 3.1 — Checkpoint store {#phase-3-1}

> **Invoke:** `Build phase 3.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 7–10 days |
| **Track** | 3 — Review & checkpoints |
| **Roadmap** | [Phase 3](../07-implementation-roadmap#phase-3) |
| **QA script** | QA-P3 steps 6–7 |

## Deliverable {#phase-3-1-deliverable}

Session-scoped checkpoints: snapshot action_log, plan, spec path, terminal IDs; create on turn complete; restore/rewind.

## Depends / blocks {#phase-3-1-deps}

| | Phase |
|---|-------|
| **Depends on** | 2.2 |
| **Blocks** | 3.2 |

## Out of scope {#phase-3-1-out-of-scope}

- Unified review panel tabs (3.2)
- Git stash integration default-on (flag, default off)

---

## Tasks {#phase-3-1-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 3.1.1 | `CheckpointStore` in `cuecode_sandbox` | `crates/cuecode_sandbox/src/checkpoint.rs` | `[ ]` |
| 3.1.2 | Snapshot: action_log, plan, spec path, terminal IDs | `crates/cuecode_sandbox/src/checkpoint.rs` | `[ ]` |
| 3.1.3 | Create on turn complete (configurable) | `crates/agent_ui/`, `crates/cuecode_sandbox/` | `[ ]` |
| 3.1.4 | Restore / rewind | `crates/cuecode_sandbox/src/checkpoint.rs` | `[ ]` |
| 3.1.5 | Optional git stash integration (flag, default off) | `crates/cuecode_sandbox/src/checkpoint.rs` | `[ ]` |
| 3.1.6 | Checkpoint timeline UI (minimal) | `crates/agent_ui/` | `[ ]` |

---

## Implementation notes {#phase-3-1-impl}

```rust
pub fn create_checkpoint(session: &SandboxSession, cx: &App) -> anyhow::Result<Checkpoint>;
pub fn list_checkpoints(session_id: &acp_thread::ThreadId) -> Vec<CheckpointMeta>;
pub fn restore_checkpoint(id: CheckpointId, opts: RestoreOptions, cx: &App) -> anyhow::Result<()>;
```

- Store under `~/.local/share/cuecode/checkpoints/<session_id>/cp_*.json`.
- Snapshot includes: pending + applied edits from `action_log`, plan, linked spec path, terminal session IDs.

---

## Verify {#phase-3-1-verify}

```bash
cd apps/CueCode-IDE
cargo test -p cuecode_sandbox -- checkpoint
```

---

## Exit criteria {#phase-3-1-exit}

- [ ] Rewind entire turn, not just last hunk — [07 §phase-3-acceptance](../07-implementation-roadmap.md#phase-3-acceptance) row 4

---

## QA {#phase-3-1-qa}

Manual steps before marking **Status** `[x]`:

1. Fix intent — agent changes two files — turn completes
2. Checkpoint timeline shows new entry
3. Restore previous checkpoint — confirm dialog — files rewind
4. Reject all in review — files restored to pre-turn state

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P3 steps 6–7

---

## PR checklist {#phase-3-1-pr}

- [ ] PR title/body cites **Build phase 3.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-3-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Checkpoint stack | ../../core/05-innovations.md#checkpoint-stack |
| Review lifecycle | ../../core/04-sandbox-core.md#review |
| Checkpoint flow | ../../core/06-system-design.md#checkpoint-flow |

---

## Changelog {#phase-3-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
