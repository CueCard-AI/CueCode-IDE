# Build phase 4.1 — Trust store + UI {#phase-4-1}

> **Invoke:** `Build phase 4.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 7–10 days |
| **Track** | 4 — Trust graph |
| **Roadmap** | [Phase 4](../07-implementation-roadmap#phase-4) |
| **QA script** | QA-P4 |

## Deliverable {#phase-4-1-deliverable}

Per-repo trust JSON; promotion thresholds; Settings trust UI; auto-approve proven-safe actions. **Phase 4 complete.**

## Depends / blocks {#phase-4-1-deps}

| | Phase |
|---|-------|
| **Depends on** | 2.2 |
| **Blocks** | C.4 |

## Out of scope {#phase-4-1-out-of-scope}

- Cloud BYOK trust routing (C.4)
- Multi-lane (5.1)

---

## Tasks {#phase-4-1-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 4.1.1 | Trust JSON per repo hash | `crates/cuecode_sandbox/src/trust.rs` | `[ ]` |
| 4.1.2 | Promotion thresholds + hard deny | `crates/cuecode_sandbox/src/trust.rs` | `[ ]` |
| 4.1.3 | Settings → Agent → Trust UI | `crates/settings_ui/` or settings panel | `[ ]` |
| 4.1.4 | Tool card labels: auto-approved vs you approved | `crates/agent_ui/` | `[ ]` |
| 4.1.5 | Unit tests in `cuecode_sandbox` | `crates/cuecode_sandbox/tests/` | `[ ]` |
| 4.1.6 | Analytics: `cuecode.trust.*` | `crates/telemetry/` | `[ ]` |

---

## Implementation notes {#phase-4-1-impl}

```rust
pub fn load_trust_store(repo_hash: &str) -> anyhow::Result<TrustStore>;
pub fn record_trust_evidence(store: &mut TrustStore, evidence: TrustEvidence);
pub fn evaluate_trust(request: &ToolRequest, store: &TrustStore) -> TrustDecision;
```

- Store at `~/.config/cuecode/trust/<repo_hash>.json`.
- Hard deny: `.env` writes, destructive rm, etc. — never auto-promote.
- Tool cards show "auto-approved" vs "you approved" labels.

---

## Verify {#phase-4-1-verify}

```bash
cd CueCode-IDE
cargo test -p cuecode_sandbox -- trust
```

---

## Exit criteria {#phase-4-1-exit}

- [ ] [07 §phase-4-exit](../07-implementation-roadmap.md#phase-4-exit) all rows
- [ ] **Phase 4 complete**

---

## QA {#phase-4-1-qa}

Manual steps before marking **Status** `[x]`:

1. Deny then allow same `cargo test` five times — sixth run auto-approved
2. Settings → Trust — rule visible with evidence
3. Revoke rule — next run confirms again
4. Attempt `.env` write — always blocked
5. Run full QA-P4 — steps 2, 3, 5 green

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P4

---

## PR checklist {#phase-4-1-pr}

- [ ] PR title/body cites **Build phase 4.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-4-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Trust graph | ../../core/05-innovations.md#trust-graph |
| Trust UI | ../../design/09-ui-ux-spec.md#trust-ui |
| Permissions | ../../agent/08-agent-tools-and-skills.md#permissions |

---

## Changelog {#phase-4-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
