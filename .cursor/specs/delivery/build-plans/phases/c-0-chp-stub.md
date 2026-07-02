# Build phase C.0 — CHP stub + one tool {#phase-c-0}

> **Invoke:** `Build phase C.0` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[~] In progress` |
| **Last verified** | Partial — `cuecode-cloud-m0` repo exists |
| **Duration** | ~3 weeks |
| **Track** | C — Cloud harness (parallel) |
| **Roadmap** | [Cloud M0](../07-implementation-roadmap#phase-3b) |
| **QA script** | M0 exit — CHP integration test |

## Deliverable {#phase-c-0-deliverable}

CHP v0 message types; stub `harness-api`; `cuecode_cloud` crate; `read_file` round-trip integration test.

## Depends / blocks {#phase-c-0-deps}

| | Phase |
|---|-------|
| **Depends on** | 2.1 |
| **Blocks** | C.1 |

## Out of scope {#phase-c-0-out-of-scope}

- Real model gateway, streaming (C.1)
- Subagent async cloud scheduler (C.2)

---

## Tasks {#phase-c-0-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| C.0.1 | CHP v0 message types | `crates/cuecode_cloud/`, [03-protocol](../../harness/cloud/03-protocol.md) | `[~]` |
| C.0.2 | Stub harness-api | private `cuecode-harness` / `cuecode-cloud-m0` repo | `[~]` |
| C.0.3 | `cuecode_cloud` crate + adapter | `crates/cuecode_cloud/` | `[~]` |
| C.0.4 | `read_file` round-trip integration test | `crates/cuecode_cloud/tests/` | `[ ]` |

---

## Implementation notes {#phase-c-0-impl}

- CHP loop: `SessionCreate` → `UserMessage` → `ToolRequest` → `ToolResult` → `TurnComplete`.
- Local dev: in-process stub server via feature flag `cloud_harness_stub`.
- Client renders tool card identically to local agent.
- Server rejects `edit_file` for explore `agent_type` in stub allowlist.
- No proprietary prompt bodies in `cuecode_sandbox` — types only.

---

## Verify {#phase-c-0-verify}

```bash
cd CueCode-IDE
cargo test -p cuecode_cloud
./script/clippy -p cuecode_cloud
```

---

## Exit criteria {#phase-c-0-exit}

- [ ] CHP `SessionCreate` → `UserMessage` → `ToolRequest` → `ToolResult` → `TurnComplete`
- [ ] Client renders tool card identically to local agent
- [ ] Server rejects `edit_file` for explore agent_type in stub allowlist
- [ ] `./script/clippy` clean on `cuecode_cloud`
- [ ] [08 §M0 exit](../../harness/cloud/08-roadmap.md#m0-exit) all rows

---

## QA {#phase-c-0-qa}

Manual steps before marking **Status** `[x]`:

1. Enable `cloud_harness_stub` — send prompt — stub returns fake assistant + read_file
2. Permission modal appears — approve — file contents return upstream
3. Turn completes — no real model spend
4. Integration test: mock HTTP server + read_file — green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — M0 exit — CHP integration test

---

## PR checklist {#phase-c-0-pr}

- [ ] PR title/body cites **Build phase C.0** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-c-0-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| CHP protocol | ../../harness/cloud/03-protocol.md |
| Cloud M0 | ../../harness/cloud/08-roadmap.md#m0 |
| Open client | ../../harness/cloud/04-open-client.md |
| Tool host | ../../harness/cloud/06-tool-host.md |

---

## Changelog {#phase-c-0-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Generated; partial M0 in cuecode-cloud-m0 |
