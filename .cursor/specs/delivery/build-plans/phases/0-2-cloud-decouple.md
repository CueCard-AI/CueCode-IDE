# Build phase 0.2 — Cloud decouple & defaults {#phase-0-2}

> **Invoke:** `Build phase 0.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x] Done` |
| **Last verified** | 2026-06-20 |
| **Duration** | 4–7 days |
| **Track** | 0 — Rebrand & decouple |
| **Roadmap** | [Phase 0](../07-implementation-roadmap#phase-0) |
| **QA script** | QA-P0, QA-RB-1 steps 5–7, QA-RB-2 |

## Deliverable {#phase-0-2-deliverable}

Agent works on first prompt with local/BYOK model. No sign-in wall, no Zed Pro upsell, no zed.dev default traffic.

## Depends / blocks {#phase-0-2-deps}

| | Phase |
|---|-------|
| **Depends on** | 0.1 |
| **Blocks** | 0.3 |

## Out of scope {#phase-0-2-out-of-scope}

- CLI rebrand, bundle scripts, CI grep (0.3)
- Spec index, intent profiles (Track 1+)

---

## Tasks {#phase-0-2-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 0.2.1 | Default model → Ollama/BYOK | `assets/settings/default.json` | `[x]` |
| 0.2.2 | Replace CueCode onboarding | `crates/ai_onboarding/`, `crates/onboarding/` | `[x]` |
| 0.2.3 | Remove Zed Pro / trial upsell | `crates/agent_ui/src/end_trial_upsell.rs`, `edit_prediction_ui/`, `thread_view.rs` | `[x]` |
| 0.2.4 | Hide sign-in + account UI | `crates/title_bar/`, `crates/cuecode/src/zed.rs` | `[x]` |
| 0.2.5 | Hide collab/channels menus | `crates/collab_ui/`, `crates/app_menus/src/app_menus.rs` | `[x]` |
| 0.2.6 | Stub/replace billing URLs | `crates/client/src/zed_urls.rs`, `crates/client/src/client.rs` | `[x]` |
| 0.2.7 | Disable auto-update to zed.dev | `crates/auto_update/` | `[x]` |
| 0.2.8 | Telemetry off by default | `crates/telemetry/`, `assets/settings/default.json` | `[x]` |
| 0.2.9 | About dialog: CueCode + GPL | About view in `crates/zed/` | `[x]` |

---

## Implementation notes {#phase-0-2-impl}

- Default agent model must not require zed.dev auth — Ollama or BYOK path.
- Onboarding screens: CueCode branding, no "Sign in to Zed".
- Grep target after changes:

```bash
rg -n "zed\.dev|Sign in to Zed|Zed Pro" crates --glob '!target/**'
```

Expect **zero hits** in user-facing agent/onboarding paths.

---

## Verify {#phase-0-2-verify}

```bash
cd CueCode-IDE
rg -n "zed\.dev|Sign in to Zed|Zed Pro" crates --glob '!target/**'
./script/qa-p0.sh
```

---

## Exit criteria {#phase-0-2-exit}

- [ ] Fresh install: agent prompt streams without auth — [07 §phase-0-acceptance](../07-implementation-roadmap.md#phase-0-acceptance) rows 2, 4, 7
- [ ] No new writes to `~/.config/zed/` — [03 §ERR-001](../../core/03-fork-and-rebrand.md#err-001)
- [ ] Idle app: no zed.dev traffic — [03 §ERR-006](../../core/03-fork-and-rebrand.md#err-006)
- [ ] [03 §first-launch-flow](../../core/03-fork-and-rebrand.md#first-launch-flow) reaches **SUCCESS: Phase 0 exit**

---

## QA {#phase-0-2-qa}

Manual steps before marking **Status** `[x]`:

1. Delete `~/.config/cuecode/` on test machine — fresh launch
2. Skip onboarding — open agent panel — no sign-in wall
3. Configure Ollama — send prompt — verify stream
4. Grep agent/onboarding paths for `zed.dev` — zero hits
5. Run `./script/qa-p0.sh` steps 3–4, 7 — pass

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P0, QA-RB-1 steps 5–7, QA-RB-2

---

## PR checklist {#phase-0-2-pr}

- [ ] PR title/body cites **Build phase 0.2** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-0-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Decouple defaults | ../../core/03-fork-and-rebrand.md#decouple-defaults |
| Onboarding mockup | ../../core/03-fork-and-rebrand.md#mockup-onboarding |
| Models infra | ../../ops/10-infrastructure.md#models |

---

## Changelog {#phase-0-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Generated from master build plan |
