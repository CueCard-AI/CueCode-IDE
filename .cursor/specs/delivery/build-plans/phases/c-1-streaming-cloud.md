# Build phase C.1 — Streaming default cloud {#phase-c-1}

> **Invoke:** `Build phase C.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | ~4 weeks |
| **Track** | C — Cloud harness (parallel) |
| **Roadmap** | [Cloud M1](../07-implementation-roadmap#phase-3b) |
| **QA script** | M1 exit — stream latency |

## Deliverable {#phase-c-1-deliverable}

Production-shaped streaming; cloud agent default in CueCode cloud build; model-gateway v1; offline Model A fallback.

## Depends / blocks {#phase-c-1-deps}

| | Phase |
|---|-------|
| **Depends on** | C.0, 1.2 |
| **Blocks** | C.2 |

## Out of scope {#phase-c-1-out-of-scope}

- Background spawn cloud (C.2)
- BYOK enterprise (C.4)

---

## Tasks {#phase-c-1-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| C.1.1 | Gateway router + fast/quality tiers | private `cuecode-harness` model-gateway | `[ ]` |
| C.1.2 | Turn engine persistent transcript store | private harness-api | `[ ]` |
| C.1.3 | Client stream decoder → `acp_thread` | `crates/cuecode_cloud/` | `[ ]` |
| C.1.4 | Onboarding: cloud vs local explainer | `crates/ai_onboarding/` | `[ ]` |
| C.1.5 | Cloud build flag (`release_channel = cloud`) | `crates/release_channel/` | `[ ]` |
| C.1.6 | Context assembly v1 — spec index from client snapshot | `crates/cuecode_cloud/` | `[ ]` |

---

## Implementation notes {#phase-c-1-impl}

- model-gateway v1: CueCode-managed keys only (BYOK deferred to C.4).
- Streaming `SessionUpdate` → client decoder → `acp_thread` token stream.
- `release_channel = cloud` enables harness default; GPL tarball stays Model A.
- Offline / air-gap: fallback to local agent when no auth.
- Compaction v0 preserves linked spec path.

---

## Verify {#phase-c-1-verify}

```bash
cd apps/CueCode-IDE
cargo build -p cuecode_cloud --features cloud
# Measure first token < 3s p95
```

---

## Exit criteria {#phase-c-1-exit}

- [ ] First token < 3s p95 (managed route, US)
- [ ] Rate limit UX matches [07 §rate-limits](../../harness/cloud/07-model-gateway.md#rate-limits)
- [ ] Compaction v0 preserves linked spec path
- [ ] Cloud build launches with harness default; GPL build unchanged
- [ ] [08 §M1 exit](../../harness/cloud/08-roadmap.md#m1-exit) all rows

---

## QA {#phase-c-1-qa}

Manual steps before marking **Status** `[x]`:

1. Cloud build — sign in — default agent streams via harness-api
2. GPL build — no cloud sign-in required — local agent works
3. Air-gap / no auth — falls back to Model A
4. Rate limit hit — UX matches spec

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — M1 exit — stream latency

---

## PR checklist {#phase-c-1-pr}

- [ ] PR title/body cites **Build phase C.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-c-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Model gateway | ../../harness/cloud/07-model-gateway.md |
| Turn engine | ../../harness/cloud/05-cloud-services.md#turn-engine |
| Cloud M1 | ../../harness/cloud/08-roadmap.md#m1 |

---

## Changelog {#phase-c-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
