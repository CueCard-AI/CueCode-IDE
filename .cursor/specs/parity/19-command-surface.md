# Command surface — slash to GPUI {#command-surface}

> **Program:** [15-competitive-parity](./15-competitive-parity.md) · **Inventory:** [00 §commands](../research/00-claude-code-inventory.md#commands-top-60)

Claude Code **slash commands** map to CueCode **GPUI surfaces** — command palette, composer `/`, agent header, settings. Parity = same capability in **≤2 user actions**, not identical syntax.

---

## Mapping rules {#mapping-rules}

1. **Prefer palette + composer** over new top-level menus.
2. **Agent harness commands** (`/plan`, `/tasks`) → agent panel when thread focused.
3. **Editor commands** (`/vim`, `/theme`) → existing Zed settings ( **Adopt** ).
4. **Rejected CC commands** → document why ([15 §rejects](./15-competitive-parity.md#explicit-rejects)).

---

## Session commands {#session-commands}

| CC command | CueCode access | UI surface | Decision |
|------------|----------------|------------|----------|
| `/resume` | Thread sidebar → Open | Agent panel | Adapt |
| `/rewind` | Checkpoints → Rewind to here | Timeline + confirm | Adapt |
| `/clear` | New thread | Sidebar + | Adapt |
| `/rename` | Rename thread | Sidebar context menu | Adopt |
| `/export` | Export session bundle | Thread menu | Adapt |
| `/share` | Share link (opt-in) | Defer Phase 6 | Defer |
| `/session` | Session metadata drawer | Agent header | Adapt |
| `/tag` | Tag thread | Defer | Defer |
| `/copy` | Copy transcript | Conversation menu | Adapt |

**Flow:** [16 flow H](./16-end-to-end-flows.md#flow-h-resume-export)

---

## Context commands {#context-commands}

| CC command | CueCode access | Spec |
|------------|----------------|------|
| `/compact` | Command palette: "Compact conversation" | [17 §compact](./17-memory-and-context.md#compact) |
| `/context` | Agent header → Context budget | [17 §context-budget-ui](./17-memory-and-context.md#context-budget-ui) |
| `/memory` | Settings → Memory or `@memory` | [17 §memory-ui](./17-memory-and-context.md#memory-ui) |
| `/files` | Composer `@` file picker | Adopt |
| `/add-dir` | Add folder to workspace | Adopt (Zed) |

---

## Harness commands {#harness-commands}

| CC command | CueCode access | Spec |
|------------|----------------|------|
| `/plan` | Intent → Plan + plan gate banner | [16 flow B](./16-end-to-end-flows.md#flow-b-ship-verify) |
| `/tasks` | Lane panel + task list | [18 §task-protocol](./18-teams-and-tasks.md#task-protocol) |
| `/agents` | Spawn agent picker / builtin types | [local §builtin-agents](../harness/local/01-agent-harness.md#builtin-agents) |
| `/skills` | Skills browser + `/skill_name` in composer | [08 §skills](../agent/08-agent-tools-and-skills#skills) |
| `/fast` | Lane model hint Fast | [local §builtin-agents](../harness/local/01-agent-harness.md#builtin-agents) |
| `/effort` | Model effort in header | Adapt |

Composer `/` menu registers CueCode skills **and** harness shortcuts above.

---

## Git & review commands {#git-commands}

| CC command | CueCode access |
|------------|----------------|
| `/commit` | Review footer → Commit |
| `/review` | Intent → Review |
| `/diff` | Unified review → Diffs tab |
| `/branch` | Git panel (Zed) |
| `/pr_comments` | Defer — GitHub integration [20](./20-platform-integrations.md) |

---

## MCP & settings {#mcp-commands}

| CC command | CueCode access |
|------------|----------------|
| `/mcp` | Settings → MCP servers |
| `/config` | Settings → Agent |
| `/permissions` | Settings → Trust + tool permissions |
| `/sandbox-toggle` | Intent header sandbox badge menu |
| `/hooks` | Cursor hooks docs + stop hook settings [20](./20-platform-integrations.md) |

---

## Model commands {#model-commands}

| CC command | CueCode access |
|------------|----------------|
| `/model` | Agent header model picker |
| `/cost` | Agent header → Usage & cost |
| `/usage` | Same panel |
| `/rate-limit-options` | Error toast → recovery [10](../ops/10-infrastructure) |

---

## Auth commands {#auth-commands}

| CC command | CueCode access |
|------------|----------------|
| `/login` | CueCode cloud sign-in (optional) |
| `/logout` | Sign out |
| `/upgrade` | Cloud upgrade |
| `/extra-usage` | Usage limits panel |

No zed.dev login for core agent ([03](../core/03-fork-and-rebrand)).

---

## Onboarding commands {#onboarding-commands}

| CC command | CueCode access |
|------------|----------------|
| `/init` | Welcome wizard: link specs, local model, optional skills |
| `/doctor` | Command palette: "CueCode Doctor" → [10 runbooks](../ops/10-infrastructure#operational-runbooks) |
| `/release-notes` | What's new on upgrade |

**Adapt** `/init` to produce `.cursor/specs/` onboarding story instead of CLAUDE.md-only ([05 SDAL moat](../core/05-innovations#sdal)).

---

## Settings commands (Adopt Zed) {#settings-commands}

| CC command | CueCode |
|------------|---------|
| `/vim` | Zed vim mode |
| `/theme` | Theme picker |
| `/keybindings` | Keymap editor |
| `/color` | Theme accents |
| `/output-style` | Agent output style setting |

---

## Rejected commands {#rejected-commands}

| CC command | Rationale |
|------------|-----------|
| `/ide`, `/bridge` | CueCode is the IDE [20](./20-platform-integrations.md) |
| `/desktop`, `/mobile` | Non-goal v1 |
| Plugin commands | Reject plugins [20](./20-platform-integrations.md) |
| `/chrome`, `/stickers`, `/btw`, `/thinkback` | Internal/easter egg |
| `/heapdump` | Dev-only |

---

## Implementation checklist {#checklist}

- [ ] Register palette actions with stable IDs (`cuecode.command.*`)
- [ ] Composer `/` merges skills + harness shortcuts
- [ ] Each row linked from [00-inventory](../research/00-claude-code-inventory.md)
- [ ] i18n strings in [09 copy deck](../design/09-ui-ux-spec) when UI exists

---

## Document status {#status}

| Field | Value |
|-------|-------|
| Status | Draft — gap spec |
| Last updated | 2026-06-17 |
