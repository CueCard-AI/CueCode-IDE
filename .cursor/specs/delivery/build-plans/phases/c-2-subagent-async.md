# Build phase C.2 — Subagent async {#phase-c-2}

> **Invoke:** `Build phase C.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | ~4 weeks |
| **Track** | C — Cloud harness (parallel) |
| **Roadmap** | [Cloud M2](../07-implementation-roadmap#phase-3b) |
| **QA script** | M2 exit — QA-P3b cloud variant |

## Deliverable {#phase-c-2-deliverable}

Cloud scheduler Async queue; `SpawnSubagent` CHP; `SessionNotification` push; sidechain transcript sync.

## Depends / blocks {#phase-c-2-deps}

| | Phase |
|---|-------|
| **Depends on** | C.1, 3b.1 |
| **Blocks** | C.3 |

## Out of scope {#phase-c-2-out-of-scope}

- VERDICT hybrid handoffs (C.3)
- BYOK (C.4)

---

## Tasks {#phase-c-2-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| C.2.1 | Server subagent spawner + builtin registry | private harness-api scheduler | `[ ]` |
| C.2.2 | CHP notification subscription (WebSocket or SSE) | `crates/cuecode_cloud/` | `[ ]` |
| C.2.3 | Client `ExecutionContext` on spawn from composer | `crates/cuecode_cloud/`, `crates/agent_ui/` | `[ ]` |
| C.2.4 | EC-20: duplicate `session_id` resume handling | `crates/cuecode_cloud/` | `[ ]` |
| C.2.5 | Task pills + rail — data from CHP | `crates/agent_ui/` | `[ ]` |
| C.2.6 | Sidechain transcript sync (cloud SoT) | `crates/cuecode_cloud/` | `[ ]` |

---

## Implementation notes {#phase-c-2-impl}

- Same `ExecutionContext` enum on wire as local `cuecode_sandbox`; server implements queues.
- `SpawnSubagent` CHP: explore + verification builtins.
- `SessionNotification` push maps to local notification kinds (reuse 3b.2 rail UI).
- Async lane does not block composer (configurable block flag for Active spawn).
- Sidechain view loads from cloud sync — cloud is source of truth.

---

## Verify {#phase-c-2-verify}

```bash
cd apps/CueCode-IDE
cargo test -p cuecode_cloud -- spawn
# Manual: background explore + notification
```

---

## Exit criteria {#phase-c-2-exit}

- [ ] Background explore completes with notification (matches QA-P3b)
- [ ] `session_id` resume without duplicate thread
- [ ] Async lane does not block composer (configurable)
- [ ] Sidechain view loads from cloud sync
- [ ] [08 §M2 exit](../../harness/cloud/08-roadmap.md#m2-exit) all rows

---

## QA {#phase-c-2-qa}

Manual steps before marking **Status** `[x]`:

1. Spawn background explore while typing in main composer
2. Notification on complete — pill + rail
3. Resume child via `session_id` — no duplicate thread
4. Explore subagent cannot edit — enforced server + client

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — M2 exit — QA-P3b cloud variant

---

## PR checklist {#phase-c-2-pr}

- [ ] PR title/body cites **Build phase C.2** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-c-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Scheduler | ../../harness/cloud/05-cloud-services.md#scheduler |
| Part B async (local) | ../../harness/local/01-agent-harness.md#part-b-async |
| Cloud M2 | ../../harness/cloud/08-roadmap.md#m2 |

---

## Changelog {#phase-c-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
