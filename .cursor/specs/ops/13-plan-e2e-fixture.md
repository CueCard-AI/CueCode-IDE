# Plan E2E fixture — PulseBoard QA repo {#plan-e2e-fixture}

> **Parent:** [ops/README](./README.md)  
> **Repo:** [CueCard-AI/cuecode-testing-repo](https://github.com/CueCard-AI/cuecode-testing-repo) (private)  
> **Seed in monorepo:** [`ops/qa-fixture/pulseboard/`](../../../ops/qa-fixture/pulseboard/) — edit here; sync with `./script/sync-qa-fixture`
> **Local clone (open in CueCode):** [`cuecode-testing-repo/`](../../../cuecode-testing-repo/) — `./script/clone-qa-fixture-repo`

CueCode Planning Hub **Implement**, ticket sessions, multi-language agent work, and Plan QA run against a **dedicated polyglot fixture** — not against the CueInference monorepo while building apps/CueCode-IDE.

---

## Roles (three layers) {#roles}

| Layer | Location | Purpose |
|-------|----------|---------|
| **Spec + playbook** | CueInference `.cursor/specs/ops/13-plan-e2e-fixture.md` (this doc) | Contract, QA script, generation waves |
| **Runnable fixture** | `CueInference/cuecode-testing-repo/` (clone) → GitHub `CueCard-AI/cuecode-testing-repo` | Open in CueCode; Implement; agent edits; yaml writes |
| **Unit tests** | `apps/CueCode-IDE/crates/cuecode_plans/tests/` | CI without cloning GitHub |

**CueInference `.cuecode/project.yaml`** remains the **real delivery roadmap** (phases 1.3, 1.4, 2.1…). Use Planning Hub here for **Build track smoke only** — do not click **Implement** on dogfood phases in this repo.

---

## Fictional product: PulseBoard {#pulseboard}

Team analytics dashboard — events ingested, aggregated, displayed. Polyglot monorepo:

| Stack | Path | Role |
|-------|------|------|
| **Rust** | `rust/pulse-core/` | Shared types + validation |
| **Go** | `go/services/api/` | HTTP API |
| **Python** | `python/workers/aggregator/` | Batch worker |
| **JS** | `js/apps/web/` | Minimal web shell |

Specs live under `.cursor/specs/` in the fixture repo. Build graph in `.cuecode/project.yaml`.

---

## Repo layout (target) {#layout}

```text
cuecode-testing-repo/
  README.md
  AGENTS.md
  BOOTSTRAP.md
  .cuecode/project.yaml
  .cursor/specs/
    product/vision.md
    phases/*.md
  rust/pulse-core/
  go/services/api/
  python/workers/aggregator/
  js/apps/web/
  scripts/verify-all.sh
```

Canonical seed copy: **`CueInference/ops/qa-fixture/pulseboard/`**.  
Day-to-day QA checkout: **`CueInference/cuecode-testing-repo/`** (fixed path — not `/tmp`).

---

## Setup (once per machine) {#setup}

```bash
cd /path/to/CueInference
./script/clone-qa-fixture-repo
```

Creates or updates **`cuecode-testing-repo/`** at the monorepo root (beside `apps/`).

Open in CueCode:

```bash
cd apps/CueCode-IDE/ && ./script/run-cuecode-dev ../../cuecode-testing-repo
```

Also listed in **`CueInference.code-workspace`** as a workspace folder after reload.

---

## Bootstrap phases (manifest v1) {#bootstrap-phases}

Initial `project.yaml` ships three build phases:

| id | Purpose |
|----|---------|
| `phase-00-repo-green` | `./scripts/verify-all.sh` passes (trivial seeds) |
| `phase-01-generate-stacks` | Agent expands Rust/Go/Python/JS per vision + specs |
| `phase-02-plan-hub-smoke` | Manual QA-P1-Plan steps 1–6 |

After wave 5 (agent-generated), manifest grows with stack-specific phases, `refs[]`, and `suggested_next`.

---

## Agent generation waves {#generation-waves}

Copy prompts from fixture `BOOTSTRAP.md` into Implement sessions **in the testing repo only**.

| Wave | Ticket | Outcome |
|------|--------|---------|
| 0 | (human seed) | Light tree committed; validate passes |
| 1 | `phase-rust-core` (add to manifest) | `pulse-core` + tests + `rust/core.md` |
| 2 | `phase-go-api` | HTTP handlers + `go/api.md` + `refs[]` |
| 3 | `phase-py-worker` | pytest worker |
| 4 | `phase-js-web` | minimal UI + test |
| 5 | hub / agent | Expand `project.yaml` graph (deps, suggested_next) |
| 6 | — | Harden `verify-all.sh`; optional GitHub Actions in fixture repo |

---

## QA-P1-Plan (20 min) {#qa-p1-plan}

Run in **CueCode** with **`cuecode-testing-repo/`** open:

```bash
cd CueInference/cuecode-testing-repo   # or: ./script/clone-qa-fixture-repo first
```

1. **Clone / open** fixture repo — not CueInference.
2. Planning Hub → **Build track** lists phases from `.cuecode/project.yaml`.
3. **Suggested next** and **Blocked** badges match deps.
4. **`cargo run -p cuecode_plans --bin cuecode-plan -- validate`** from fixture root (requires apps/CueCode-IDE on PATH or run from dev checkout with `--project-root` pointing at fixture).
5. **Implement** `phase-01-generate-stacks` (or next suggested) — thread opens, composer stub, **do not Send** if you only want yaml + pin check.
6. Hand-edit `project.yaml` → hub shows **manifest reloaded** banner.
7. Check exit `- [ ]` boxes in active phase md → **Mark phase done?** → yaml `status: done`.
8. `./scripts/verify-all.sh` green after agent work (waves 1–4).

**Pass:** steps 2, 3, 5 (without Send optional), 6, 7 when applicable.

**CueInference dogfood:** steps 2–3 only on this monorepo; defer 5–7 to fixture repo.

---

## Sync (bidirectional) {#sync}

Three layers: **seed** (`ops/qa-fixture/pulseboard/`, committed in CueInference) ↔ **local clone** (`cuecode-testing-repo/`) ↔ **GitHub** (`CueCard-AI/cuecode-testing-repo`).

| Direction | Command | Notes |
|-----------|---------|-------|
| Status | `./script/sync-qa-fixture --status` | Diff seed vs clone vs remote |
| Outbound | `./script/sync-qa-fixture --to-remote --push` | Seed → clone → GitHub; CI on `ops/qa-fixture/**` push to `main` |
| Inbound (local) | `./script/sync-qa-fixture --from-clone` | After Implement / agent QA — no `--delete` |
| Inbound (remote) | `./script/sync-qa-fixture --from-remote` | Clone GitHub → seed; weekly ingest workflow opens a PR |

**Loop after agent work in the fixture repo:**

1. `./script/sync-qa-fixture --from-clone`
2. Review `ops/qa-fixture/pulseboard/` → commit to CueInference
3. Push `main` → CI republishes to GitHub (or `./script/sync-qa-fixture --to-remote --push`)

**Safety:** outbound uses `rsync --delete`. If the clone has files not in the seed, publish fails until you ingest (`--from-clone`) or pass `--force` (data loss).

Legacy alias: `./script/publish-qa-fixture --push` → `--to-remote --push`.

### CI

| Workflow | Trigger | Action |
|----------|---------|--------|
| `publish_qa_fixture.yml` | Push to `main` changing `ops/qa-fixture/**` | Publish seed → GitHub |
| `ingest_qa_fixture_manual.yml` | Manual dispatch | PR: GitHub → `ops/qa-fixture/pulseboard/` |
| `ingest_qa_fixture_weekly.yml` | Weekly cron (Mon 14:00 UTC) | Same ingest as PR |

Secret: **`CUECODE_TESTING_REPO_PUBLISH_TOKEN`** (contents:write on `cuecode-testing-repo`).

---

## Publishing seed to GitHub {#publish}

Edit the seed under `ops/qa-fixture/pulseboard/`, then:

```bash
./script/sync-qa-fixture --to-remote --push
# or legacy:
./script/publish-qa-fixture --push
```

Or manually:

```bash
./script/clone-qa-fixture-repo
rsync -a --delete ops/qa-fixture/pulseboard/ cuecode-testing-repo/
cd cuecode-testing-repo && git add -A && git status
```

---

## Cross-links {#cross-links}

| Doc | Link |
|-----|------|
| Planning Hub design | [16-planning-hub §exit-criteria](../design/16-planning-hub.md#exit-criteria) |
| Build phase 1.4 | [1-4-planning-hub-manifest](../delivery/build-plans/phases/1-4-planning-hub-manifest.md) |
| Bootstrap sub-phase | [1-4a-qa-fixture-bootstrap](../delivery/build-plans/phases/1-4a-qa-fixture-bootstrap.md) |
| Roadmap QA | [07 §QA-P1-Plan](../delivery/07-implementation-roadmap.md#qa-p1-plan) |
| Testing strategy | [06 §testing](../core/06-system-design.md#testing) |

---

## Changelog {#changelog}

| Date | Change |
|------|--------|
| 2026-06-22 | Bidirectional sync: `sync-qa-fixture`, publish + ingest CI |
| 2026-06-22 | Initial fixture spec + monorepo seed path |
