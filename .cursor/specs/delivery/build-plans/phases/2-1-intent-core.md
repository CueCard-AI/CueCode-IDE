# Build phase 2.1 — `cuecode_sandbox` intent core {#phase-2-1}

> **Invoke:** `Build phase 2.1` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 7–10 days |
| **Track** | 2 — Intent profiles |
| **Roadmap** | [Phase 2](../07-implementation-roadmap#phase-2) |
| **QA script** | QA-P2 steps 1–2, 5–6 |

## Deliverable {#phase-2-1-deliverable}

Explore / Fix / Ship / Review intents reconfigure tool permissions, sandbox policy, and system prompt suffix. Persisted per workspace.

## Depends / blocks {#phase-2-1-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.6 ([Planning Hub Organize alpha](./1-6-organize-with-ai.md)) |
| **Blocks** | 2.2, 3.1, 3b.1, 4.1, C.0 |

## Out of scope {#phase-2-1-out-of-scope}

- Intent switcher UI, sandbox badge (2.2)
- Checkpoints, review panel (Track 3)
- Background spawn (3b.1)

---

## Tasks {#phase-2-1-tasks}

Implement in order. Paths relative to `apps/CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 2.1.1 | Create `cuecode_sandbox` (or module → migrate) | `crates/cuecode_sandbox/` | `[ ]` |
| 2.1.2 | `Intent` enum + `IntentProfile` struct | `crates/cuecode_sandbox/src/intent.rs` | `[ ]` |
| 2.1.3 | Wire intent → `tool_permissions` | `crates/agent/`, `crates/cuecode_sandbox/src/policy.rs` | `[ ]` |
| 2.1.4 | Wire intent → `agent::sandboxing` | `crates/agent/src/sandboxing.rs` | `[ ]` |
| 2.1.5 | Intent block in agent templates | `crates/agent/templates/` | `[ ]` |
| 2.1.6 | Persist intent per workspace | `crates/cuecode_sandbox/src/persistence.rs` | `[ ]` |
| 2.1.7 | `~/.config/cuecode/intent_profiles.json` overrides | `crates/cuecode_sandbox/src/persistence.rs` | `[ ]` |
| 2.1.8 | Feature-flag Orchestrate (off until 3b/5) | `crates/cuecode_sandbox/src/intent.rs` | `[ ]` |

---

## Implementation notes {#phase-2-1-impl}

Create `crates/cuecode_sandbox/` (or module under `agent` first, migrate later) with:

```rust
pub enum Intent {
    Explore,
    Fix,
    Ship,
    Review,
    Orchestrate,  // feature-flag off until 3b/5
}

pub enum ExecutionContext {
    Active,
    Async,
    Hybrid,
}

pub struct IntentProfile {
    pub intent: Intent,
    pub tool_overlay: ToolPermissionOverlay,
    pub network: NetworkPolicy,
    pub fs_write: FsWritePolicy,
    pub sandbox_enabled: bool,
    pub default_execution: ExecutionContext,
    pub system_prompt_suffix: &'static str,
}

pub fn default_profiles() -> Vec<IntentProfile>;
pub fn apply_intent(session: &mut SandboxSession, intent: Intent, cx: &mut App) -> anyhow::Result<()>;
```

Wire intent → `tool_permissions` ([08 §permissions](../../agent/08-agent-tools-and-skills.md#permissions))
and `agent::sandboxing` ([10 §terminal-sandbox](../../ops/10-infrastructure.md#terminal-sandbox)).
Persist per workspace; overrides at `~/.config/cuecode/intent_profiles.json`.

---

## Verify {#phase-2-1-verify}

```bash
cd apps/CueCode-IDE
cargo test -p cuecode_sandbox
cargo build -p cuecode_sandbox -p agent
```

---

## Exit criteria {#phase-2-1-exit}

- [ ] Explore denies `edit_file` at permission layer — [07 §phase-2-acceptance](../07-implementation-roadmap.md#phase-2-acceptance) row 1
- [ ] Fix enables macOS sandbox when flag on — row 3

---

## QA {#phase-2-1-qa}

Manual steps before marking **Status** `[x]`:

1. Set Explore via API/settings — agent `edit_file` denied at permission layer
2. Switch Fix — edit path available (confirm or execute)
3. Close workspace — reopen — intent persisted

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P2 steps 1–2, 5–6

---

## PR checklist {#phase-2-1-pr}

- [ ] PR title/body cites **Build phase 2.1** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-2-1-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| Intent profiles | ../../core/04-sandbox-core.md#intent-profiles |
| Permissions | ../../agent/08-agent-tools-and-skills.md#permissions |
| Terminal sandbox | ../../ops/10-infrastructure.md#terminal-sandbox |

---

## Changelog {#phase-2-1-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
