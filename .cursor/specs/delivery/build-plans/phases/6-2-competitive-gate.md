# Build phase 6.2 — Competitive 1.0 gate {#phase-6-2}

> **Invoke:** `Build phase 6.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | After 6.1 + parity backlog |
| **Track** | 6 — Ship |
| **Roadmap** | [Phase 6](../07-implementation-roadmap#phase-6) |
| **QA script** | Flows A–H |

## Deliverable {#phase-6-2-deliverable}

Competitive 1.0: inventory complete, command surface mapped, Flows A–H pass, surface matrix published.

## Depends / blocks {#phase-6-2-deps}

| | Phase |
|---|-------|
| **Depends on** | 6.1, 15 §P4 |
| **Blocks** | — (release gate) |

## Out of scope {#phase-6-2-out-of-scope}

- Post-1.0 parity backlog items marked Defer
- Cloud M4 enterprise (C.4) — parallel track

---

## Tasks {#phase-6-2-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 6.2.1 | 100% inventory rows Adopt/Adapt/Defer/Reject | `research/00-claude-code-inventory.md`, `research/01-parity-decisions.md` | `[ ]` |
| 6.2.2 | ≥90% top-60 commands mapped in GPUI | `parity/19-command-surface.md` | `[ ]` |
| 6.2.3 | Flows A–H manual QA pass | `parity/16-end-to-end-flows.md` | `[ ]` |
| 6.2.4 | [21 §surface-matrix](../../parity/21-ai-surfaces.md#surface-matrix) published | `parity/21-ai-surfaces.md` | `[ ]` |
| 6.2.5 | Link all PRs to inventory row IDs | PR process, `parity/15-competitive-parity.md` | `[ ]` |

---

## Implementation notes {#phase-6-2-impl}

- Every Claude Code inventory row must have Adopt/Adapt/Defer/Reject decision in `01-parity-decisions.md`.
- Top-60 commands: map to GPUI command palette / agent tools / settings.
- Flows A–H: manual QA with recorded build hash, date, tester.
- Surface matrix published in `21-ai-surfaces.md`.

---

## Verify {#phase-6-2-verify}

```bash
# Manual QA — Flows A–H
# See parity/16-end-to-end-flows.md
```

---

## Exit criteria {#phase-6-2-exit}

- [ ] [15 §competitive-gate](../../parity/15-competitive-parity.md#competitive-gate) all **Then** clauses pass

---

## QA {#phase-6-2-qa}

Manual steps before marking **Status** `[x]`:

1. Run Flow A (daily coding) — pass
2. Run Flow B (ship verify) — pass
3. Run Flows C–H — pass
4. Verify inventory 100% decided
5. Verify surface matrix published

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — Flows A–H

---

## PR checklist {#phase-6-2-pr}

- [ ] PR title/body cites **Build phase 6.2** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-6-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Competitive gate | ../../parity/15-competitive-parity.md#competitive-gate |
| End-to-end flows | ../../parity/16-end-to-end-flows.md |
| Command surface | ../../parity/19-command-surface.md |

---

## Changelog {#phase-6-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
