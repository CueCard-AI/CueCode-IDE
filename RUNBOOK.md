# CueCode RUNBOOK

How to run **CueCode IDE** end to end. This file is **GPL-safe** — it covers only
the IDE, the in-tree harness stub, the Plan E2E fixture, and Cursor config sync.

| Context | This directory is… |
|---------|-------------------|
| CueInference monorepo | `apps/CueCode-IDE/` (monorepo root = `../..`) |
| Public shadow ([CueCard-AI/CueCode-IDE](https://github.com/CueCard-AI/CueCode-IDE)) | repo root |

Full monorepo journeys (dashboard, inference demo, private harness, sites, SMBH)
live in the private repo: `CueInference/RUNBOOK.md` § End-to-end journeys (J0–J9).

**30 minutes:** [J0](#j0--first-ide-build) then [J1](#j1--plan-e2e-fixture) or [J2](#j2--cloud-agent-harness-stub).

---

## Ports

| Port | Service |
|------|---------|
| — | CueCode desktop (GPUI) |
| `8787` | `harness-stub` (cloud agent path) |

---

## J0 — First IDE build {#j0--first-ide-build}

**Goal:** Launch CueCode from this package.

**Prereqs:** Rust via rustup; platform deps (macOS: Xcode CLT + `cmake`). First
build is slow (thousands of crates).

**Terminals:** 1

```bash
# CueInference monorepo:
cd apps/CueCode-IDE
# Public shadow: already at repo root

./script/run-cuecode-dev
# or: cargo run -p cuecode --bin cuecode
```

**Done when:** CueCode window opens; config paths are CueCode (not Zed).

**Fails:**

| Symptom | Fix |
|---------|-----|
| Linker / cmake errors | Install platform deps (see package README) |
| Monorepo skills missing in Cursor | Open `CueInference.code-workspace` at the monorepo root |

---

## J1 — Plan E2E fixture {#j1--plan-e2e-fixture}

**Goal:** Open CueCode against the PulseBoard QA fixture for Planning Hub dogfood.

### CueInference monorepo

```bash
# from monorepo root
./script/clone-qa-fixture-repo
cd apps/CueCode-IDE
./script/run-cuecode-dev ../../cuecode-testing-repo
```

Optional seed check: `./ops/qa-fixture/pulseboard/scripts/verify-all.sh`

### Public shadow

Clone the published fixture beside this repo (or wherever you keep QA trees):

```bash
git clone https://github.com/CueCard-AI/cuecode-testing-repo.git ../cuecode-testing-repo
cd /path/to/CueCode-IDE   # this package
./script/run-cuecode-dev ../cuecode-testing-repo
```

**Done when:** CueCode opens the PulseBoard fixture tree (not a random folder).

**Fails:** Wrong relative path to the clone; re-check the `run-cuecode-dev` argument.

---

## J2 — Cloud agent (harness-stub) {#j2--cloud-agent-harness-stub}

**Goal:** Run `CUECODE_AGENT_RUNTIME=cloud` against the **GPL in-tree stub** (no
private CueHarness required).

**Terminals:** 2

```bash
# Terminal A — from this package
cargo run -p harness_stub --bin harness-stub
curl -sf http://127.0.0.1:8787/health

# Terminal B
export CUECODE_AGENT_RUNTIME=cloud
export CUECODE_HARNESS_URL=ws://127.0.0.1:8787/v1/chp/connect
cargo run -p cuecode --bin cuecode
# helper: ./script/cuecode-local --stub
```

**Done when:** `/health` on `:8787` is green; IDE cloud mode can talk to the stub.

**Fails:**

| Symptom | Fix |
|---------|-----|
| Connection refused | Start stub before the IDE |
| Still local NativeAgent | Export both env vars before launch |

---

## J9 — Edit specs → sync → reload {#j9--edit-specs--sync--reload}

**Goal:** Refresh Cursor after changing rules/skills/specs.

### CueInference monorepo

Edit at **repo-root** `.cursor/`, then:

```bash
# from monorepo root
./script/sync-cursor-config --to-ide
# Cursor: reload window
```

If you edited only `apps/CueCode-IDE/.cursor/`:

```bash
./script/sync-cursor-config   # IDE → root
```

### Public shadow

Edit `.cursor/` in this repo; reload Cursor. Publish/sync from CueInference is
separate (maintainers).

**Done when:** New rule/skill/spec is visible after reload.

---

## QA gates (before IDE PRs)

```bash
# from this package
./script/rebrand-check.sh
./script/clippy -p workspace -p agent_ui   # or crates you touched
# heavier: ./script/qa-p0.sh
```

---

## See also

- [`README.md`](./README.md) — quickstart + BYOK
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — IDE contributing norms
- [`.cursor/specs/00-README.md`](./.cursor/specs/00-README.md) — spec index
