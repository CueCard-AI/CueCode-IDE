# Build phase 6.1 — Release artifacts + docs {#phase-6-1}

> **Invoke:** `Build phase 6.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | Ongoing (first slice ~4 weeks) |
| **Track** | 6 — Ship |
| **Roadmap** | [Phase 6](../07-implementation-roadmap#phase-6) |
| **QA script** | QA-P6 |

## Deliverable {#phase-6-1-deliverable}

Rebranded release builds, docs site, example template repo, GPL source release process, crash reporting opt-in.

## Depends / blocks {#phase-6-1-deps}

| | Phase |
|---|-------|
| **Depends on** | 5.2 |
| **Blocks** | 6.2 |

## Out of scope {#phase-6-1-out-of-scope}

- Competitive 1.0 inventory gate (6.2)
- Enterprise BYOK cloud (C.4)

---

## Tasks {#phase-6-1-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 6.1.1 | macOS DMG / Linux tarball build scripts rebranded | `script/bundle-*.sh`, release CI | `[ ]` |
| 6.1.2 | Windows best-effort build notes | `docs/`, `CONTRIBUTING.md` | `[ ]` |
| 6.1.3 | CueCode docs site | `docs/` | `[ ]` |
| 6.1.4 | Example template repo with `.cursor/specs/` + skills | external template repo | `[ ]` |
| 6.1.5 | GPL source release process | `CONTRIBUTING.md`, release docs | `[ ]` |
| 6.1.6 | Crash reporting opt-in | `crates/telemetry/` | `[ ]` |
| 6.1.7 | Release notes template | `AGENTS.md` or `docs/release-notes.md` | `[ ]` |

---

## Implementation notes {#phase-6-1-impl}

- Release artifacts must not reference Zed branding or zed.dev update URLs.
- Docs site covers: onboarding, intent profiles, review + rewind, `@spec`.
- Template repo: minimal project with `.cursor/specs/` and `.cursor/skills/`.
- GPL checklist: source offer, LICENSE, third-party notices.

---

## Verify {#phase-6-1-verify}

```bash
cd apps/CueCode-IDE
./script/bundle-mac.sh  # or platform equivalent
# QA-P6 on clean VM
```

---

## Exit criteria {#phase-6-1-exit}

- [ ] [07 §phase-6-exit](../07-implementation-roadmap.md#phase-6-exit) rows 1–3
- [ ] [07 §phase-6-acceptance](../07-implementation-roadmap.md#phase-6-acceptance) rows 1–3 pass

---

## QA {#phase-6-1-qa}

Manual steps before marking **Status** `[x]`:

1. Install from release artifact (no `cargo build`)
2. Complete onboarding all four screens
3. Clone template repo — `@spec` works
4. Follow docs "review + rewind" page — matches UI copy
5. Run full QA-P6 on clean VM — all steps green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P6

---

## PR checklist {#phase-6-1-pr}

- [ ] PR title/body cites **Build phase 6.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-6-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| GPL checklist | ../../core/03-fork-and-rebrand.md#gpl-checklist |
| Build release | ../../ops/10-infrastructure.md#build-release |
| Release gates | ../../ops/11-metrics-and-success.md#release-gates |

---

## Changelog {#phase-6-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
