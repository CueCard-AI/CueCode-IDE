# Build phase 1.5 — Spec roots + pin modes (P-H2) {#phase-1-5}

> **Invoke:** `Build phase 1.5` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 4–6 days |
| **Track** | 1 — Planning Hub |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [16-planning-hub §P-H2](../../design/16-planning-hub.md#delivery-phases) |
| **QA script** | QA-P1 (extended) |

## Deliverable {#phase-1-5-deliverable}

**`cuecode_specs`** multi-root: prefer `.cuecode/specs`, fallback `.cursor/specs`, optional `merge_cursor`. Pin modes **off · summary · full · section** wired to agent context injection.

## Depends / blocks {#phase-1-5-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.4 |
| **Blocks** | — (recommended before 2.2; not hard gate for 2.1) |

## Out of scope {#phase-1-5-out-of-scope}

- Organize pipeline (1.6)
- `cursor-compat` export CLI (later P-H4)
- Promote-to-`.cuecode/specs` file copy UX (optional stub)

---

## Tasks {#phase-1-5-tasks}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.5.1 | Prefer-only index: `.cuecode/specs` if present, else `.cursor/specs` | `crates/cuecode_specs/src/parse.rs`, `index.rs` | `[ ]` |
| 1.5.2 | `roots.merge_cursor` from manifest — union index, `.cuecode` wins on clash | `crates/cuecode_specs/`, `cuecode_plans/` | `[ ]` |
| 1.5.3 | Merge manifest `artifacts[]` external paths into index | `crates/cuecode_specs/`, `cuecode_plans/` | `[ ]` |
| 1.5.4 | Session: `active_ticket_id` + `pinned_bundle` + per-ref `pin_mode` | `crates/acp_thread/`, `crates/agent/` | `[ ]` |
| 1.5.5 | Pin injection: summary (frontmatter + tasks), full body, section anchor | `crates/cuecode_specs/`, `crates/agent/` | `[ ]` |
| 1.5.6 | Plan Pin ▼ control; chip shows `📎 {title} · {done}/{total} · {n} specs` | `crates/agent_ui/src/planning_hub.rs` | `[ ]` |
| 1.5.7 | Compaction preserves ticket id + pin modes | `crates/agent/` | `[ ]` |
| 1.5.8 | Update system prompt copy: `.cuecode/specs` primary | `crates/agent/templates/` | `[ ]` |
| 1.5.9 | `@phase` / `@plan` mentions + `MentionUri::PlanArtifact` | `crates/acp_thread/`, `crates/agent_ui/` | `[ ]` |

---

## Implementation notes {#phase-1-5-impl}

- Spec roots: [16 §spec-roots](../../design/16-planning-hub.md#spec-roots).
- Pin policy: [16 §pin-policy](../../design/16-planning-hub.md#pin-policy).
- Chat mentions: [16 §chat-integration](../../design/16-planning-hub.md#chat-integration).
- Token budget: summary must inject less than full (E3).

---

## Verify {#phase-1-5-verify}

```bash
cd apps/CueCode-IDE
cargo test -p cuecode_specs
cargo build -p agent_ui -p cuecode
# Manual: pin summary vs full — measurable context difference
# Manual: repo with only .cursor/specs still indexes (fallback)
```

---

## Exit criteria {#phase-1-5-exit}

- [ ] [16-planning-hub E3, E4](../../design/16-planning-hub.md#exit-criteria)
- [ ] Pin modes work from hub + chip

---

## Changelog {#phase-1-5-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial sub-phase doc |
