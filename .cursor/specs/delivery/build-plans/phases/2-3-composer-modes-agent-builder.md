# Build phase 2.3 — Composer modes + Agent Builder {#phase-2-3}

> **Invoke:** `Build phase 2.3` — open **this file only**; implement sub-phases in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[ ] Not started` |
| **Last verified** | — |
| **Duration** | 3–4 weeks (2.3a→e) |
| **Track** | 2 — Intent profiles + composer UX |
| **Roadmap** | [Phase 2](../07-implementation-roadmap#phase-2) |
| **Deep spec** | [14-agent-modes-and-builder](../../agent/14-agent-modes-and-builder.md) |
| **QA script** | QA-P2 modes extension (see §verify) |

## Deliverable {#phase-2-3-deliverable}

Cursor-class **composer mode picker** (Agent / Ask / Plan / …), deprecation of
`ManageProfilesModal` maze, **saved agents** schema + migration, **Agent Builder**
settings UI with Test run; optional agent-assisted edits + YAML I/O in 2.3d.

## Depends / blocks {#phase-2-3-deps}

| | Phase |
|---|-------|
| **Depends on** | 2.1 (intent core — strongly recommended), 2.2 (intent UI — partial OK) |
| **Blocks** | 5.2 composer polish (footer baselines), 6.2 competitive gate (Cursor mode parity) |

## Out of scope {#phase-2-3-out-of-scope}

- Cloud sync of agent definitions
- Dockable builder panel (settings-only v1)
- Replacing ACP external agent mode selector

---

## Sub-phases {#phase-2-3-subphases}

| ID | Scope | Exit |
|----|-------|------|
| **2.3a** | Mode presets + composer picker | EC-AM-1, EC-AM-2 |
| **2.3b** | `AgentDefinition` schema + migration | EC-AM-4 |
| **2.3c** | Builder editor + Test run | EC-AM-5, EC-AM-6 |
| **2.3d** | Agent assist + YAML import/export | EC-AM-7 |
| **2.3e** | Telemetry, docs, polish | Beta-ready footer + settings |

---

## Tasks {#phase-2-3-tasks}

Paths relative to `apps/CueCode-IDE/` unless noted.

### 2.3a — Composer mode picker {#phase-2-3a}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 2.3a.1 | Add `ModePreset` catalog (Agent/Ask/Plan/Debug/Review/Orchestrate) | `crates/agent_settings/src/mode_presets.rs` | `[ ]` |
| 2.3a.2 | Map presets → intent ids (stub OK if 2.1 incomplete) | `crates/agent_settings/`, `cuecode_sandbox/` | `[ ]` |
| 2.3a.3 | Replace profile picker with mode picker in composer footer | `crates/agent_ui/src/profile_selector.rs` or `mode_picker.rs` | `[ ]` |
| 2.3a.4 | Popover: presets + "Manage agents…" (no Configure modal) | `crates/agent_ui/` | `[ ]` |
| 2.3a.5 | Rename display Write→Agent in defaults | `assets/settings/default.json` | `[ ]` |
| 2.3a.6 | Thread stores `ModeSelection` | `crates/agent/src/thread.rs` | `[ ]` |
| 2.3a.7 | Prompt partials per preset | `crates/agent/src/templates/modes/` | `[ ]` |
| 2.3a.8 | **Retain** split I/O rings in footer layout (`#split_token_usage`) | `crates/agent_ui/src/conversation_view/thread_view.rs` | `[ ]` |

### 2.3b — Schema + migration {#phase-2-3b}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 2.3b.1 | `AgentDefinitionContent` in settings | `crates/settings_content/src/agent.rs` | `[ ]` |
| 2.3b.2 | `AgentDefinition` runtime type + resolve | `crates/agent_settings/src/agent_definition.rs` | `[ ]` |
| 2.3b.3 | Runtime precedence (intent ∩ preset ∩ saved) | `crates/agent/src/thread.rs`, `tool_permissions.rs` | `[ ]` |
| 2.3b.4 | Settings migration legacy profiles → definitions | `crates/settings/src/settings_file.rs` | `[ ]` |
| 2.3b.5 | DB thread load maps legacy `profile` column | `crates/agent/src/db.rs` | `[ ]` |
| 2.3b.6 | Saved agents section in mode popover | `crates/agent_ui/` | `[ ]` |

