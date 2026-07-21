# Build phase 0.1 — Identity & paths {#phase-0-1}

> **Invoke:** `Build phase 0.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x] Done` |
| **Last verified** | 2026-06-20 — rebrand-check + qa-p0 (task 0.1.7 icons still open) |
| **Duration** | 2–3 days |
| **Track** | 0 — Rebrand & decouple |
| **Roadmap** | [Phase 0](../07-implementation-roadmap#phase-0) |
| **QA script** | QA-P0 partial, QA-RB-1 steps 1–4 |

## Deliverable {#phase-0-1-deliverable}

Binary name, `APP_NAME`, window title, and OS paths all say **CueCode**. Compile assert passes. No config collision with Zed on first launch.

## Depends / blocks {#phase-0-1-deps}

| | Phase |
|---|-------|
| **Depends on** | — |
| **Blocks** | 0.2 |

## Out of scope {#phase-0-1-out-of-scope}

- Cloud decouple, Ollama defaults, sign-in removal (0.2)
- CLI rebrand, bundle scripts, CI grep (0.3)
- Rename internal `crates/zed` crate — [03 §rename-depth L3](../../core/03-fork-and-rebrand.md#rename-depth)
- Rename `.zed/` project dirs — [12-open-questions](../../ops/12-open-questions.md)

---

## Tasks {#phase-0-1-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 0.1.1 | `APP_NAME = "CueCode"` | `crates/paths/src/paths.rs` | `[x]` |
| 0.1.2 | Binary `cuecode` | `crates/cuecode/Cargo.toml` `[[bin]]` | `[x]` |
| 0.1.3 | Compile-time assert passes | `crates/cuecode/src/main.rs` | `[x]` |
| 0.1.4 | Display names + app IDs | `crates/release_channel/src/lib.rs` | `[x]` |
| 0.1.5 | Window/title bar strings | `crates/title_bar/`, `crates/cuecode/src/zed.rs` | `[x]` |
| 0.1.6 | Windows metadata | `crates/windows_resources/` | `[x]` |
| 0.1.7 | App icons (minimal swap OK) | `assets/icons/`, `crates/icons/` | `[ ]` |

---

## Implementation notes {#phase-0-1-impl}

- `paths::APP_NAME` must be `"CueCode"` — config under `~/.config/cuecode/`.
- Release channel display: Stable → `"CueCode"`, Dev → `"CueCode Dev"`, etc.
- macOS bundle IDs: `dev.cuecode.CueCode`, etc. (see `release_channel::app_id`).
- Title bar must not show bare product name **Zed**.
- ERR-002 compile assert in `main.rs` must pass on every build.

---

## Verify {#phase-0-1-verify}

```bash
cd apps/CueCode-IDE
./script/rebrand-check.sh
./script/qa-p0.sh
```

---

## Exit criteria {#phase-0-1-exit}

- [ ] `cargo run --bin cuecode` launches
- [ ] Window title = **CueCode**
- [ ] First launch writes only under `cuecode` paths — [03 §G1–G5](../../core/03-fork-and-rebrand.md#goal-acceptance)
- [ ] [07 §phase-0-acceptance](../07-implementation-roadmap.md#phase-0-acceptance) rows 1, 3, 6 pass

---

## QA {#phase-0-1-qa}

Manual steps before marking **Status** `[x]`:

1. Launch app — title bar reads CueCode (light + dark)
2. Inspect `~/.config/` — only `cuecode/` created, not `zed/`
3. Run `./script/rebrand-check.sh` — Tier 1 identity gates green
4. Run `./script/qa-p0.sh` steps 1–2, 5 — pass

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P0 partial, QA-RB-1 steps 1–4

---

## PR checklist {#phase-0-1-pr}

- [ ] PR title/body cites **Build phase 0.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-0-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Rebrand tiers | ../../core/03-fork-and-rebrand.md |
| Paths architecture | ../../core/02-current-architecture.md#paths |
| UI title bar | ../../design/09-ui-ux-spec.md |

---

## Changelog {#phase-0-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Generated; 0.1.7 icons deferred |
