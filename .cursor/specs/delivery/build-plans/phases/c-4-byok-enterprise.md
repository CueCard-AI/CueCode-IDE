# Build phase C.4 — BYOK enterprise {#phase-c-4}

> **Invoke:** `Build phase C.4` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | ~6 weeks |
| **Track** | C — Cloud harness (parallel) |
| **Roadmap** | [Cloud M4](../07-implementation-roadmap#phase-3b) |
| **QA script** | M4 exit — SSO + BYOK smoke |

## Deliverable {#phase-c-4-deliverable}

BYOK passthrough gateway; org admin console; enterprise SSO; regional routing; audit export.

## Depends / blocks {#phase-c-4-deps}

| | Phase |
|---|-------|
| **Depends on** | C.3, 4.1 |
| **Blocks** | — (cloud track complete) |

## Out of scope {#phase-c-4-out-of-scope}

- GPL tarball cloud features
- Post-M4 compliance certifications

---

## Tasks {#phase-c-4-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| C.4.1 | BYOK encrypted key upload | model-gateway, settings UI | `[ ]` |
| C.4.2 | Org admin console (seats, usage, route policy) | private admin console | `[ ]` |
| C.4.3 | SSO (SAML/OIDC) | private auth service | `[ ]` |
| C.4.4 | Regional routing (EU pool) | model-gateway | `[ ]` |
| C.4.5 | Audit export (transcript + tool log JSONL) | harness-api + admin console | `[ ]` |
| C.4.6 | Usage dashboard: tokens per team | admin console | `[ ]` |

---

## Implementation notes {#phase-c-4-impl}

- BYOK: encrypted key upload; gateway passthrough with audit; key not stored at rest after session.
- SSO login → org JWT → harness access.
- Regional routing: EU pool for data residency.
- GDPR export: transcript JSONL download for compliance officers.

---

## Verify {#phase-c-4-verify}

```bash
# SSO smoke test
# BYOK session completes without key at rest
# Audit export download
```

---

## Exit criteria {#phase-c-4-exit}

- [ ] BYOK session completes without storing key at rest
- [ ] SSO login → org JWT → harness access
- [ ] Usage dashboard: tokens per team
- [ ] GDPR export: transcript JSONL download
- [ ] [08 §M4 exit](../../harness/cloud/08-roadmap.md#m4-exit) all rows

---

## QA {#phase-c-4-qa}

Manual steps before marking **Status** `[x]`:

1. Upload BYOK key — session completes — key not at rest after
2. SSO login — org JWT — harness access granted
3. Admin console — usage dashboard shows tokens per team
4. Export transcript JSONL — complete audit trail

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — M4 exit — SSO + BYOK smoke

---

## PR checklist {#phase-c-4-pr}

- [ ] PR title/body cites **Build phase C.4** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-c-4-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| BYOK flow | ../../harness/cloud/07-model-gateway.md#byok-flow |
| Keys BYOK | ../../harness/cloud/07-model-gateway.md#keys-byok |
| Cloud M4 | ../../harness/cloud/08-roadmap.md#m4 |
| Trust store (local) | ../../core/05-innovations.md#trust-graph |

---

## Changelog {#phase-c-4-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
