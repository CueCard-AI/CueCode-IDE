# Build phase 1.4 — Manifest + Build track + Implement (P-H1) {#phase-1-4}

> **Invoke:** `Build phase 1.4` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Complete — QA-P1-Plan on PulseBoard; polish in [1.4b](./1-4b-plan-ui-integration.md) |
| **Last verified** | 2026-06-23 |
| **Duration** | 5–8 days |
| **Track** | 1 — Planning Hub |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [16-planning-hub §P-H1](../../design/16-planning-hub.md#delivery-phases) |
| **QA script** | QA-P1 (extended) |

## Deliverable {#phase-1-4-deliverable}

**`cuecode_plans`** crate loads hub-compiled `.cuecode/plans/project.yaml` (v1 alias: `.cuecode/project.yaml`). Plan shows **Build track**, **Implement ticket session** (primary + `refs[]` + `in_progress`), checkbox rollup + **Mark phase done?** Hub is the **only** yaml writer.

> **UI integration (Plan tab + detached window):** [1.4b](./1-4b-plan-ui-integration.md) — depends on this phase for manifest + Implement.

## Depends / blocks {#phase-1-4-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.3-rev |
| **Blocks** | 1.4b, 1.5, 1.6, 2.1 |

## Out of scope {#phase-1-4-out-of-scope}

- Organize kanban + recon agent (1.6)
- `.cuecode/specs` multi-root prefer (1.5) — pointer paths to `.cursor/specs/` OK
- Pin modes beyond path link (1.5)

---

## Tasks {#phase-1-4-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.4.1 | Create `crates/cuecode_plans/` — `ProjectManifest`, `Artifact`, `Ref`, `validate()`, `resolve_path()` | `crates/cuecode_plans/` | `[x]` |
| 1.4.2 | Load/watch `plans/project.yaml` (alias root `.cuecode/project.yaml`); out-of-sync banner | `crates/cuecode_plans/`, Plan UI | `[x]` |
| 1.4.3 | Hub **Build track** tab from manifest (`kind: build_phase`, deps, status) | `crates/agent_ui/src/planning_hub.rs` | `[x]` |
| 1.4.4 | **Implement ticket session** → thread, `active_ticket_id`, `refs[]` bundle, yaml `in_progress`, composer stub | `crates/agent_ui/`, `crates/acp_thread/` | `[x]` |
| 1.4.4a | Parse `refs[]` on artifacts; resolve spec paths for Implement bundle | `crates/cuecode_plans/` | `[x]` |
| 1.4.5 | Checkbox rollup from phase md; **Mark phase done?** when all checked → yaml `status: done` | `crates/cuecode_plans/` | `[x]` |
| 1.4.6 | `build_track.suggested_next` in hub UI | `crates/cuecode_plans/` | `[x]` |
| 1.4.7 | Dogfood: check in / load repo-root `.cuecode/project.yaml` (pointer-only artifacts) | `.cuecode/project.yaml` (repo root) | `[x]` |
| 1.4.8 | `cuecode plan validate` CLI stub for CI | `crates/cli/` or `crates/cuecode_plans/` | `[x]` |
| 1.4.9 | Manifest summary block in system prompt (cheap) | `crates/agent/templates/` | `[x]` |

---

## Implementation notes {#phase-1-4-impl}

- Progress model: [16 §progress](../../design/16-planning-hub.md#progress) — yaml upstream, checkboxes downstream.
- Implement bundle: [16 §implement-bundle](../../design/16-planning-hub.md#implement-bundle).
- Governance: [16 §manifest-governance](../../design/16-planning-hub.md#manifest-governance).
- Session: `active_ticket_id` + `pinned_bundle`; bridge from `active_spec_path` at Implement.
- On-disk: [16 §cuecode-layout](../../design/16-planning-hub.md#cuecode-layout).

---

## Verify {#phase-1-4-verify}

```bash
cd CueCode-IDE
cargo build -p cuecode_plans -p agent_ui -p cuecode
cargo test -p cuecode_plans
# Hub smoke: Build track on CueInference .cuecode/project.yaml (no Implement)
# Implement E2E: QA-P1-Plan on cuecode-testing-repo — see ops/13-plan-e2e-fixture.md
```

---

## Exit criteria {#phase-1-4-exit}

- [x] [16-planning-hub E2, E6, E7](../../design/16-planning-hub.md#exit-criteria) — E2/E7 on [PulseBoard fixture](../../ops/13-plan-e2e-fixture.md), not this repo
- [x] Hub writes yaml; hand-edit shows reload banner (fixture repo or controlled test)
- [x] **Phase 1 agent loop** usable via [1.4a](./1-4a-qa-fixture-bootstrap.md) seed

---

## QA {#phase-1-4-qa}

1. Build track lists phases from `.cuecode/project.yaml` (**CueInference** — smoke)
2. **Implement phase** — run on [cuecode-testing-repo](../../ops/13-plan-e2e-fixture.md), not CueInference
3. Complete checkboxes → confirm mark done → yaml updates (**fixture repo**)
4. `cuecode plan validate` passes on dogfood manifest + fixture seed (`qa-fixture/pulseboard/`)

---

## Deep specs {#phase-1-4-specs}

| Topic | Doc |
|-------|-----|
| `project.yaml` schema | [16 §project-yaml](../../design/16-planning-hub.md#project-yaml) |
| `refs[]` + Implement bundle | [16 §implement-bundle](../../design/16-planning-hub.md#implement-bundle) |
| Plan UI (1.4b) | [16 §plan-ui](../../design/16-planning-hub.md#plan-ui) |
| Harness | [16 §harness](../../design/16-planning-hub.md#harness) |
| QA fixture | [ops/13-plan-e2e-fixture](../../ops/13-plan-e2e-fixture.md) · [1.4a](./1-4a-qa-fixture-bootstrap.md) |

---

## Changelog {#phase-1-4-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial sub-phase doc |
