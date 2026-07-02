# Dev, test, run, and deploy — agent E2E runbook {#dev-and-deploy}

> **Branch:** [harness/cloud/](./README.md) — Model B local development and M0 implementation.  
> **Status:** Implemented in public repo (M0). Private `cuecode-harness` is a sibling repo at `~/CueInference/cuecode-harness`.

This document is the **single execution runbook** for agents and engineers. Follow steps in order.
Cross-links: [03-protocol](./03-protocol.md) · [04-open-client](./04-open-client.md) · [08-roadmap §M0](./08-roadmap.md#m0)

---

## Mission {#mission}

Ship **M0**: CHP round-trip + one `read_file` tool through GPL `harness-stub` and `cuecode_cloud`.

| Layer | Crate / path | License |
|-------|--------------|---------|
| CHP types | `crates/cuecode_chp` | GPL |
| CHP client | `crates/cuecode_cloud` | GPL |
| Stub server | `crates/harness_stub` (`harness-stub` bin) | GPL |
| Private orchestration | `~/CueInference/cuecode-harness/services/harness-api` | Proprietary |

---

## Prerequisites {#prerequisites}

```bash
# From CueCode-IDE root (public GPL repo)
test -d crates && test -f Cargo.toml
rustc --version
cargo --version
```

Optional: `foreman`, `docker`, `curl`.

---

## Repository layout {#repo-layout}

Canonical meta folder:

```
~/CueInference/
├── README.md                # platform overview
├── CueCode-IDE/             # public GPL repo
│   ├── crates/cuecode_chp/
│   ├── crates/cuecode_cloud/
│   ├── crates/harness_stub/
│   ├── script/cuecode-local
│   ├── Procfile.harness
│   └── docker-compose.harness-dev.yml
└── cuecode-harness/         # private harness API
    ├── vendor/cuecode_chp/  # symlink → ../CueCode-IDE/crates/cuecode_chp
    ├── services/harness-api/
    └── script/dev-harness
```

If `CueCode-IDE` is checked out elsewhere, `script/cuecode-local --harness` checks sibling `../cuecode-harness` and `~/CueInference/cuecode-harness`.

---

## Run modes {#run-modes}

| Mode | Command | Harness |
|------|---------|---------|
| **0 — Unit** | `cargo test -p cuecode_chp -p cuecode_cloud -p harness_stub` | In-process / fake |
| **1 — Stub** | `script/cuecode-local --stub` | GPL `harness-stub` on `:8787` |
| **2 — Stack** | `script/cuecode-local --harness` or start `~/CueInference/cuecode-harness` + `script/cuecode-local` | Private `harness-api` |
| **3 — Local** | `script/cuecode-local --local` | NativeAgent |

---

## Environment {#environment}

Precedence: `CUECODE_*` env → `~/.config/cuecode/settings.json` → defaults.

| Variable | Default (dev) |
|----------|---------------|
| `CUECODE_HARNESS_URL` | `ws://127.0.0.1:8787/v1/chp/connect` |
| `CUECODE_AGENT_RUNTIME` | `local` (IDE); use `cloud` with stub/stack |
| `HARNESS_PORT` | `8787` |

Copy `.env.example` to `.env` for local overrides.

---

## Step 0 — Verify workspace {#step-0}

**Gate:**

```bash
cargo test -p cuecode_chp -p harness_stub -p cuecode_cloud
```

All tests must pass before proceeding.

---

## Step 1 — `cuecode_chp` {#step-1-chp}

**Path:** `crates/cuecode_chp/`

- `src/envelope.rs` — `ChpEnvelope`, `CHP_VERSION`
- `src/message_type.rs` — `session.start`, `tool.request`, etc.
- `src/messages.rs` — builders for M0 flow
- `fixtures/*.json` — golden JSON
- `tests/fixtures_roundtrip.rs`

**Gate:** `cargo test -p cuecode_chp`

---

## Step 2 — `harness_stub` {#step-2-stub}

**Path:** `crates/harness_stub/`

- `src/engine.rs` — M0 state machine (session → turn → tool → end)
- `src/server.rs` — axum `/health`, `/v1/chp/connect` WebSocket
- `src/main.rs` — binary `harness-stub`

**Run manually:**

```bash
cargo run -p harness_stub --bin harness-stub
curl -sf http://127.0.0.1:8787/health
# {"status":"ok","chp_version":"1.0"}
```

**Gate:** `cargo test -p harness_stub`

---

## Step 3 — `cuecode_cloud` {#step-3-client}

**Path:** `crates/cuecode_cloud/`

- `src/chp/client.rs` — `ChpClient`, `run_m0_roundtrip`
- `src/settings.rs` — env helpers
- `tests/m0_roundtrip.rs` — integration vs stub server

**Gate:**

```bash
cargo test -p cuecode_cloud
./script/clippy -p cuecode_cloud
```

**M1+ (not M0):** `CloudAgentConnection` implementing `acp_thread::AgentConnection`.

---

## Step 4 — `script/cuecode-local` {#step-4-script}

```bash
script/cuecode-local --help
script/cuecode-local --stub      # GPL harness-stub + zed (cloud runtime env)
script/cuecode-local --harness   # sibling cuecode-harness + zed
script/cuecode-local --local       # NativeAgent only
```

With harness already running:

```bash
CUECODE_HARNESS_URL=ws://127.0.0.1:8787/v1/chp/connect script/cuecode-local
```

---

## Step 5 — Foreman {#step-5-foreman}

```bash
foreman start -f Procfile.harness
```

Requires `foreman` (`brew install foreman` on macOS).

---

## Step 6 — Docker stub {#step-6-docker}

```bash
docker compose -f docker-compose.harness-dev.yml up -d --build
curl -sf http://127.0.0.1:8787/health
CUECODE_HARNESS_URL=ws://127.0.0.1:8787/v1/chp/connect script/cuecode-local
docker compose -f docker-compose.harness-dev.yml down
```

Dockerfile: `crates/harness_stub/Dockerfile`

---

## Step 7 — Private repo {#step-7-private}

**Path:** `~/CueInference/cuecode-harness/`

```bash
cd ~/CueInference/cuecode-harness
cp .env.example .env
script/dev-harness
curl -sf http://127.0.0.1:8787/health
# {"status":"ok","service":"harness-api","chp_version":"1.0"}
```

**Gate:** `cargo test -p harness-api -p cuecode_chp`

CHP types come from `vendor/cuecode_chp` (symlink to the GPL crate). For CI without a sibling checkout, use git + `[patch]` (see `cuecode-harness/README.md`).

Deploy tags: `harness-staging`, `harness-production` via `script/deploy-harness`.

Legacy template notes: `docs/cuecode-harness-template/README.md`

---

## Step 8 — CI {#step-8-ci}

Workflow: `.github/workflows/cuecode-cloud-m0.yml`

```bash
cargo test -p cuecode_chp -p harness_stub -p cuecode_cloud
./script/clippy -p cuecode_cloud
```

---

## M0 acceptance {#m0-acceptance}

| # | Criterion | Verify |
|---|-----------|--------|
| 1 | CHP fixtures parse | `cargo test -p cuecode_chp` |
| 2 | Stub engine unit test | `cargo test -p harness_stub` |
| 3 | Client ↔ stub integration | `cargo test -p cuecode_cloud` |
| 4 | `/health` on :8787 | `curl` after `cargo run -p harness_stub --bin harness-stub` |
| 5 | `script/cuecode-local --stub` or `--harness` starts | manual smoke |
| 6 | No proprietary prompts in GPL tree | review |
| 7 | CI workflow present | `.github/workflows/cuecode-cloud-m0.yml` |

**CHP M0 message sequence:**

```
session.start → session.started → turn.start → turn.stream → tool.request
→ tool.result → turn.end
```

---

## Troubleshooting {#troubleshooting}

| Symptom | Fix |
|---------|-----|
| `Connection refused :8787` | Run `--stub` or start harness manually |
| Tests flake on port bind | Set `HARNESS_PORT=8788` |
| `VERSION_MISMATCH` | Align `CHP_VERSION` in client and server |
| Docker build slow | Expected — full workspace copy in Dockerfile |

---

## Agent handoff prompt {#agent-handoff}

```
Read .cursor/specs/harness/cloud/09-dev-and-deploy.md.
Verify M0 gates. If failing, fix crates/cuecode_chp, cuecode_cloud, harness_stub.
Do not add proprietary orchestration to the GPL tree.
```

---

## Document status {#document-status}

| Field | Value |
|-------|-------|
| Status | Implemented (M0) |
| Last verified | 2026-06-17 |
| Next milestone | M1 — `CloudAgentConnection` + model gateway ([08-roadmap](./08-roadmap.md#m1)) |
