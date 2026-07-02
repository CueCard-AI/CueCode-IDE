# Build phase X.Y — Title {#phase-X-Y}

> **Invoke:** `Build phase X.Y` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ]` Not started · `[~]` In progress · `[x]` Done |
| **Last verified** | — |
| **Duration** | — |
| **Track** | — |
| **Roadmap** | [07 §phase-N](../07-implementation-roadmap#phase-N) |

## Deliverable {#phase-X-Y-deliverable}

One sentence: when this sub-phase is done, what is true?

## Depends / blocks {#phase-X-Y-deps}

| | Phase |
|---|-------|
| **Depends on** | — |
| **Blocks** | — |

## Out of scope {#phase-X-Y-out-of-scope}

What the **next** sub-phase owns — do not implement here.

---

## Tasks {#phase-X-Y-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| X.Y.1 | … | `crates/…` | `[ ]` |

---

## Implementation notes {#phase-X-Y-impl}

Types, modules, or behavior to implement **in this sub-phase** (inline — do not hunt other docs to start coding).

---

## Verify {#phase-X-Y-verify}

```bash
cd CueCode-IDE
# commands
```

---

## Exit criteria {#phase-X-Y-exit}

- [ ] …

---

## QA {#phase-X-Y-qa}

Manual steps before marking **Status** `[x]`:

1. …

**Script:** [07 §QA-PN](../07-implementation-roadmap#manual-qa-scripts)

---

## PR checklist {#phase-X-Y-pr}

- [ ] PR links **Build phase X.Y** and this file
- [ ] Tasks `X.Y.N` all `[x]` above
- [ ] Exit criteria checked
- [ ] Update [build-plans README](./README.md#phase-index) status column
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if milestone complete

---

## Deep specs (reference) {#phase-X-Y-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|

---

## Changelog {#phase-X-Y-changelog}

| Date | Change |
|------|--------|
| | Initial sub-phase doc |
