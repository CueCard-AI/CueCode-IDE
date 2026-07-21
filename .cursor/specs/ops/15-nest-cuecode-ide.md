# Nest CueCode-IDE — GPL client under `apps/` {#nest-cuecode-ide}

> **Parent:** [ops/README](./README.md) · Spec index: [00-README](../00-README.md)
>
> **Status:** **Done** (archive). Nest complete — IDE lives at `apps/CueCode-IDE/`
> (N0–N4). Day-to-day: [MONOREPO § Working in the nested IDE](../../../MONOREPO.md#working-in-the-nested-ide).
> Do **not** treat N0–N4 as active work.
>
> **Scope (historical):** Move the GPL IDE tree from monorepo-root was-`CueCode-IDE/`
> into `apps/CueCode-IDE/` without breaking public shadow publish, Cursor sync,
> harness CHP symlink, Plan E2E paths, or IDE CI.
>
> **Related:** [14-monorepo-layout](./14-monorepo-layout.md) ·
> [13-plan-e2e-fixture](./13-plan-e2e-fixture.md) ·
> [MONOREPO.md](../../../MONOREPO.md) · [AGENTS.md](../../../AGENTS.md) ·
> [AGENTS_GUIDE.md](../../../AGENTS_GUIDE.md) ·
> [03-fork-and-rebrand](../core/03-fork-and-rebrand.md) ·
> [harness/cloud/09-dev-and-deploy](../harness/cloud/09-dev-and-deploy.md)

---

## One sentence {#one-sentence}

Nest the CueCode desktop client under an `apps/` shelf so the monorepo root
reads as intentional systems — without breaking the GPL `git archive` publish
pipeline or any IDE↔harness↔QA coupling.

---

## Why this exists {#why}

[14-monorepo-layout](./14-monorepo-layout.md) deliberately left was-`CueCode-IDE/`
at root: it is the **public GPL archive source**, not a private shelf peer.

After H0–H6 (before this nest), the explorer looked like:

```text
cloud/  sites/  hiring/  ops/  research/  CueCode-IDE/  script/  …  # formerly at root
```

That is correct engineering under the old lock — and it still *feels* like a
special case. Nesting under `apps/` is better monorepo design **only if** the
publish and coupling matrix in this spec stays green.

This program is **not** a tidy-up. It is a **publish-pipeline migration**.

---

## Problem {#problem}

1. Root still hosts the largest product tree next to shelves, inviting “should
   this live under `cloud/`?” confusion (answer: never under `cloud/`).
2. Agents and humans already assume `cd CueCode-IDE` from monorepo root — a
   move without a verify matrix will strand builds, QA, and the public shadow.
3. Several couplings use **relative depth** from the IDE package root
   (`cuecode-local`, QA clone path, harness symlink). Nesting changes those
   depths even when the folder basename stays `CueCode-IDE`.

---

## Goals and non-goals {#goals}

### Goals

1. Monorepo path becomes **`apps/CueCode-IDE/`** (folder basename unchanged).
2. Public GitHub repo stays **`CueCard-AI/CueCode-IDE`** (name ≠ monorepo path).
3. Every mechanical move ships with CI, scripts, workspace, and coupling fixes
   in the **same** merge train as the verify matrix.
4. Zero intentional change to public shadow **content** beyond path-agnostic
   docs that already live inside the IDE tree (archive root of the public repo
   remains the IDE package root).
5. After merge, a fresh clone can build the IDE, sync `.cursor/`, run harness
   via `--harness`, and open the Plan E2E fixture without tribal knowledge.

### Non-goals

- Renaming the folder to `apps/cuecode` (v2 — doubles doc churn).
- Moving `.cursor/`, `.agents/`, or `script/` under `apps/`.
- Moving `cuecode-testing-repo/` (stays monorepo root).
- Nesting `hiring/` or changing SMBH publish.
- Relocating harness, sites, or dashboard.
- Changing GPL licensing, public remote URL, or force-push policy of publish.
- Workspace multi-root redesign beyond updating the IDE folder path.

### Principles {#principles}

- **Publish first.** If `git archive` / publish is wrong, nothing else matters.
- Prefer `git mv` (history).
- Keep folder basename `CueCode-IDE` in v1.
- One concern per PR phase (see [phases](#phases)); never mix with site deploys.
- Mechanical path rewrites are automated; verify matrix is human-gated.
- Rollback plan exists before merge ([rollback](#rollback)).

---

## Locked decisions {#locked}

| Decision | Lock |
|----------|------|
| Target path | `apps/CueCode-IDE/` |
| Folder rename | **No** in this program (`CueCode-IDE` basename stays) |
| Shelf name | `apps/` (not `products/`, not `client/`) |
| Public repo | `CueCard-AI/CueCode-IDE` unchanged |
| Archive prefix | `git archive HEAD:apps/CueCode-IDE` |
| `.cursor/` / `.agents/` | Stay at monorepo root; sync target becomes `apps/CueCode-IDE/.cursor` |
| `script/` | Stays at monorepo root |
| QA clone | `cuecode-testing-repo/` stays at monorepo root |
| Harness path | Stays `cloud/cuecode-harness/` |
| Prerequisite | [14](./14-monorepo-layout.md) H0–H6 on `main` |
| Phase order | N0 → N1 → N2 → N3 → N4 (see [phases](#phases)) |

**Rejected alternatives**

| Idea | Why rejected |
|------|----------------|
| Nest under `cloud/` | IDE is GPL client, not CueCloud backend |
| Nest under `sites/` | Not a marketing site |
| `apps/cuecode` rename in same PR | Extra blast; defer |
| Shim symlink `CueCode-IDE` → `apps/CueCode-IDE` at root | Hides broken path filters; forbid long-term (optional **N1-only** temporary shim is allowed if listed in PR and removed in N3) |

---

## Target hierarchy (relevant slice) {#target}

```text
CueInference/
├── apps/
│   └── CueCode-IDE/             # was `CueCode-IDE/` at root — GPL archive source
├── cloud/
│   └── cuecode-harness/
│       └── vendor/cuecode_chp → ../../../apps/CueCode-IDE/crates/cuecode_chp
├── sites/
├── hiring/
├── ops/
├── research/
├── cuecode-testing-repo/        # gitignored; KEEP at root
├── .cursor/  .agents/  script/  # KEEP at root
├── CueInference.code-workspace  # folder path → apps/CueCode-IDE
└── …
```

Public shadow after publish (unchanged shape):

```text
CueCard-AI/CueCode-IDE/          # repo root == former was-`CueCode-IDE/` contents
├── crates/
├── script/
├── .cursor/
└── …
```

---

## Coupling inventory {#couplings}

Every row must be green in the [verify matrix](#verify) before merge.

### P0 — break = stopped publish or broken local cloud loop

| ID | Coupling | Before nest | After nest (current) |
|----|----------|-------------|----------------------|
| C1 | GPL archive | `git archive HEAD:CueCode-IDE` in `script/publish-cuecode-ide` | `HEAD:apps/CueCode-IDE` |
| C2 | Publish path filters | `publish_cuecode_ide.yml` → `CueCode-IDE/**` | `apps/CueCode-IDE/**` |
| C3 | Cursor sync canonical | `script/sync-cursor-config` → `$ROOT/CueCode-IDE/.cursor` | `$ROOT/apps/CueCode-IDE/.cursor` |
| C4 | CHP symlink | `cloud/cuecode-harness/vendor/cuecode_chp` → `../../../CueCode-IDE/crates/cuecode_chp` | `../../../apps/CueCode-IDE/crates/cuecode_chp` |
| C5 | `cuecode-local` discovery | `$ROOT/../cloud/cuecode-harness` (`$ROOT` = IDE package) | `$ROOT/../../cloud/cuecode-harness` (+ keep legacy fallbacks) |
| C6 | Plan E2E open path | `cd CueCode-IDE && ./script/run-cuecode-dev ../cuecode-testing-repo` | `cd apps/CueCode-IDE && ./script/run-cuecode-dev ../../cuecode-testing-repo` |
| C7 | Rebrand / IDE CI | `cuecode_rebrand_check.yml` paths + `working-directory` + `CARGO_TARGET_DIR` + rust-cache workspaces | All under `apps/CueCode-IDE` |
| C8 | Workspace folder | `CueInference.code-workspace` → `CueCode-IDE` | `apps/CueCode-IDE` |

**Symlink depth note:** From `cloud/cuecode-harness/vendor/`, both
`../../../CueCode-IDE/...` (before nest) and `../../../apps/CueCode-IDE/...` (after)
use **three** `../` segments — only the final path segment changes. Do **not**
blindly add a fourth `../`.

**`cuecode-local` depth note:** `$ROOT` is the IDE package root. Nesting adds
one directory (`apps/`), so harness moves from one level up to **two** levels up
relative to `$ROOT`. Absolute `$HOME/CueInference/cloud/cuecode-harness`
candidates stay valid.

### P1 — break = wrong cwd / agent builds stale trees

| ID | Coupling | Action |
|----|----------|--------|
| C9 | `AGENTS.md` / `AGENTS_GUIDE.md` hard rules | Canonical path + cargo sanity substring |
| C10 | `RUNBOOK.md`, `README.md`, `MONOREPO.md`, `CONTRIBUTING.md` | Rewrite `cd CueCode-IDE` / table paths |
| C11 | `script/clone-qa-fixture-repo` echo | Print `apps/CueCode-IDE` open command |
| C12 | was-`CueCode-IDE/script/run-cuecode-dev` staging warning | Update hardcoded absolute path example |
| C13 | `cloud/cuecode-harness/script/dev-harness` error text | Fix stale `../CueCode-IDE` message |
| C14 | Harness README / AGENTS relative links | `../../CueCode-IDE` → `../../apps/CueCode-IDE` (or equivalent) |
| C15 | Layout spec 14 | Point “IDE at root” locks to this spec; mark superseded |

### P2 — break = confusion / stale verify blocks

| ID | Coupling | Action |
|----|----------|--------|
| C16 | Delivery build-phase verify blocks | Mass rewrite `cd CueCode-IDE` → `cd apps/CueCode-IDE` (generated + hand-written) |
| C17 | `.agents/skills/**` | Path mentions |
| C18 | Marketing “not under CueCode-IDE” copy | Still true if path is `apps/CueCode-IDE`; update tree diagrams that show was-`CueCode-IDE/` at root |
| C19 | Harness cloud `09-dev-and-deploy` trees | Update checkout diagram |
| C20 | was-`CueCode-IDE/docs/cuecode-harness-template` | Sibling layout diagram |
| C21 | Any remaining root-docs / specs | Grep gate in N3 |

### Out of scope couplings (do not “fix” here)

| Item | Why |
|------|-----|
| Public URLs `github.com/CueCard-AI/CueCode-IDE` | Repo name unchanged |
| CHP subprotocol string `cuecode-harness.chp.v1` | Not a filesystem path |
| Remote `/opt/cuecloud-*` | Unrelated |
| Inside-IDE relative crate paths | Package-internal; archive root unchanged |

---

## File / script checklist {#checklist}

### Must edit (mechanicals)

| Path | Change |
|------|--------|
| `script/publish-cuecode-ide` | `HEAD:apps/CueCode-IDE`; commit message text |
| `script/sync-cursor-config` | `CANONICAL="$ROOT/apps/CueCode-IDE/.cursor"`; help text |
| `.github/workflows/publish_cuecode_ide.yml` | path filters + step names |
| `.github/workflows/cuecode_rebrand_check.yml` | paths, `working-directory`, `CARGO_TARGET_DIR`, cache workspaces |
| `CueInference.code-workspace` | folder `path` (+ display name if desired) |
| `cloud/cuecode-harness/vendor/cuecode_chp` | retarget symlink |
| was-`CueCode-IDE/script/cuecode-local` (after mv: under `apps/`) | discovery candidates + error string |
| `script/clone-qa-fixture-repo` | printed open-in-CueCode path |
| `apps/CueCode-IDE/script/run-cuecode-dev` | staging warning path |
| `cloud/cuecode-harness/script/dev-harness` | missing-symlink hint |

### Must edit (docs / agent surfaces)

| Path | Change |
|------|--------|
| `AGENTS.md`, `AGENTS_GUIDE.md` | Canonical paths, sanity check |
| `README.md`, `MONOREPO.md`, `RUNBOOK.md`, `CONTRIBUTING.md` | Tables + commands |
| `.cursor/specs/ops/14-monorepo-layout.md` | Cross-link; IDE no longer “immutable forever” |
| `.cursor/specs/ops/13-plan-e2e-fixture.md` | Open commands |
| `.cursor/specs/harness/cloud/09-dev-and-deploy.md` | Tree |
| Delivery / agent specs with `cd CueCode-IDE` | Mechanical rewrite |
| This spec status table | Mark phases done as they ship |

### Optional temporary

| Path | Notes |
|------|-------|
| Root symlink `CueCode-IDE` → `apps/CueCode-IDE` | **Only** if N1 needs a bridge for in-flight local scripts; must be deleted in N3; never commit as permanent |

---

## Phased delivery {#phases}

Invoke with: **`Build nest CueCode-IDE phase N0`** … **`N4`**
(or **`Build ops/15 phase N…`**).

### Phase N0 — Spec + freeze {#n0}

**Do**

1. This spec locked and indexed (ops README, 00-README, MONOREPO pointer).
2. Confirm [14](./14-monorepo-layout.md) H0–H6 is on `main`.
3. Snapshot baseline: public shadow tip SHA; local `git archive HEAD:CueCode-IDE | tar -t | wc -l`.
4. Freeze: no parallel refactors of publish / harness / rebrand CI.

**Baseline snapshot (2026-07-20, recorded on `ops/nest-cuecode-ide`)**

| Item | Value |
|------|-------|
| Monorepo commit (spec PR) | `d2e58d6d3c` (N0 commit; tip may advance) |
| `git archive HEAD:CueCode-IDE \| tar -t \| wc -l` | **5222** |
| Public shadow `CueCard-AI/CueCode-IDE@main` | `52da0bce4fa99056297599c20ec2c4beb887f598` |
| Public tip message | published from `CueInference/main@08c4caa8d204` |
| Layout H0–H4b on `main` | Yes (via PR #14) |
| Layout H4c–H6 on `main` | **No** — open [PR #17](https://github.com/CueCard-AI/CueInference/pull/17); **block N1** until merged (or rebase nest PR onto #17 tip) |
| Freeze | Do not parallel-edit `script/publish-cuecode-ide`, `script/sync-cursor-config`, `cuecode_rebrand_check.yml`, or harness `vendor/cuecode_chp` outside this program |
| Verify-matrix owner (merge day) | Engineer merging [PR #18](https://github.com/CueCard-AI/CueInference/pull/18) — run [verify matrix](#verify) on a fresh clone |

**Exit**

- [x] Spec indexed + synced (`./script/sync-cursor-config --to-ide`)
- [x] Baseline archive file count recorded (5222) + public tip SHA above
- [x] Owner assigned for verify matrix on merge day (PR #18 merger)

---

### Phase N1 — Mechanicals {#n1}

```bash
mkdir -p apps
git mv CueCode-IDE apps/CueCode-IDE
```

**Same PR (or immediate follow-up blocked on N1 CI):**

1. Update C1–C3, C7–C8 (publish, sync, workflows, workspace).
2. Do **not** leave path filters pointing at root was-`CueCode-IDE/**`.
3. Optional: temporary root symlink (document in PR; remove in N3).

**Exit**

- [x] `test -d apps/CueCode-IDE/crates`
- [x] `test ! -e CueCode-IDE` **or** symlink-only with PR note (no symlink used)
- [x] `git archive HEAD:apps/CueCode-IDE | tar -t | head` succeeds
- [x] `publish_cuecode_ide.yml` / `cuecode_rebrand_check.yml` path filters updated
- [x] `sync-cursor-config --to-ide` writes under `apps/CueCode-IDE/.cursor`

**Do not** merge N1 without at least C1–C3 and C7–C8 green in CI on the PR.

---

### Phase N2 — Couplings {#n2}

1. Retarget `vendor/cuecode_chp` (C4); fix `dev-harness` message (C13).
2. Update `cuecode-local` (C5).
3. Update QA open paths (C6, C11, spec 13).
4. Update harness README/AGENTS links (C14).
5. Update `run-cuecode-dev` warning (C12).

**Exit**

- [x] Symlink resolves to `apps/CueCode-IDE/crates/cuecode_chp`
- [x] `cd cloud/cuecode-harness && cargo check -p harness-api`
- [x] Discovery script finds harness via relative candidate from nested IDE
- [x] Documented Plan E2E command uses `../../cuecode-testing-repo`

N1+N2 may ship as **one** PR if the verify matrix for both is complete.

---

### Phase N3 — Docs / specs mass rewrite {#n3}

1. Automated rewrite of monorepo-relative `CueCode-IDE/` → `apps/CueCode-IDE/`
   where it means the **filesystem path** (not the GitHub repo URL).
2. Protect false positives:
   - `github.com/CueCard-AI/CueCode-IDE`
   - `CueCard-AI/CueCode-IDE` repo refs
   - Historical changelog rows (optional: leave + add “was root” note)
3. Remove temporary root symlink if present.
4. `./script/sync-cursor-config --to-ide`.
5. Grep gate (see [grep gate](#grep-gate)).

**Exit**

- [x] Grep gate clean
- [x] AGENTS / AGENTS_GUIDE / RUNBOOK / README / MONOREPO updated
- [x] Spec 14 cross-link updated
- [x] No root `CueCode-IDE` file/dir/symlink left

---

### Phase N4 — Verify + land {#n4}

Run the full [verify matrix](#verify) on a **fresh clone** of the PR branch.
Merge only when all P0/P1 checks pass. Watch Actions on `main` after merge
([post-merge](#post-merge)).

**Exit**

- [x] Fresh-clone verify matrix signed off
- [x] Post-merge publish + rebrand workflows green (or explicitly waived with reason)
- [x] This spec status table → **Done**
- [x] Changelog row on this spec + brief note in MONOREPO layout section

---

## Grep gate {#grep-gate}

After N3, from monorepo root, actionable hits for a **filesystem** root path
must be zero outside allowlisted historical text.

Suggested audit (adjust for tool flavor):

```bash
# Fail if root-relative filesystem path still used as cwd/path (exclude URLs)
rg -n --pcre2 \
  '(?<![/\w])CueCode-IDE/' \
  --glob '!**/node_modules/**' --glob '!**/target/**' \
  AGENTS.md AGENTS_GUIDE.md README.md MONOREPO.md RUNBOOK.md CONTRIBUTING.md \
  script .github CueInference.code-workspace \
  .cursor/specs cloud sites hiring ops \
  | rg -v 'github\.com/CueCard-AI/CueCode-IDE|CueCard-AI/CueCode-IDE' \
  | rg -v 'apps/CueCode-IDE/' \
  | rg -v 'was `CueCode-IDE/`|was-`CueCode-IDE|formerly at root|HEAD:CueCode-IDE|git mv CueCode-IDE|nesting `CueCode-IDE/|\(\?<!\[/\\w\]\)CueCode-IDE'
```

Also confirm **positive** presence:

```bash
test -d apps/CueCode-IDE
rg -n 'HEAD:apps/CueCode-IDE' script/publish-cuecode-ide
rg -n 'apps/CueCode-IDE' script/sync-cursor-config
rg -n "apps/CueCode-IDE" .github/workflows/publish_cuecode_ide.yml
rg -n "apps/CueCode-IDE" .github/workflows/cuecode_rebrand_check.yml
```

---

## Verify matrix {#verify}

Run on a **clean clone** of the branch (not a dirty worktree that still has
muscle-memory symlinks).

| # | Check | Pass criteria |
|---|-------|---------------|
| V1 | Sync | `./script/sync-cursor-config --to-ide` exits 0; file appears under `apps/CueCode-IDE/.cursor/` |
| V2 | Archive list | `git archive HEAD:apps/CueCode-IDE \| tar -t \| head` shows `Cargo.toml`, `crates/`, `script/` at archive root |
| V3 | Archive parity | File count ≈ N0 baseline; spot-diff vs previous `HEAD:CueCode-IDE` export from pre-move tag |
| V4 | Publish dry-run | Export archive → `diff -qr` against cloned public shadow tip; only expected deltas |
| V5 | Symlink | `readlink` / `realpath` of `cloud/cuecode-harness/vendor/cuecode_chp` → `…/apps/CueCode-IDE/crates/cuecode_chp` |
| V6 | Harness build | `cd cloud/cuecode-harness && cargo check -p harness-api` |
| V7 | Local harness | From `apps/CueCode-IDE`, `script/cuecode-local --harness` reaches `http://127.0.0.1:8787/health` |
| V8 | QA path | `./script/clone-qa-fixture-repo` then `cd apps/CueCode-IDE && ./script/run-cuecode-dev ../../cuecode-testing-repo` starts against fixture |
| V9 | IDE check | `cd apps/CueCode-IDE && cargo check -p cuecode` (or rebrand workflow job equivalent) |
| V10 | Workspace | Open `CueInference.code-workspace`; IDE folder loads |
| V11 | Grep gate | [Grep gate](#grep-gate) clean |
| V12 | No root leftover | `test ! -e CueCode-IDE` |

**P0 subset (merge blocker):** V1–V7, V11–V12.  
**P1 subset (merge blocker):** V8–V10.  
**V4** may be dry-run only if token policy forbids pushing from laptops; then V4
is mandatory in CI or as a maintainer-run step before merge.

---

## Post-merge watch {#post-merge}

Within 30 minutes of merge to `main`:

1. `Publish CueCode-IDE public shadow` — green; public tip commit message references CueInference SHA.
2. `cuecode_rebrand_check` (or successor) — green.
3. Spot-check public repo root still looks like an IDE checkout (not an `apps/` wrapper).
4. Dogfood: one engineer opens workspace + Plan E2E fixture.

If publish force-pushed a bad tree: fix forward with a corrected archive publish
**immediately**; do not leave public shadow broken overnight.

---

## Rollback {#rollback}

| Stage | Action |
|-------|--------|
| Before merge | Delete branch / close PR |
| After merge, before bad publish | Revert merge commit on `main`; ensure path filters restored |
| After bad publish | Re-run `script/publish-cuecode-ide` from a known-good `main` (or revert + publish); public remote is force-pushed by design — coordinate |

Do **not** attempt rollback via leaving a permanent root symlink “for compatibility.”

---

## PR sequence {#prs}

| PR | Phase | Title sketch |
|----|-------|--------------|
| A | N0 | docs: lock nest CueCode-IDE spec (ops/15) |
| B | N1–N2 | Nest CueCode-IDE under apps/ + publish/harness couplings |
| C | N3–N4 | Docs/grep gate + verify sign-off |

**Allowed:** B combines N1+N2.  
**Forbidden:** B without updated publish path filters.  
**Forbidden:** Combining with landing/labs deploy refactors.

### Per-PR definition of done {#dod}

- [x] `git mv` only (no copy/delete of the IDE tree)
- [x] Couplings C1–C8 updated in the mechanical PR
- [x] Verify matrix P0 green on the PR
- [x] `./script/sync-cursor-config --to-ide` if specs changed
- [x] Grep gate green before final merge
- [x] Link PR to phase anchors (`{#n0}` … `{#n4}`)
- [x] Update [migration status](#status)

---

## Agent execution notes {#agents}

When the user says **`Build nest CueCode-IDE phase N…`**:

1. Read this file end-to-end + confirm H0–H6 prerequisite.
2. Stay on a branch from latest `main`.
3. Prefer `required_permissions: ["all"]` for `git mv` / sync around `.cursor`.
4. Never rewrite `github.com/CueCard-AI/CueCode-IDE` URLs to an `apps/` path.
5. After path edits under `.cursor/specs/`, run `./script/sync-cursor-config --to-ide`.
6. Do not mark N4 done without fresh-clone evidence for V1–V12.

---

## Risk register {#risks}

| Risk | Severity | Mitigation |
|------|----------|------------|
| Wrong archive prefix | Critical | V2–V4; PR blocks on publish workflow path filter review |
| Symlink points at missing tree | Critical | V5–V6 |
| `cuecode-local` still looks one level up | Critical | V7; keep absolute `$HOME/CueInference/cloud/...` fallback |
| QA `../` still used | High | V8; update clone script echo |
| CI `working-directory` miss | High | V9; rebrand workflow on PR |
| Docs lag → agents use old cwd | Med | N3 grep gate; AGENTS hard rules |
| Temporary symlink forgotten | Med | N3 exit criterion `test ! -e CueCode-IDE` |
| Force-push bad public tree | Critical | Post-merge watch; rollback table |

---

## Migration status {#status}

| Phase | Work | Status |
|-------|------|--------|
| N0 | Spec lock + baseline + freeze | **Done** (2026-07-20) — baseline 5222 files; public tip `52da0bce` |
| N1 | `git mv` + publish/sync/CI/workspace | **Done** (2026-07-20) — no root symlink |
| N2 | Symlink, `cuecode-local`, QA paths | **Done** (2026-07-20) |
| N3 | Docs/specs mass rewrite + grep gate | **Done** (2026-07-20) |
| N4 | Fresh-clone verify + land | **Done** (2026-07-20) — PR #18 merged `b24c7ec9a2` |

---

## Acceptance {#acceptance}

- [x] Spec indexed in [ops/README](./README.md) and [00-README](../00-README.md)
- [x] Locked decisions recorded ([locked](#locked))
- [x] Coupling inventory + verify matrix written
- [x] N1–N4 complete with green CI
- [x] Public shadow publish green on `main` after land
- [x] No root was-`CueCode-IDE/` path remaining (except historical notes)

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-07-20 | Coupling table columns: Today/After → Before nest / After nest (current) |
| 2026-07-20 | Docs polish: status → archive; day-to-day paths → MONOREPO § nested IDE |
| 2026-07-20 | **N4 done:** fresh-clone V1–V12 signed off; PR #18 merged; post-merge publish + rebrand watched |
| 2026-07-20 | **N3 done:** mass rewrite filesystem `CueCode-IDE/` → `apps/CueCode-IDE/`; protect GitHub repo refs; grep gate clean; ops/14 cross-link |
| 2026-07-20 | **N2 done:** retarget `vendor/cuecode_chp`; `cuecode-local` + QA `../../cuecode-testing-repo`; harness README/AGENTS |
| 2026-07-20 | **N1 done:** `CueCode-IDE/` → `apps/CueCode-IDE/`; publish archive prefix + sync CANONICAL + rebrand/publish CI + workspace |
| 2026-07-20 | **N0 done:** baseline `git archive` count **5222**; public tip `52da0bce`; freeze + verify owner on PR #18; H4c–H6 still open as #17 (N1 gate) |
| 2026-07-20 | Initial locked spec for nesting `CueCode-IDE/` under `apps/` (phases N0–N4) |
