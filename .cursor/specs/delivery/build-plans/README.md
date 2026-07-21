# Build plans {#build-plans-index}

Executable phase-by-phase implementation plans for CueCode.

```text
Build phase 1.1    →  open phases/1-1-cuecode-specs.md ONLY
Build phase 3b.2   →  open phases/3b-2-notification-verdict.md ONLY
```

Each sub-phase is a **self-contained** PR batch: tasks with file paths, inline implementation notes, verify commands, exit criteria, and QA steps. **Do not** start implementation from chat summaries or the master index alone.

---

## Documents

| Doc | Purpose |
|-----|---------|
| **[phases/](./phases/)** | **39 sub-phase files — execution contract** |
| [00-master-build-plan](./00-master-build-plan.md) | Index, dependency graph, phase lookup table |
| [TEMPLATE-subphase](./TEMPLATE-subphase.md) | Mandatory sections for new sub-phases |

Regenerate: `python3 apps/CueCode-IDE/script/generate-build-phase-docs.py`

---

## Phase index {#phase-index}

**Status** lives in each sub-phase file header — update there first, then [07 §progress](../07-implementation-roadmap#progress).

| Phase | Status | Doc | Depends |
|-------|--------|-----|---------|
| 0.1 | `[x]` Done* | [0-1-identity](./phases/0-1-identity.md) | — |
| 0.2 | `[x]` Done | [0-2-cloud-decouple](./phases/0-2-cloud-decouple.md) | 0.1 |
| 0.3 | `[x]` Done | [0-3-packaging-qa](./phases/0-3-packaging-qa.md) | 0.2 |
| 1.1 | `[x]` Done | [1-1-cuecode-specs](./phases/1-1-cuecode-specs.md) | 0.3 |
| 1.2 | `[x]` Done | [1-2-agent-spec-integration](./phases/1-2-agent-spec-integration.md) | 1.1 |
| 1.3 | `[~]` Superseded | [1-3-spec-ui-stub](./phases/1-3-spec-ui-stub.md) | — → use 1.3-rev |
| 1.3-rev | `[x]` Done | [1-3-rev-planning-hub-v0](./phases/1-3-rev-planning-hub-v0.md) | 1.2 |
| 1.4 | `[x]` Done | [1-4-planning-hub-manifest](./phases/1-4-planning-hub-manifest.md) | 1.3-rev |
| 1.4a | `[x]` Done | [1-4a-qa-fixture-bootstrap](./phases/1-4a-qa-fixture-bootstrap.md) | 1.4 |
| 1.4b | `[x]` Done | [1-4b-plan-ui-integration](./phases/1-4b-plan-ui-integration.md) | 1.4 |
| 1.4c | `[x]` Done | [1-4c-plan-navigation-polish](./phases/1-4c-plan-navigation-polish.md) | 1.4b |
| 1.4d | `[x]` Done | [1-4d-surface-shortcuts-polish](./phases/1-4d-surface-shortcuts-polish.md) | 1.4c |
| 1.4e | `[x]` Done | [1-4e-agent-layout-multi-window](./phases/1-4e-agent-layout-multi-window.md) | 1.4d |
| 1.4f | `[x]` Done | [1-4f-layout-studio](./phases/1-4f-layout-studio.md) | 1.4e |
| 1.4h | `[x]` Done | [1-4h-agent-nav-column-split](./phases/1-4h-agent-nav-column-split.md) | 1.4f |
| 1.4i | `[x]` Done | [1-4i-split-column-resize-ux](./phases/1-4i-split-column-resize-ux.md) | 1.4h |
| 1.4j | `[x]` Done | [1-4j-inner-column-fixed-pixel-sizing](./phases/1-4j-inner-column-fixed-pixel-sizing.md) | 1.4i |
| 1.4k | `[x]` Done | [1-4k-side-column-row](./phases/1-4k-side-column-row.md) | 1.4j |
| 1.4l | `[x]` Done | [1-4l-side-column-layout-engine](./phases/1-4l-side-column-layout-engine.md) | 1.4k |
| 1.4m | `[x]` Done | [1-4m-layout-stabilization-recovery](./phases/1-4m-layout-stabilization-recovery.md) | 1.4l |
| 1.5 | `[ ]` | [1-5-spec-roots-pin-modes](./phases/1-5-spec-roots-pin-modes.md) | 1.4m |
| 1.6 | `[ ]` | [1-6-organize-with-ai](./phases/1-6-organize-with-ai.md) | 1.4 |
| 2.1 | `[ ]` | [2-1-intent-core](./phases/2-1-intent-core.md) | 1.6 |
| 2.2 | `[ ]` | [2-2-intent-ui](./phases/2-2-intent-ui.md) | 2.1 |
| 2.3 | `[ ]` | [2-3-composer-modes-agent-builder](./phases/2-3-composer-modes-agent-builder.md) | 2.1 (2.2 rec.) |
| 3.1 | `[ ]` | [3-1-checkpoint-store](./phases/3-1-checkpoint-store.md) | 2.2 |
| 3.2 | `[ ]` | [3-2-review-panel](./phases/3-2-review-panel.md) | 3.1 |
| 3b.1 | `[ ]` | [3b-1-background-spawn](./phases/3b-1-background-spawn.md) | 2.2 |
| 3b.2 | `[ ]` | [3b-2-notification-verdict](./phases/3b-2-notification-verdict.md) | 3b.1 |
| 4.1 | `[ ]` | [4-1-trust-store](./phases/4-1-trust-store.md) | 2.2 |
| 5.1 | `[ ]` | [5-1-multi-lane](./phases/5-1-multi-lane.md) | 3.2, 3b.2 |
| 5.2 | `[ ]` | [5-2-composer-polish](./phases/5-2-composer-polish.md) | 5.1 |
| 6.1 | `[ ]` | [6-1-release-docs](./phases/6-1-release-docs.md) | 5.2 |
| 6.2 | `[ ]` | [6-2-competitive-gate](./phases/6-2-competitive-gate.md) | 6.1 |
| C.0 | `[~]` Partial | [c-0-chp-stub](./phases/c-0-chp-stub.md) | 2.1 |
| C.1 | `[ ]` | [c-1-streaming-cloud](./phases/c-1-streaming-cloud.md) | C.0 |
| C.2 | `[ ]` | [c-2-subagent-async](./phases/c-2-subagent-async.md) | C.1, 3b.1 |
| C.3 | `[ ]` | [c-3-verdict-hybrid](./phases/c-3-verdict-hybrid.md) | C.2, 3b.2 |
| C.4 | `[ ]` | [c-4-byok-enterprise](./phases/c-4-byok-enterprise.md) | C.3, 4.1 |

\*0.1 task 0.1.7 custom dock icon still `[ ]` — see [0-1-identity](./phases/0-1-identity.md).

\*1.4f title-bar Layout icon still deferred — see [1-4f-layout-studio](./phases/1-4f-layout-studio.md) task 1.4f.8.

**Next recommended:** [Build phase 1.5](./phases/1-5-spec-roots-pin-modes.md) (spec roots + pin modes). Layout track 1.4k–1.4m closed 2026-07-02 — see [07 §1.4m](../07-implementation-roadmap.md#phase-1-4m-layout-stabilization).

---

## Polish backlog (return later) {#polish-backlog}

Not blocking Track 1. Pick up when polishing docs or pre-release.

| Item | Status | Where to start |
|------|--------|----------------|
| **Doc screenshots (10)** | Placeholders in repo; need **CueCode** re-capture | [Pass D CDN backlog §A](../../core/03-pass-d-docs-cdn-backlog.md#a-app-screenshots--10-total-capture--rehost) |
| **0.1.7 custom dock icon** | Open | [0-1-identity](./phases/0-1-identity.md) task 0.1.7 |
| **Auto-update E2E smoke** | Playbook only | [03-pass-c-auto-update-smoke-test](../../core/03-pass-c-auto-update-smoke-test.md) |

### Doc screenshots — quick reference {#doc-screenshots-backlog}

Mechanical URL/path sweep is **done** (0.3.6). Images on disk are **upstream placeholders** until you re-capture in CueCode (title bar must say **CueCode**).

| What | Path in repo | Referenced from |
|------|--------------|-----------------|
| 7× project panel PNGs | `apps/CueCode-IDE/docs/src/assets/images/project-panel/` | `docs/src/project-panel.md` |
| 3× Instruments WEBPs | `apps/CueCode-IDE/docs/src/assets/images/troubleshooting/` | `docs/src/troubleshooting.md` |
| Asset README | `apps/CueCode-IDE/docs/src/assets/README.md` | — |

**Full shot list** (filename + what to show): [03-pass-d-docs-cdn-backlog.md §A](../../core/03-pass-d-docs-cdn-backlog.md#a-app-screenshots--10-total-capture--rehost)

**Verify no CDN regressions:**

```bash
cd apps/CueCode-IDE
./script/rebrand-check.sh   # includes docs zed.dev / images.zed.dev gates
rg 'images\.zed\.dev|cdn\.zed\.dev' docs/   # expect zero hits
```

Also tracked in [07 §progress optional](../07-implementation-roadmap#progress-phase-0) and [Build phase 0.3 follow-up](./phases/0-3-packaging-qa.md#phase-0-3-follow-up).

---

## Related specs

| Doc | Role |
|-----|------|
| [07-implementation-roadmap](../07-implementation-roadmap) | Product phases 0–6 — **[Progress checklist](../07-implementation-roadmap#progress)** |
| [03-fork-and-rebrand](../../core/03-fork-and-rebrand) | Rebrand tiers, ERR scenarios |
| [parity/15-competitive-parity](../../parity/15-competitive-parity.md) | Claude Code parity P0–P4 |
| [harness/cloud/08-roadmap](../../harness/cloud/08-roadmap.md) | Cloud C.0–C.4 detail |

---

## Critical path

```
0.1 → 0.2 → 0.3 → 1.1 → 1.2 → 1.3-rev → 1.4 → 1.6 → 2.1 → 2.2 → 3.1 → 3.2 → 5.1 → 6.1
```

Parallel after 1.4: **1.5** (spec roots + pin modes) · `3b.*` after 2.2 · `4.1` after 2.2 · `C.*` after 2.1
