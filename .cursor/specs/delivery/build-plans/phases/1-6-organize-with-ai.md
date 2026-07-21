# Build phase 1.6 — Organize with AI (P-H3) {#phase-1-6}

> **Invoke:** `Build phase 1.6` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 10–15 days |
| **Track** | 1 — Planning Hub |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [16-planning-hub §P-H3](../../design/16-planning-hub.md#delivery-phases) |
| **QA script** | QA-P1 full + Organize scenario |

## Deliverable {#phase-1-6-deliverable}

**Full Organize pipeline:** Layer 1 Rust recon → Layer 2 hub kanban board → Layer 3 read-only structure agent → **Accept plan** writes `project.yaml`. Incremental re-scan + partial accept. **Phase 1 complete** for Planning Hub alpha.

## Depends / blocks {#phase-1-6-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.4 |
| **Blocks** | 2.1 |
| **Recommended after** | 1.5 (not hard — pointers to `.cursor/specs` OK for Organize v1) |

## Out of scope {#phase-1-6-out-of-scope}

- Plan builder chat (P-H6)
- README docs-map sync (P-H6)
- `cursor-compat` export (P-H4)

---

## Tasks {#phase-1-6-tasks}

### Layer 1 — Rust recon

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.6.1 | Scan `**/*.md`, README, `.cursor/**`, gitignore, size caps | `crates/cuecode_plans/src/recon.rs` | `[ ]` |
| 1.6.2 | Heuristics: frontmatter, checkboxes, "Build phase", "Exit criteria" → pre-label + confidence | `crates/cuecode_plans/src/recon.rs` | `[ ]` |
| 1.6.3 | Incremental scan: new/changed since last `project.yaml` `adopted_at` | `crates/cuecode_plans/` | `[ ]` |

### Layer 2 — Hub board

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.6.4 | Organize tab: columns Product · Build · Spec · Docs · Ignore · Unsorted | `crates/agent_ui/src/planning_hub.rs` | `[ ]` |
| 1.6.5 | Drag-drop, merge duplicates, edit titles, **Explain why** chip | `crates/agent_ui/src/planning_hub.rs` | `[ ]` |
| 1.6.6 | Partial accept (e.g. Build track only) | `crates/agent_ui/`, `cuecode_plans/` | `[ ]` |
| 1.6.7 | Draft manifest in memory until **Accept plan** — hub writes yaml | `crates/cuecode_plans/` | `[ ]` |
| 1.6.8 | Re-organize merges into existing manifest (no delete user artifacts) | `crates/cuecode_plans/` | `[ ]` |

### Layer 3 — Structure agent

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.6.9 | Read-only Organize agent profile (read/grep/list only) | `crates/agent/`, `crates/cuecode_sandbox/` or tool wall | `[ ]` |
| 1.6.10 | Classify ambiguous bucket → column assignments | Organize session + hub | `[ ]` |
| 1.6.11 | Propose `depends_on`, build order, artifact ids for Build column | Organize session + hub | `[ ]` |
| 1.6.12 | Wire **Organize this project** empty-state CTA → pipeline | `crates/agent_ui/src/planning_hub.rs` | `[ ]` |

---

## Implementation notes {#phase-1-6-impl}

- Three layers: [16 §organize-pipeline](../../design/16-planning-hub.md#organize-pipeline).
- No filesystem writes until Accept; Promote copies on explicit action only.
- Agent spec (future): `agent/15-plan-recon.md`.

---

## Verify {#phase-1-6-verify}

```bash
cd apps/CueCode-IDE
cargo build -p cuecode_plans -p agent_ui -p cuecode
cargo test -p cuecode_plans
# Manual: repo without project.yaml → Organize → board → Accept → yaml written
# Manual: partial accept Build only
# Manual: re-organize adds candidates without deleting existing artifacts
```

---

## Exit criteria {#phase-1-6-exit}

- [ ] [16-planning-hub E1](../../design/16-planning-hub.md#exit-criteria)
- [ ] **Phase 1 complete** — Planning Hub alpha (see [07 §phase-1-exit](../07-implementation-roadmap#phase-1-exit))
- [ ] Organize CTA from hub empty state works end-to-end

---

## QA {#phase-1-6-qa}

1. Scattered-md fixture repo → Organize → Accept → `project.yaml` + Build track
2. Low-confidence files land in Unsorted
3. Structure agent cannot write files or run terminal
4. Skip dismisses empty-state nag (persisted)

---

## Deep specs {#phase-1-6-specs}

| Topic | Doc |
|-------|-----|
| Organize pipeline | [16 §organize-pipeline](../../design/16-planning-hub.md#organize-pipeline) |
| Journeys | [16 §journey-scattered](../../design/16-planning-hub.md#journey-scattered) |

---

## Changelog {#phase-1-6-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial sub-phase doc — full Organize alpha |
