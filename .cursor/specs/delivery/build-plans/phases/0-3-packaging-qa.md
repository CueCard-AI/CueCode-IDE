# Build phase 0.3 — Packaging, CLI & rebrand QA {#phase-0-3}

> **Invoke:** `Build phase 0.3` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x] Done` |
| **Last verified** | 2026-06-20 — 0.3.3 installer strings + 0.3.6 docs sweep |
| **Duration** | 2–4 days |
| **Track** | 0 — Rebrand & decouple |
| **Roadmap** | [Phase 0](../07-implementation-roadmap#phase-0) |
| **QA script** | QA-P0 full, QA-RB-3–5 |

## Deliverable {#phase-0-3-deliverable}

Shippable `cuecode` CLI, bundle scripts, CI rebrand gate, and full Phase 0 QA pass. Safe to start Track 1.

## Depends / blocks {#phase-0-3-deps}

| | Phase |
|---|-------|
| **Depends on** | 0.2 |
| **Blocks** | 1.1 |

## Out of scope {#phase-0-3-out-of-scope}

- `cuecode_specs` crate (1.1)
- Docs site, release DMG (6.1)

---

## Tasks {#phase-0-3-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 0.3.1 | CLI rebrand (`cuecode --help`) | `crates/cuecode/`, CLI entrypoints | `[x]` |
| 0.3.2 | Bundle scripts (macOS `.app`, Linux `.desktop`) | `script/bundle-*.sh`, packaging configs | `[x]` |
| 0.3.3 | Flatpak/snap/Windows installer strings | installer metadata, `crates/windows_resources/` | `[x]` |
| 0.3.4 | CI grep job for rebrand regression | `.github/workflows/` or CI scripts | `[x]` |
| 0.3.5 | Link CONTRIBUTING/README to specs | `CONTRIBUTING.md`, `README.md` | `[x]` |
| 0.3.6 | Optional: strip/replace `docs/` Zed copy | `docs/` | `[x]` |

---

## Implementation notes {#phase-0-3-impl}

- `cuecode --help` must show CueCode branding, not Zed.
- Bundle IDs and `.desktop` `Name=` fields must match `APP_NAME`.
- CI job runs `rebrand-check.sh` or equivalent grep gates on every PR.
- CONTRIBUTING links to `.cursor/specs/00-README.md`.

---

## Verify {#phase-0-3-verify}

```bash
cd apps/CueCode-IDE
./script/rebrand-check.sh
./script/qa-p0.sh
cuecode --help | head -20
```

---

## Exit criteria {#phase-0-3-exit}

- [ ] All [07 §phase-0-exit](../07-implementation-roadmap.md#phase-0-exit) checkboxes
- [ ] All [07 §phase-0-acceptance](../07-implementation-roadmap.md#phase-0-acceptance) rows pass
- [ ] **Phase 0 complete** — safe to start 1.1

---

## QA {#phase-0-3-qa}

Manual steps before marking **Status** `[x]`:

1. Run full `./script/qa-p0.sh` — all steps green
2. Run `cuecode --help` — CueCode strings only
3. Spot-check macOS `.app` bundle name in Finder
4. Confirm CI rebrand grep job exists and passes on main

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P0 full, QA-RB-3–5

---

## PR checklist {#phase-0-3-pr}

- [ ] PR title/body cites **Build phase 0.3** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-0-3-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| QA-RB-3–5 | ../../core/03-fork-and-rebrand.md#qa-rb-3 |
| Build release | ../../ops/10-infrastructure.md#build-release |
| Assets packaging | ../../core/03-fork-and-rebrand.md#assets-packaging |

---

## Follow-up: doc screenshots {#phase-0-3-follow-up}

Task **0.3.6** removed `zed.dev` / CDN URLs and linked local assets. The **10 doc screenshots** are still **upstream placeholders** — re-capture in CueCode when you have time.

| # | File (drop-in replace) | Doc page | Capture notes |
|---|------------------------|----------|---------------|
| 1–7 | `docs/src/assets/images/project-panel/*.png` | [project-panel.md](../../../../docs/src/project-panel.md) | CueCode window; sticky scroll, auto-fold, compare, git badges — see backlog §A1 |
| 8–10 | `docs/src/assets/images/troubleshooting/*.webp` | [troubleshooting.md](../../../../docs/src/troubleshooting.md) | macOS Instruments profiling **CueCode** — see backlog §A2 |

**Checklist:** [03-pass-d-docs-cdn-backlog.md §A](../../core/03-pass-d-docs-cdn-backlog.md#a-app-screenshots--10-total-capture--rehost) · **Index:** [build-plans README §doc-screenshots-backlog](../README.md#doc-screenshots-backlog)

After replacing files, rebuild docs (`cd docs && mdbook build`) and spot-check the two pages above.

---

## Changelog {#phase-0-3-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | 0.3.3/0.3.6 done; doc screenshot follow-up in §follow-up |
