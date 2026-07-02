# Build phase C.3 — VERDICT Hybrid {#phase-c-3}

> **Invoke:** `Build phase C.3` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | ~3 weeks |
| **Track** | C — Cloud harness (parallel) |
| **Roadmap** | [Cloud M3](../07-implementation-roadmap#phase-3b) |
| **QA script** | M3 exit — VERDICT dogfood |

## Deliverable {#phase-c-3-deliverable}

Cloud VERDICT parser; FAIL blocks session complete; hybrid handoff artifacts; plan→implement→verify pipeline.

## Depends / blocks {#phase-c-3-deps}

| | Phase |
|---|-------|
| **Depends on** | C.2, 3b.2 |
| **Blocks** | C.4 |

## Out of scope {#phase-c-3-out-of-scope}

- BYOK enterprise SSO (C.4)
- Full Orchestrate coordinator (minimal v1 only)

---

## Tasks {#phase-c-3-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| C.3.1 | Verification agent outline + parser tests | private harness-api verdict service | `[ ]` |
| C.3.2 | Artifact store for evidence markdown | private harness-api | `[ ]` |
| C.3.3 | Client override audit log entry | `crates/cuecode_cloud/` | `[ ]` |
| C.3.4 | Coordinator spawn (Orchestrate intent) — minimal v1 | `crates/cuecode_sandbox/`, harness-api | `[ ]` |
| C.3.5 | Plan import to `AcpThread.plan` | `crates/cuecode_cloud/` | `[ ]` |
| C.3.6 | Unified review on VERDICT click | `crates/agent_ui/` | `[ ]` |

---

## Implementation notes {#phase-c-3-impl}

- Ship flow: approve plan (active) → implement worker (async) → verify (async) → VERDICT PASS → complete.
- VERDICT FAIL: structured notification — not prose inference; blocks session complete (EC-16).
- Hybrid handoff artifacts per [local §C.5](../../harness/local/01-agent-harness.md#c-5-hybrid-handoff-artifacts-required).
- Override writes append-only audit event (local + server).

---

## Verify {#phase-c-3-verify}

```bash
# Dogfood on internal repo
# VERDICT PASS and FAIL paths
```

---

## Exit criteria {#phase-c-3-exit}

- [ ] VERDICT FAIL shows structured notification — not prose inference
- [ ] FAIL unacknowledged blocks session complete
- [ ] Hybrid flow produces plan + checkpoint + VERDICT artifact chain
- [ ] PASS path dogfood on internal repo
- [ ] [08 §M3 exit](../../harness/cloud/08-roadmap.md#m3-exit) all rows

---

## QA {#phase-c-3-qa}

Manual steps before marking **Status** `[x]`:

1. Ship flow end-to-end — VERDICT PASS — mark complete
2. Inject FAIL — blocks complete until override
3. Click VERDICT notification — lands in unified review
4. Override audit entry written

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — M3 exit — VERDICT dogfood

---

## PR checklist {#phase-c-3-pr}

- [ ] PR title/body cites **Build phase C.3** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-c-3-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| VERDICT service | ../../harness/cloud/05-cloud-services.md#verdict |
| Part C hybrid | ../../harness/local/01-agent-harness.md#part-c-hybrid |
| Cloud M3 | ../../harness/cloud/08-roadmap.md#m3 |

---

## Changelog {#phase-c-3-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