### 2.3c — Agent Builder UI {#phase-2-3c}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 2.3c.1 | Settings → Agents nav + list page | `crates/settings_ui/src/pages/agents_page.rs` | `[ ]` |
| 2.3c.2 | Builder editor blocks (purpose, model, tools, MCP, skills, instructions) | `crates/settings_ui/` | `[ ]` |
| 2.3c.3 | Save / Fork / Revert | `crates/agent_settings/`, `settings_ui/` | `[ ]` |
| 2.3c.4 | Test run modal (ephemeral thread) | `crates/agent_ui/` or `agent/` | `[ ]` |
| 2.3c.5 | Deprecate `ManageProfilesModal` → redirect | `crates/agent_ui/src/agent_configuration/` | `[ ]` |

### 2.3d — Agentic builder + I/O {#phase-2-3d}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 2.3d.1 | Builder assist chat + propose patch | `crates/settings_ui/`, `crates/agent/` | `[ ]` |
| 2.3d.2 | Diff preview before apply | `crates/settings_ui/` | `[ ]` |
| 2.3d.3 | YAML export/import (`*.agent.yaml`) | `crates/agent_settings/src/export.rs` | `[ ]` |
| 2.3d.4 | Validation + size limits on import | `crates/agent_settings/` | `[ ]` |

### 2.3e — Polish {#phase-2-3e}

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 2.3e.1 | Telemetry events (§telemetry in deep spec) | `crates/telemetry/` | `[ ]` |
| 2.3e.2 | Docs: agent modes + builder | `docs/` or `.cursor/specs` cross-links | `[ ]` |
| 2.3e.3 | GPUI screenshot baselines for footer | `crates/agent_ui/tests/` | `[ ]` |
| 2.3e.4 | Update competitive parity checklist | `.cursor/specs/parity/` | `[ ]` |

---

## Verify {#phase-2-3-verify}

```bash
cd apps/CueCode-IDE
cargo build -p agent_ui -p agent_settings -p settings_ui -p agent
cargo test -p agent_settings -p agent
# Manual: QA-P2 modes (see deep spec §acceptance EC-AM-1..7)
```

---

## Exit criteria {#phase-2-3-exit}

- [ ] All [14-agent-modes-and-builder §acceptance](../../agent/14-agent-modes-and-builder.md#acceptance) EC-AM-1..6 pass (EC-AM-7 for 2.3d; **EC-AM-8** split rings for 2.3a)
- [ ] No entry point opens legacy name-only New Profile modal
- [ ] `./script/clippy` on touched crates

---

## QA {#phase-2-3-qa}

1. Footer shows **Agent ▼** not Write; popover has six presets + Manage agents…
2. **Ask** mode — agent cannot `edit_file` (deny visible)
3. **Split I/O rings** — ↑↓ circular progress visible left of mode/model when model supports split display
4. Create saved agent in Settings → Agents → appears in popover
4. Upgrade settings with legacy `debugger` profile → saved agent preserved
5. Test run — tool trace modal, no new sidebar thread
6. (2.3d) Describe change in builder assist → diff → Save

---

## PR checklist {#phase-2-3-pr}

- [ ] PR cites **Build phase 2.3x** and [14-agent-modes-and-builder](../../agent/14-agent-modes-and-builder.md)
- [ ] Sub-phase tasks marked `[x]`
- [ ] Update [build-plans README](../README.md#phase-index)

---

## Changelog {#phase-2-3-changelog}

| Date | Change |
|------|--------|
| 2026-06-20 | Initial sub-phase doc |
