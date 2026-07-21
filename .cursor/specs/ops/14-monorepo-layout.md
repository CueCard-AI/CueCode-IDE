# Monorepo layout — hierarchy migration {#monorepo-layout}

> **Parent:** [ops/README](./README.md) · Spec index: [00-README](../00-README.md)
>
> **Status:** Spec locked. **H0–H6 done.** Migration complete.
>
> **Current state:** The GPL IDE lives at **`apps/CueCode-IDE/`**
> ([15-nest-cuecode-ide](./15-nest-cuecode-ide.md), **Done**). Day-to-day paths:
> [MONOREPO § Working in the nested IDE](../../../MONOREPO.md#working-in-the-nested-ide).
>
> **Scope:** CueInference root folder hierarchy only (shelves). This H0–H6
> program did **not** nest the GPL IDE or move `.cursor/` (agent discovery +
> public sync). Paths written as was-`CueCode-IDE/` mean “as locked during
> H0–H6.” Nesting was a **separate** follow-on: [15](./15-nest-cuecode-ide.md).
>
> **Related:** [MONOREPO.md](../../../MONOREPO.md) · [AGENTS.md](../../../AGENTS.md) ·
> [13-plan-e2e-fixture](./13-plan-e2e-fixture.md) ·
> [15-nest-cuecode-ide](./15-nest-cuecode-ide.md) ·
> [01-cuecloud-landing §repo-placement](../marketing/01-cuecloud-landing.md#repo-placement) ·
> [04-cuelabs](../marketing/04-cuelabs.md)

---

## One sentence {#one-sentence}

Reorganize the CueInference root so peer “apps” become intentional shelves
(`cloud/`, `sites/`, `hiring/`, `ops/`, `research/`) without breaking GPL
publish, live site deploys, harness builds, or Plan E2E fixture paths.

---

## Problem {#problem}

The monorepo root is a flat list of product trees, stubs, sites, fixtures,
experiments, and gitignored local clones. That makes discovery hard for humans
and agents, invites wrong edits in the wrong tree, and obscures the CueCloud
vs Cue Labs vs research mental model.

Noise that is **not** a product (but still appears in the explorer):

| Path | Status |
|------|--------|
| `claude-code-main/` | Gitignored local research |
| `cuecode-testing-repo/` | Gitignored QA clone (keep at root) |
| `**/node_modules`, `**/target`, `**/.next` | Build artifacts |

---

## Goals and non-goals {#goals}

### Goals

1. Root reads as **systems**, not a junk drawer (~8–12 intentional top-level entries).
2. Layout matches the company story: CueCloud product, public sites, hiring kit, ops, research.
3. Every filesystem move ships in the **same PR** as CI, scripts, docs, specs, and
   (when specs change) `./script/sync-cursor-config --to-ide`.
4. Zero downtime for live sites: remote paths `/opt/cuecloud-landing` and
   `/opt/cuelabs` stay; only **repo** paths change.
5. Explorer noise drops in Phase 0 via workspace + `files.exclude` **before** any `git mv`.

### Non-goals

- Split into multiple git remotes.
- Move was-`CueCode-IDE/` (now `apps/CueCode-IDE/`) in this program — see [15-nest-cuecode-ide](./15-nest-cuecode-ide.md).
- Rename product brands (`landing` → `cuecloud-site`, etc.) — optional later.
- Delete `cloud/orchestrator/` or `cloud/dashboard/` — relocate only.
- Nest `hiring/` under `sites/labs/` (kit ≠ site).

### Principles {#principles}

- One PR per phase (never mix sites + harness).
- Prefer `git mv` (history).
- Keep `script/` at monorepo root (ubiquitous `./script/…` + CI `bash script/…`).
- Keep `.cursor/` and `.agents/` at monorepo root.
- Move compose siblings together (`dashboard` + `orchestrator`).
- Hiring kit stays a first-class root (or later ops/research) — **not** inside `sites/labs`.

---

## Locked decisions {#locked}

| Decision | Lock |
|----------|------|
| Shelf names | `cloud/`, `sites/`, `hiring/`, `ops/`, `research/` |
| was `CueCode-IDE/` | Stayed at root for this program; nest → [15](./15-nest-cuecode-ide.md) `apps/CueCode-IDE/` |
| `script/` | Stays at root (no bury under `ops/` without shims) |
| `.cursor/`, `.agents/` | Stay at root |
| `hiring/` | Stays at root (public SMBH `git archive` source) |
| QA clone | `cuecode-testing-repo/` stays at root (adjacent to IDE) |
| Remote deploy dirs | Unchanged (`/opt/cuecloud-landing`, `/opt/cuelabs`) |
| Phase order | 0 → 1 → 2 → 4a/4b → 4c → 5 → 6 (see [phases](#phases)) |

**Open (non-blocking):** prefer `cuecloud/` instead of `cloud/` — default remains
`cloud/` unless product renames the shelf before Phase 4.

---

## Target hierarchy {#target}

> **Today’s tree** (after H0–H6 **and** nest N0–N4). During H0–H6 the IDE was
> still at root was-`CueCode-IDE/`; that is historical only.

```text
CueInference/
├── AGENTS.md, AGENTS_GUIDE.md, README.md, MONOREPO.md, CUECLOUD.md, …
├── CueInference.code-workspace
├── package.json                 # npm run cuecloud / cuelabs
├── script/                      # KEEP at root
├── .cursor/  .agents/  .github/  .cuecode/
│
├── apps/
│   └── CueCode-IDE/             # GPL archive source (was root; see [15](./15-nest-cuecode-ide.md))
├── cuecode-testing-repo/        # gitignored clone; KEEP at root
│
├── cloud/                       # CueCloud backend + consoles
│   ├── cuecode-harness/
│   ├── cue-control/
│   ├── cue-scheduler/
│   ├── dashboard/
│   └── orchestrator/            # sibling of dashboard (compose context)
│
├── sites/                       # public web
│   ├── landing/                 # cuecloud.io
│   └── labs/                    # cuelabs.cloud
│
├── hiring/                      # KEEP at root
│   └── small-model-big-harness/
│
├── ops/
│   └── qa-fixture/              # seed only; clone stays at root
│
└── research/
    ├── side-projects/
    │   └── inference-demo/
    └── claude-code-main/        # gitignored local research
```

---

## Immutable in this program {#immutable}

> H0–H6 treated the IDE path as immutable **for this program**. Nesting later
> moved it to `apps/CueCode-IDE/` under [15](./15-nest-cuecode-ide.md).

| Path | Why |
|------|-----|
| `apps/CueCode-IDE/` (was `CueCode-IDE/` at root for H0–H6) | GPL surface; nest history — [15](./15-nest-cuecode-ide.md) |
| `.cursor/`, `.agents/` | Agent discovery; `script/sync-cursor-config` |
| `CUECLOUD.md` | Private; never sync into `.cursor/` |
| `script/` | Root npm + CI entrypoints |
| `.github/` | Actions home |
| `cuecode-testing-repo/` location | Spec + `../cuecode-testing-repo` ergonomics |
| Remote `/opt/...` | Production layout independent of repo paths |

---

## Current root inventory {#inventory}

### Tracked product / infra dirs (current)

| Path | Role |
|------|------|
| `apps/CueCode-IDE/` | Desktop IDE (Zed fork); was `CueCode-IDE/` at root |
| `cloud/cuecode-harness/` | CueCloud agent runtime (CHP) |
| `cloud/cue-control/` | Fleet plane stub |
| `cloud/cue-scheduler/` | Decode scheduler stub |
| `cloud/dashboard/` | Usage / billing console prototype |
| `cloud/orchestrator/` | FastAPI front-end; **compose sibling** of dashboard — under-documented in MONOREPO/AGENTS |
| `sites/landing/` | Cue Cloud marketing (`cuecloud.io`) |
| `sites/labs/` | Cue Labs site (`cuelabs.cloud`) |
| `hiring/` | Hiring challenges (SMBH) |
| `ops/qa-fixture/` | Plan E2E PulseBoard seed |
| `research/side-projects/` | Experiments (`inference-demo/`) — moved in H1 |
| `script/` | Monorepo ops wrappers |

### Gitignored at root

| Path | Role |
|------|------|
| `claude-code-main/` | Local research |
| `cuecode-testing-repo/` | Local QA clone |

### Doc drift (resolved in H0 / H6)

- ~~`MONOREPO.md` CI table omits site deploy workflows~~ — fixed
- ~~`cloud/orchestrator/` missing from project tables~~ — fixed

---

## Blast radius {#blast-radius}

### Severity heatmap

| Move | Severity | Notes |
|------|----------|-------|
| Workspace + `files.exclude` | None | Phase 0 |
| `research/claude-code-main` | Low | `.gitignore` + research specs |
| `research/side-projects` | Med–High | Demo CI + **`strip_components`** |
| `sites/{landing,labs}` | **High** | Two prod deploys + npm depths + placement lock |
| Keep `hiring/` | — | By design |
| `cloud/{control,scheduler}` | Low | Docs |
| `cloud/{dashboard,orchestrator}` | Med | Compose sibling |
| `cloud/cuecode-harness` | **Critical** | Symlink + `cuecode-local` + workspace |
| `ops/qa-fixture` | High | All QA scripts + CI path filters |

### Hard couplings (must not break)

| Coupling | Detail |
|----------|--------|
| Harness ↔ IDE | `vendor/cuecode_chp` → `../../../apps/CueCode-IDE/crates/cuecode_chp` (was `../../../CueCode-IDE/...` pre-nest) |
| IDE harness discovery | `apps/CueCode-IDE/script/cuecode-local` + `$HOME/CueInference/cloud/cuecode-harness` |
| Dashboard ↔ orchestrator | `cloud/dashboard/docker-compose.yml` `context: ../orchestrator` |
| Sites npm | Root + nested `package.json` → `script/dev-site` relative depths |
| SMBH publish | `script/publish-smbh` `SOURCE_PATH=hiring/small-model-big-harness` |
| GPL publish | `git archive HEAD:apps/CueCode-IDE` + `sync-cursor-config --to-ide` (was `HEAD:CueCode-IDE`) |
| Inference demo CI | `strip_components: 3` for `research/side-projects/inference-demo/` |
| Landing placement | [01 §repo-placement](../marketing/01-cuecloud-landing.md#repo-placement) **Locked** at `sites/landing/` |

### CI workflows that embed paths

| Workflow | Paths |
|----------|-------|
| `deploy-landing.yml` | `sites/landing/**` |
| `deploy-cuelabs.yml` | `sites/labs/**` |
| `deploy-inference-demo.yml` | `research/side-projects/inference-demo/**` + `strip_components: 3` |
| `publish_smbh.yml` | `hiring/small-model-big-harness/**` |
| `publish_qa_fixture.yml` | `ops/qa-fixture/**` |
| `publish_cuecode_ide.yml` | `apps/CueCode-IDE/**` (was `CueCode-IDE/**` at root for H0–H6) |
| `cuecode_rebrand_check.yml` | `apps/CueCode-IDE/**` (was `CueCode-IDE/**` at root for H0–H6) |

---

## Phased delivery {#phases}

Say **`Build monorepo layout phase H0`** … **`H6`** when executing.
Do not skip ahead of incomplete prior phases.

### Phase H0 — Inventory truth + explorer hygiene {#h0}

**No `git mv`.**

| Work | Detail |
|------|--------|
| Docs | Add `cloud/orchestrator/` to `MONOREPO.md` / `AGENTS.md` tables |
| Docs | Add CI rows for `deploy-landing.yml`, `deploy-cuelabs.yml` |
| Docs | Add “Monorepo layout” section pointing at this spec + migration status table |
| Workspace | Expand `CueInference.code-workspace` settings; optional named folders |
| Exclude | `files.exclude` for `claude-code-main`, `cuecode-testing-repo`, `**/node_modules`, `**/target`, `**/.next` (repo `.vscode/settings.json` and/or workspace `settings`) |

**Exit criteria**

- [x] Explorer no longer treats gitignored research/QA clones as first-class apps
- [x] Docs list orchestrator + both site deploy workflows
- [x] This spec linked from `MONOREPO.md`
- [x] CI path filters unchanged

---

### Phase H1 — Research shelf {#h1}

**Moves**

```bash
mkdir -p research
git mv side-projects research/side-projects
# local-only:
# mv claude-code-main research/claude-code-main
```

**Same-PR checklist**

| Item | Action |
|------|--------|
| `deploy-inference-demo.yml` | Paths → `research/side-projects/inference-demo/**`; fix `working-directory`, `source`; set **`strip_components: 3`** |
| Docs | README, MONOREPO, AGENTS, RUNBOOK |
| Specs | Any `side-projects/inference-demo` links; then `sync-cursor-config --to-ide` |
| `.gitignore` | `research/claude-code-main/` |

**Exit criteria**

- [x] Grep clean for root `side-projects/` (except changelog / this spec)
- [x] Inference demo path updated (`research/side-projects/inference-demo`)
- [x] Deploy workflow path audit (`strip_components: 3`)

**Do not** combine with H2.

---

### Phase H2 — Sites shelf {#h2}

**Preflight (blocking)**

1. Unlock and rewrite [01-cuecloud-landing §repo-placement](../marketing/01-cuecloud-landing.md#repo-placement)
   from root `landing/` → `sites/landing/`.
2. Update [04-cuelabs](../marketing/04-cuelabs.md) app paths to `sites/labs/`.
3. Confirm local secrets/certs/tfstate move with the trees
   (`*/deploy/.env.secrets`, `certs/`, `*.tfstate` — gitignored).

**Moves**

```bash
mkdir -p sites
git mv landing sites/landing
git mv labs sites/labs
```

**Same-PR mechanical updates**

1. **CI** — both deploy workflows: path filters + every rsync/scp prefix.
2. **`script/dev-site`** — `$ROOT/sites/landing/apps/web`, `$ROOT/sites/labs/apps/web`.
3. **`package.json` depths** — each nested package gains one `../` toward root `script/`.
4. **Package-local deploy scripts** — re-verify `dirname` roots still resolve.
5. **Docs + specs** — all monorepo path mentions; relative links from
   `.cursor/specs/marketing/*`; then `./script/sync-cursor-config --to-ide`.
6. **Workspace** — optional folders for `sites/landing`, `sites/labs`.

**Exit criteria**

- [x] `npm run cuecloud` and `npm run cuelabs` resolve apps under `sites/`
- [x] Local deploy scripts still find package-root terraform + secrets
- [ ] Post-merge: both site deploy workflows green (origin SSH smoke)
- [ ] Public `https://cuecloud.io` and `https://cuelabs.cloud` still 200
- [x] No accidental move of `hiring/`

**Out of scope:** nesting `hiring/` under `sites/labs`.

---

### Phase H3 — Hiring (docs-only default) {#h3}

**Default:** keep `hiring/` at root. Document that choice in `MONOREPO.md`
(“kit is a public shadow source, not a website”).

If a future decision nests it, same PR must update:

- `script/publish-smbh` `SOURCE_PATH`
- `publish_smbh.yml` path filters
- [05-hiring-challenge](../marketing/05-hiring-challenge.md)

**Exit criteria (default path)**

- [x] MONOREPO/AGENTS state hiring stays at root by design

---

### Phase H4 — Cloud shelf {#h4}

#### H4a — Stubs {#h4a}

```bash
mkdir -p cloud
git mv cue-control cloud/cue-control
git mv cue-scheduler cloud/cue-scheduler
```

Docs only (`MONOREPO.md`, `CUECLOUD.md`, `AGENTS.md`, stub READMEs). No CI.

**Exit criteria**

- [x] Stubs under `cloud/`; docs paths updated

#### H4b — Dashboard + orchestrator (pair) {#h4b}

```bash
git mv dashboard cloud/dashboard
git mv orchestrator cloud/orchestrator
```

Confirm `cloud/dashboard/docker-compose.yml` `context: ../orchestrator` still
valid. Update orchestrator README paths. Naming note: FastAPI `cloud/orchestrator/`
≠ CueHarness “orchestrator” language in `CUECLOUD.md` — disambiguate in docs.

**Exit:** `docker compose config` from `cloud/dashboard` succeeds.

**Exit criteria**

- [x] Pair under `cloud/`; compose `context: ../orchestrator` valid
- [x] Docs disambiguate FastAPI package vs CueHarness orchestrator prose

#### H4c — Harness last (critical) {#h4c}

```bash
git mv cuecode-harness cloud/cuecode-harness
```

| Item | Action |
|------|--------|
| `vendor/cuecode_chp` | Retarget symlink for harness under `cloud/` (was `../../CueCode-IDE`; nest later → `../../../apps/CueCode-IDE/...`) |
| was-`CueCode-IDE/script/cuecode-local` | Sibling discovery + home absolute path |
| `CueInference.code-workspace` | Folder path |
| Harness README / Cargo path docs | Update |
| RUNBOOK / any `dev-harness` | Update |

**Exit criteria**

- [x] Symlink resolves
- [x] Harness builds (`cargo check` / API)
- [x] IDE `--harness` finds new path (`cuecode-local` discovery updated)
- [x] Workspace opens harness folder (`cloud/cuecode-harness`)

**Do not** ship H4c in the same PR as H2.

---

### Phase H5 — Ops shelf (`qa-fixture`) {#h5}

```bash
mkdir -p ops
git mv qa-fixture ops/qa-fixture
```

Update: `script/clone-qa-fixture-repo`, `sync-qa-fixture`, `ingest-qa-fixture-pr`,
`publish-qa-fixture`; `publish_qa_fixture.yml` (+ ingest workflows if needed);
[13-plan-e2e-fixture](./13-plan-e2e-fixture.md); README/AGENTS/RUNBOOK.

**Keep** clone at root `cuecode-testing-repo/` (gitignore anchor `/cuecode-testing-repo/`).

**Keep `script/` at root.** Root shims into `ops/script/` are a separate optional
phase and usually not worth it.

**Exit criteria**

- [x] `./script/sync-qa-fixture --status`
- [x] Clone still lands beside IDE (`cuecode-testing-repo/` at root)
- [x] Spec 13 paths accurate (`ops/qa-fixture/pulseboard/`)

---

### Phase H6 — Polish {#h6}

1. Root `README.md` tree = target layout.
2. Migration status table in `MONOREPO.md` → all complete.
3. Remove empty leftover dirs.
4. Optional: workspace multi-root by domain (Cloud / Sites / Hiring / Research).
5. Repo-wide grep for stale prefixes; fix stragglers.
6. Changelog rows on marketing specs if they still mention old paths in body copy.

**Exit criteria**

- [x] README tree matches target shelves
- [x] `MONOREPO.md` phase table all **Done**
- [x] No empty leftover root dirs (`landing/`, `qa-fixture/`, `cuecode-harness/`, …)
- [x] Stale-prefix audit clean for actionable docs/CI (skipped optional workspace regroup)
- [x] Marketing path body copy already on `sites/` / `hiring/` (no further changelog needed)

---

## PR sequence {#prs}

| PR | Phase | Title sketch |
|----|-------|--------------|
| A | H0 | docs + workspace exclude + layout target |
| B | H1 | `research/` shelf + inference-demo CI depth |
| C | H2 | `sites/{landing,labs}` + deploy/npm/specs |
| D | H4a–H4b | `cloud/` stubs + cloud/dashboard/orchestrator |
| E | H4c | `cloud/cuecode-harness` symlink migration |
| F | H5 | `ops/qa-fixture` |

Never stack **C + E**.

---

## Per-PR definition of done {#dod}

Copy into each migration PR body:

```markdown
## Path migration checklist
- [ ] git mv only (no copy/delete of trees)
- [ ] CI path filters updated
- [ ] Scripts updated (script/*, package.json depths)
- [ ] Sibling / symlink / compose paths updated
- [ ] Docs: README, AGENTS, MONOREPO, RUNBOOK
- [ ] Specs unlocked/updated; sync-cursor-config --to-ide (if specs changed)
- [ ] Workspace paths updated
- [ ] Grep for old prefix returns 0 actionable hits
- [ ] Local smoke commands recorded below
- [ ] Post-merge Actions workflows listed below green
```

---

## Risks {#risks}

1. **Landing placement lock** — Phase H2 must unlock [01 §repo-placement](../marketing/01-cuecloud-landing.md#repo-placement) deliberately.
2. **`strip_components` footgun** — H1 demo deploy mis-extracts if depth wrong.
3. **Dual `.cursor` trees** — forget `--to-ide` → public GPL shadow ships broken relative links.
4. **Harness symlink** — IDE looks fine until CHP vendor path breaks.
5. **Local secrets** — `git mv` moves working trees; confirm `.env.secrets` / certs / tfstate still beside terraform.
6. **Name collision** — FastAPI `cloud/orchestrator/` vs CueHarness “orchestrator” prose in `CUECLOUD.md`.
7. **Absolute home paths** — `cuecode-local` hardcodes `~/CueInference/cloud/cuecode-harness`.

---

## Agent checklist {#agent-checklist}

1. Read this file end-to-end before any `git mv`.
2. Execute **one** phase per PR unless the phase explicitly allows pairing (H4a+H4b only).
3. Prefer `git mv`; never leave duplicate trees.
4. After marketing/ops spec path edits: `./script/sync-cursor-config --to-ide`.
5. Do not move was-`CueCode-IDE/` (see [15](./15-nest-cuecode-ide.md)), `.cursor/`, or `script/` under this program.
6. Do not nest `hiring/` under `sites/labs/` / `sites/labs`.
7. Link the PR to the phase anchor (`{#h0}` … `{#h6}`).
8. Update the [migration status](#status) table when a phase ships.

---

## Migration status {#status}

| Phase | Work | Status |
|-------|------|--------|
| H0 | Docs + workspace exclude + layout pointer | **Done** (2026-07-20) |
| H1 | `research/` shelf | **Done** (2026-07-20) |
| H2 | `sites/{landing,labs}` | **Done** (2026-07-20) |
| H3 | Hiring stays at root (docs) | **Done** (2026-07-20) |
| H4a | `cloud/` stubs | **Done** (2026-07-20) |
| H4b | `cloud/{dashboard,orchestrator}` | **Done** (2026-07-20) |
| H4c | `cloud/cuecode-harness` | **Done** (2026-07-20) |
| H5 | `ops/qa-fixture` | **Done** (2026-07-20) |
| H6 | Polish + grep audit | **Done** (2026-07-20) |

---

## Acceptance {#acceptance}

- [x] Spec indexed in [ops/README](./README.md) and [00-README](../00-README.md)
- [x] Target hierarchy [locked](#locked) and documented in `MONOREPO.md`
- [x] Phase H0–H6 checklists complete with green CI where applicable
- [x] Live sites unaffected (repo path changes only)
- [x] GPL publish path left at was-`CueCode-IDE/` for this program (nest → [15](./15-nest-cuecode-ide.md))
- [x] Plan E2E clone still adjacent to IDE (hidden in explorer; open path for QA)

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-07-20 | Docs: current-state callout (`apps/CueCode-IDE/`); tree labeled post-nest; link MONOREPO § nested IDE |
| 2026-07-20 | **H6 done:** polish — README tree, migration status complete, stale-path audit |
| 2026-07-20 | **H5 done:** `qa-fixture` → `ops/qa-fixture`; sync/publish scripts + CI path filters; clone stays at root |
| 2026-07-20 | **H4c done:** `cuecode-harness` → `cloud/cuecode-harness`; retarget `vendor/cuecode_chp`; `cuecode-local` + workspace paths |
| 2026-07-20 | **H4a/b done:** `cue-control`/`cue-scheduler`/`dashboard`/`orchestrator` → `cloud/`; compose sibling intact; docs paths updated |
| 2026-07-20 | **H3 done:** document `hiring/` stays at root (shadow source ≠ site) |
| 2026-07-20 | **H2 done:** `landing/`+`labs/` → `sites/{landing,labs}`; deploy CI + `script/dev-site` + npm depths; placement lock → `sites/landing/` |
| 2026-07-20 | **H1 done:** `side-projects/` → `research/side-projects/`; local `claude-code-main` → `research/`; demo CI `strip_components: 3` |
| 2026-07-20 | **H0 done:** orchestrator in project tables; site deploy CI rows; workspace + `.vscode` `files.exclude`; removed `cuecode-testing-repo` workspace root |
| 2026-07-20 | Initial monorepo layout migration spec (phases H0–H6) from hierarchy plan |
