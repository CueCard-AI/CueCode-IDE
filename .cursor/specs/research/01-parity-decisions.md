# Parity decision policy {#parity-decisions}

> **Inventory:** [00-claude-code-inventory](./00-claude-code-inventory.md) · **Program:** [15-competitive-parity](../parity/15-competitive-parity.md)

Rules for **Adopt | Adapt | Defer | Reject** on every Claude Code capability row.

---

## Decision definitions {#definitions}

| Decision | Meaning | PR requirement |
|----------|---------|----------------|
| **Adopt** | Same user-visible capability; map to existing Zed/CueCode tool or setting with minimal delta | Link inventory row; behavior matches CC reference |
| **Adapt** | Same outcome, different mechanism (GPUI, intent, Rust tool wall, spec path) | Link gap doc section; document delta in PR |
| **Defer** | Competitive parity intended; not v1/Alpha | Link phase in [07-roadmap](../delivery/07-implementation-roadmap); track in inventory |
| **Reject** | Explicit non-goal for CueCode v1+ | Link [15 §rejects](../parity/15-competitive-parity.md#explicit-rejects) or ADR |

**Forbidden:** silent gaps — every Claude Code tool and top command must have a row in [00](./00-claude-code-inventory.md).

---

## Harness context requirement {#harness-context}

Every **Adapt** and **Adopt** row must classify **Active | Async | Hybrid** (or all three if applicable). See [local §three-contexts](../harness/local/01-agent-harness.md#three-contexts).

Hybrid flows must name a **handoff artifact** per [local §C.5](../harness/local/01-agent-harness.md#c-5-hybrid-handoff-artifacts-required).

---

## Parity score targets {#score-targets}

Measured against [00-inventory](./00-claude-code-inventory.md) rows at release gates:

| Gate | Adopt+Adapt coverage | Defer documented | Reject documented |
|------|---------------------|------------------|-------------------|
| **Alpha** | ≥85% tools · ≥80% top commands | 100% defer rows have phase | 100% reject rows have rationale |
| **Beta** | ≥95% tools · ≥90% top commands | defer → adopt/adapt or stay defer with date | unchanged |
| **Competitive 1.0** | 100% rows resolved (no orphan) | ≤5 defer (cron, share, remote) | rejects frozen |

**CueCode-only rows** (spec tools, SDAL) do not count toward CC parity but must appear in [08](../agent/08-agent-tools-and-skills).

---

## Cloud vs open fork {#cloud-boundary}

| Runs on | Capabilities |
|---------|--------------|
| **GPL fork (always)** | Tool execution, permissions UI, checkpoints, spec index, NativeAgent fallback |
| **Cloud harness (default product)** | Turn scheduler, builtin prompt assembly, compaction policy, lane arbitration, optional team memory sync |
| **Either** | Verification VERDICT semantics (client parses; server may spawn) |

See [harness/cloud/01 §moat-boundary](../harness/cloud/01-overview.md#moat-boundary).

---

## When to choose Adapt over Adopt {#adapt-vs-adopt}

Choose **Adapt** when any of:

1. Claude Code is CLI/Ink; CueCode is GPUI native (bridge → editor integration).
2. Claude Code uses mode flags (`coordinatorMode`, `EnterPlanMode`); CueCode uses **intent profiles**.
3. Claude Code uses prose completion; CueCode requires **structured artifacts** (VERDICT, notification JSON).
4. Rust **tool wall** enforcement is required ([15 §thesis](../parity/15-competitive-parity.md#thesis)).

Choose **Adopt** when Zed fork already ships equivalent behavior with same user mental model.

---

## Review checklist {#review-checklist}

Before merging parity work:

- [ ] Inventory row updated (decision unchanged or promoted defer→adapt)
- [ ] Active/Async/Hybrid classified
- [ ] E2E flow in [16](../parity/16-end-to-end-flows.md) updated if user journey changes
- [ ] Open question filed in [12](../ops/12-open-questions) if decision blocks on product call

---

## Document status {#status}

| Field | Value |
|-------|-------|
| Status | Living policy |
| Owner | Product + agent |
