# Claude Code reference inventory {#claude-code-inventory}

> **Source:** `~/CueInference/research/claude-code-main/` (leaked CLI, March 2026 — patterns only; CueCode implements in Rust/GPUI, GPL stack).  
> **Decisions:** [01-parity-decisions](./01-parity-decisions.md) · **Program:** [15-competitive-parity](../parity/15-competitive-parity.md)

Master index of Claude Code capabilities mapped to CueCode competitive parity. Every row has an **Adopt | Adapt | Defer | Reject** decision.

**Harness context:** Active (foreground) · Async (background) · Hybrid (handoff + artifact)

---

## How to use {#how-to-use}

1. Before adding a feature, find the row here — do not invent parallel names.
2. **Adopt** rows must match behavior in [08-agent-tools-and-skills](../agent/08-agent-tools-and-skills) or [19-command-surface](../parity/19-command-surface.md).
3. **Adapt** rows must cite the CueCode mechanism in the linked gap doc.
4. Update this file when Claude Code reference changes or CueCode ships parity.

---

## Built-in agents (Claude Code) {#builtin-agents}

| CC agent | Role | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|----------|------|-------------------|-------|----------|------|-------|
| `generalPurposeAgent` | Default implementer | `implement` builtin + Fix intent | Active | **Adapt** | [local §builtin-agents](../harness/local/01-agent-harness.md#builtin-agents) | 3b |
| `exploreAgent` | Read-only survey | `explore` builtin | Hybrid | **Adapt** | [local §A.2](../harness/local/01-agent-harness.md#a-2-active-agents) | 3b |
| `planAgent` | Strategy before edits | `plan` builtin + plan gate | Active | **Adapt** | [local §C.2](../harness/local/01-agent-harness.md#c-2-plan-ship-pipeline) | 3b |
| `verificationAgent` | Adversarial verify | `verification` builtin + VERDICT | Async | **Adapt** | [local §B.2](../harness/local/01-agent-harness.md#b-2-verification-agent-async-gate) | 3b |
| Coordinator workers | Spawn-only main | **Orchestrate intent** + coordinator | Hybrid | **Adapt** | [18-teams-and-tasks](../parity/18-teams-and-tasks.md#coordinator) | 5 |
| `claudeCodeGuideAgent` | Product help | CueCode onboarding + docs skill | Active | **Adapt** | [19 §onboarding](../parity/19-command-surface.md#onboarding-commands) | 6 |
| `statuslineSetupAgent` | CLI statusline | CueCode status line skill | Active | **Defer** | [20 §statusline](../parity/20-platform-integrations.md#statusline) | 6+ |

---

## Tools — file & search {#tools-file-search}

| CC tool | Description | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|---------|-------------|-------------------|-------|----------|------|-------|
| `FileReadTool` | Read files, images, PDFs | `read_file` | Active | **Adopt** | [08 §existing](../agent/08-agent-tools-and-skills#existing-tools) | 0 |
| `FileWriteTool` | Create / overwrite | `edit_file` / write path | Active | **Adopt** | [08](../agent/08-agent-tools-and-skills) | 0 |
| `FileEditTool` | String-replace edits | `edit_file` (edit_session) | Active | **Adopt** | [08](../agent/08-agent-tools-and-skills) | 0 |
| `GrepTool` | ripgrep search | `grep` | Active | **Adopt** | [08](../agent/08-agent-tools-and-skills) | 0 |
| `GlobTool` | Path glob | `find_path` / glob in grep | Active | **Adapt** | [08](../agent/08-agent-tools-and-skills) | 0 |
| `NotebookEditTool` | Jupyter cells | GPUI notebook editor + tool | Active | **Defer** | [20 §notebooks](../parity/20-platform-integrations.md#notebooks) | 6+ |

---

## Tools — shell & network {#tools-shell-network}

| CC tool | Description | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|---------|-------------|-------------------|-------|----------|------|-------|
| `BashTool` | Shell execution | `terminal` + OS sandbox | Active | **Adapt** | [10 §terminal-sandbox](../ops/10-infrastructure#terminal-sandbox) | 0–2 |
| `PowerShellTool` | Windows shell | `terminal` (PowerShell on Windows) | Active | **Adapt** | [10](../ops/10-infrastructure) | 0 |
| `WebFetchTool` | HTTP fetch | `fetch` | Active | **Adopt** | [08](../agent/08-agent-tools-and-skills) | 0 |
| `WebSearchTool` | Web search | `web_search` | Active | **Adopt** | [08](../agent/08-agent-tools-and-skills) | 0 |

---

## Tools — agent harness {#tools-harness}

| CC tool | Description | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|---------|-------------|-------------------|-------|----------|------|-------|
| `AgentTool` | Spawn subagent | `spawn_agent` + `run_in_background` + `agent_type` | Active/Async/Hybrid | **Adapt** | [local §B.1](../harness/local/01-agent-harness.md#b-1-background-subagents) | 3b |
| `EnterPlanModeTool` | Enter plan mode | **Plan intent** + plan gate (no separate tool) | Active | **Adapt** | [04 §intent-profiles](../core/04-sandbox-core#intent-profiles) | 2–3 |
| `ExitPlanModeTool` | Exit plan mode | Approve plan → Fix/Ship intent | Active | **Adapt** | [local §composer-states](../harness/local/01-agent-harness.md#composer-states) | 3 |
| `SyntheticOutputTool` | Structured coordinator output | Coordinator transcript + notification envelope | Hybrid | **Adapt** | [local §notification-payloads](../harness/local/01-agent-harness.md#notification-payloads) | 5 |
| `SleepTool` | Proactive wait | — | Async | **Reject** | [20 §proactive](../parity/20-platform-integrations.md#proactive-mode) | — |
| `AskUserQuestionTool` | Structured user Q | `ask_user` tool + GPUI modal | Active | **Adapt** | [08 §new-tools](../agent/08-agent-tools-and-skills#new-tools) | 3 |
| `BriefTool` | Upload brief / context | Session brief attachment + `@spec` | Hybrid | **Adapt** | [17 §brief](../parity/17-memory-and-context.md#session-brief) | 4 |

---

## Tools — tasks & teams {#tools-tasks-teams}

| CC tool | Description | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|---------|-------------|-------------------|-------|----------|------|-------|
| `TodoWriteTool` | Session todo list | `AcpThread.plan` + plan sync | Active | **Adapt** | [05 §sdal](../core/05-innovations#sdal) | 1–3 |
| `TaskCreateTool` | Create task entity | `task_create` + lane task store | Hybrid | **Adapt** | [18 §task-protocol](../parity/18-teams-and-tasks.md#task-protocol) | 5 |
| `TaskGetTool` | Get task | `task_get` | Hybrid | **Adapt** | [18](../parity/18-teams-and-tasks.md) | 5 |
| `TaskListTool` | List tasks | Lane panel + `task_list` | Hybrid | **Adapt** | [18](../parity/18-teams-and-tasks.md) | 5 |
| `TaskUpdateTool` | Update task | `task_update` + plan sync | Hybrid | **Adapt** | [18](../parity/18-teams-and-tasks.md) | 5 |
| `TaskStopTool` | Stop task/agent | Cancel background `Task` + `task_stop` | Async | **Adapt** | [18 §cancellation](../parity/18-teams-and-tasks.md#cancellation) | 3b |
| `TaskOutputTool` | Read task output | Sidechain read + notification open | Async | **Adapt** | [local §B.5](../harness/local/01-agent-harness.md#b-5-async-artifacts-on-disk) | 3b |
| `SendMessageTool` | Inter-agent message | Lane message bus + `send_lane_message` | Hybrid | **Adapt** | [18 §messaging](../parity/18-teams-and-tasks.md#lane-messaging) | 5 |
| `TeamCreateTool` | Create agent team | **Lane spawn** under Orchestrate (no separate team entity v1) | Hybrid | **Adapt** | [18 §teams-vs-lanes](../parity/18-teams-and-tasks.md#teams-vs-lanes) | 5 |
| `TeamDeleteTool` | Delete team | Close lane tab + cancel tasks | Hybrid | **Adapt** | [18](../parity/18-teams-and-tasks.md) | 5 |

---

## Tools — git & isolation {#tools-git}

| CC tool | Description | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|---------|-------------|-------------------|-------|----------|------|-------|
| `EnterWorktreeTool` | Git worktree isolation | Ship session worktree + `enter_worktree` | Hybrid | **Defer** | [18 §worktree](../parity/18-teams-and-tasks.md#worktree-isolation) | 5 |
| `ExitWorktreeTool` | Leave worktree | `exit_worktree` + merge review | Hybrid | **Defer** | [18](../parity/18-teams-and-tasks.md) | 5 |

---

## Tools — MCP & LSP {#tools-mcp-lsp}

| CC tool | Description | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|---------|-------------|-------------------|-------|----------|------|-------|
| `MCPTool` | Invoke MCP tool | `context_server` registry | Active | **Adopt** | [08 §mcp](../agent/08-agent-tools-and-skills#mcp) | 0 |
| `ListMcpResourcesTool` | List MCP resources | `list_mcp_resources` | Active | **Adopt** | [08](../agent/08-agent-tools-and-skills) | 0+ |
| `ReadMcpResourceTool` | Read MCP resource | `read_mcp_resource` | Active | **Adopt** | [08](../agent/08-agent-tools-and-skills) | 0+ |
| `McpAuthTool` | MCP OAuth | MCP auth UI in settings | Active | **Adapt** | [10 §mcp](../ops/10-infrastructure) | 2 |
| `LSPTool` | LSP symbols/actions | `diagnostics`, `find_references`, extended LSP tools | Active | **Adapt** | [08 §existing](../agent/08-agent-tools-and-skills#existing-tools) | 3 |
| `ToolSearchTool` | Deferred tool discovery | MCP catalog + `tool_search` | Active | **Defer** | [08 §deferred-catalog](../agent/08-agent-tools-and-skills#mcp) | 5+ |
| `SkillTool` | Load skill body | `skill` | Active | **Adopt** | [08 §skills](../agent/08-agent-tools-and-skills#skills) | 0 |

---

## Tools — scheduling & remote {#tools-scheduling}

| CC tool | Description | CueCode equivalent | A/A/H | Decision | Spec | Phase |
|---------|-------------|-------------------|-------|----------|------|-------|
| `CronCreateTool` | Schedule agent run | `schedule_cron` + headless session | Async | **Defer** | [20 §cron](../parity/20-platform-integrations.md#scheduled-agents) | 5+ |
| `CronListTool` | List crons | Cron panel / `cron_list` | Async | **Defer** | [20](../parity/20-platform-integrations.md) | 5+ |
| `CronDeleteTool` | Delete cron | `cron_delete` | Async | **Defer** | [20](../parity/20-platform-integrations.md) | 5+ |
| `RemoteTriggerTool` | Remote webhook trigger | CI/webhook → session notification | Async→Hybrid | **Defer** | [20 §webhooks](../parity/20-platform-integrations.md#webhooks) | 6+ |
| `ConfigTool` | Agent changes settings | — (settings UI only) | Active | **Reject** | [19 §config](../parity/19-command-surface.md#settings-commands) | — |

---

## Tools — CueCode-only (no CC analog) {#tools-cuecode-only}

| CueCode tool | Purpose | A/A/H | Spec | Phase |
|--------------|---------|-------|------|-------|
| `list_specs` | Spec index | Active | [08 §list-specs](../agent/08-agent-tools-and-skills#tool-list-specs) | 1 |
| `read_spec` | Load spec body | Active | [08 §read-spec](../agent/08-agent-tools-and-skills#tool-read-spec) | 1 |
| `update_spec` | Propose spec edit | Active | [08 §update-spec](../agent/08-agent-tools-and-skills#tool-update-spec) | 1 |
| `link_spec` | Attach spec to session | Active | [08 §link-spec](../agent/08-agent-tools-and-skills#tool-link-spec) | 1 |
| `checkpoint` | Save session snapshot | Active/Hybrid | [08 §checkpoint](../agent/08-agent-tools-and-skills#tool-checkpoint) | 3 |
| `rewind` | Restore checkpoint | Active | [08](../agent/08-agent-tools-and-skills) | 3 |
| `trust_query` | Explain trust rules | Active | [08 §trust-query](../agent/08-agent-tools-and-skills#tool-trust-query) | 4 |

---

## Services layer {#services}

| CC service | Purpose | CueCode equivalent | Decision | Spec |
|------------|---------|-------------------|----------|------|
| `api/` | LLM client | `language_models` + cloud gateway | **Adapt** | [10 §models](../ops/10-infrastructure#models), [cloud/07](../harness/cloud/07-model-gateway.md) |
| `mcp/` | MCP management | `context_server` | **Adopt** | [10 §mcp](../ops/10-infrastructure) |
| `oauth/` | Auth | CueCode OAuth / BYOK keychain | **Adapt** | [03 §decouple-cloud](../core/03-fork-and-rebrand) |
| `lsp/` | LSP manager | `project` + `language` | **Adopt** | [02](../core/02-current-architecture) |
| `compact/` | Context compression | `agent` auto-compact + context budget UI | **Adapt** | [05 §context-budget](../core/05-innovations#context-budget) |
| `extractMemories/` | Stop-hook memory | Memory extract job | **Adapt** | [17 §extract](../parity/17-memory-and-context.md#extract-pipeline) |
| `teamMemorySync/` | Team mem sync | Team memory sync (opt-in cloud) | **Defer** | [17 §team-memory](../parity/17-memory-and-context.md#team-memory) |
| `plugins/` | Plugin loader | — | **Reject** | [20 §plugins](../parity/20-platform-integrations.md#plugins) |
| `analytics/` | Feature flags | Local metrics opt-in | **Adapt** | [11 §logging](../ops/11-metrics-and-success#logging) |
| `policyLimits/` | Org policy | Trust hard-deny + intent (v1 local) | **Adapt** | [08 §hard-deny](../agent/08-agent-tools-and-skills#hard-deny) |

---

## Harness subsystems {#harness-subsystems}

| CC subsystem | Path | CueCode port | Decision | Spec |
|--------------|------|--------------|----------|------|
| Coordinator mode | `coordinator/` | Orchestrate intent | **Adapt** | [18 §coordinator](../parity/18-teams-and-tasks.md#coordinator) |
| Stop hooks | `query/stopHooks.ts` | Post-turn hook in `agent::Thread` | **Adapt** | [17 §stop-hooks](../parity/17-memory-and-context.md#stop-hooks) |
| Sidechain storage | `types/logs.ts`, AgentTool | `~/.config/cuecode/sessions/.../sidechains/` | **Adapt** | [local §B.5](../harness/local/01-agent-harness.md#b-5-async-artifacts-on-disk) |
| Forked agent | `AgentTool/forkSubagent.ts` | Background spawn + parent summary | **Defer** | [local §C.3](../harness/local/01-agent-harness.md#c-3-fork-research) |
| Tool result storage | `utils/toolResultStorage.ts` | Tool spill under session dir | **Adapt** | [local §A.4](../harness/local/01-agent-harness.md#a-4-context-economy) |
| File history / rewind | `utils/fileHistory.ts`, `/rewind` | Checkpoint stack | **Adapt** | [05 §checkpoint](../core/05-innovations#checkpoint-stack) |
| Memdir | `memdir/` | CueCode memory store | **Adapt** | [17](../parity/17-memory-and-context.md) |
| Local agent tasks | `tasks/LocalAgentTask/` | GPUI task pills + notification rail | **Adapt** | [local §async-ui](../harness/local/01-agent-harness.md#async-ui) |
| Permission hooks | `hooks/toolPermission/` | Intent + trust + confirm UI | **Adapt** | [08 §permissions](../agent/08-agent-tools-and-skills#permissions) |
| Bridge | `bridge/` | Native IDE (no external bridge) | **Adapt** | [20 §ide-native](../parity/20-platform-integrations.md#ide-native) |
| Remote sessions | `remote/` | Cloud session resume (opt-in) | **Defer** | [20 §remote](../parity/20-platform-integrations.md#remote-sessions) |

---

## Slash commands — index {#commands-index}

Full mapping: [19-command-surface](../parity/19-command-surface.md). Summary by category:

| Category | CC commands (count) | Parity target |
|----------|---------------------|---------------|
| Session lifecycle | `/resume`, `/rewind`, `/clear`, `/export`, `/share`, `/rename`, `/session` | **Adapt** → GPUI thread sidebar + checkpoints |
| Context & memory | `/compact`, `/context`, `/memory`, `/files`, `/add-dir` | **Adapt** → context budget + memory browser |
| Agent harness | `/plan`, `/tasks`, `/agents`, `/skills` | **Adapt** → intent + lane panel |
| Git & review | `/commit`, `/review`, `/diff`, `/pr_comments`, `/branch` | **Adapt** → review panel + git UI |
| MCP & config | `/mcp`, `/config`, `/permissions`, `/sandbox-toggle` | **Adapt** → settings + intent |
| Models & cost | `/model`, `/cost`, `/usage`, `/effort`, `/fast` | **Adapt** → agent header + cost panel |
| Auth & cloud | `/login`, `/logout`, `/upgrade`, `/extra-usage` | **Adapt** → CueCode cloud (optional) |
| IDE & bridge | `/ide`, `/bridge`, `/desktop`, `/mobile` | **Adapt/Reject** → native IDE; reject mobile/desktop handoff |
| Diagnostics | `/doctor`, `/status`, `/feedback`, `/release-notes` | **Adapt** → command palette + runbooks |
| UI chrome | `/theme`, `/vim`, `/voice`, `/keybindings`, `/color`, `/output-style` | **Adopt** → Zed settings (most exist) |
| Plugins & hooks | `/hooks`, plugin commands, `/reload-plugins` | **Reject/Defer** plugins; **Adapt** hooks → Cursor hooks |
| Internal / ant-only | `/heapdump`, `/stats`, `thinkback`, stickers, etc. | **Reject** for product v1 |

---

## Command rows (top 60) {#commands-top-60}

| CC command | CueCode surface | Decision | Spec anchor |
|------------|-----------------|----------|-------------|
| `/compact` | Context budget → Compact now | **Adapt** | [19 §context](../parity/19-command-surface.md#context-commands) |
| `/context` | Context budget panel | **Adapt** | [19](../parity/19-command-surface.md) |
| `/memory` | Memory browser (settings / agent) | **Adapt** | [17 §ui](../parity/17-memory-and-context.md#memory-ui) |
| `/resume` | Thread sidebar → load session | **Adapt** | [19 §session](../parity/19-command-surface.md#session-commands) |
| `/rewind` | Checkpoint timeline → Rewind | **Adapt** | [19](../parity/19-command-surface.md) |
| `/clear` | New thread / clear composer | **Adapt** | [19](../parity/19-command-surface.md) |
| `/export` | Export session bundle | **Adapt** | [19 §export](../parity/19-command-surface.md#export-share) |
| `/share` | Share session artifact (opt-in) | **Defer** | [20 §share](../parity/20-platform-integrations.md#session-share) |
| `/plan` | Plan intent + plan gate | **Adapt** | [19 §harness](../parity/19-command-surface.md#harness-commands) |
| `/tasks` | Lane panel + task list | **Adapt** | [18](../parity/18-teams-and-tasks.md) |
| `/agents` | Builtin agent picker / spawn | **Adapt** | [19](../parity/19-command-surface.md) |
| `/skills` | Skills browser + `/skill` | **Adapt** | [19](../parity/19-command-surface.md) |
| `/commit` | Git commit from review footer | **Adapt** | [19 §git](../parity/19-command-surface.md#git-commands) |
| `/review` | Review intent + unified review | **Adapt** | [16 flow B](../parity/16-end-to-end-flows.md#flow-b-ship-verify) |
| `/diff` | Unified review → Diffs tab | **Adapt** | [09 §review](../design/09-ui-ux-spec) |
| `/mcp` | MCP settings modal | **Adapt** | [19 §mcp](../parity/19-command-surface.md#mcp-commands) |
| `/config` | Settings → Agent | **Adapt** | [19](../parity/19-command-surface.md) |
| `/permissions` | Trust settings + intent | **Adapt** | [19](../parity/19-command-surface.md) |
| `/sandbox-toggle` | Sandbox badge / intent | **Adapt** | [04 §sandbox-core](../core/04-sandbox-core) |
| `/doctor` | Diagnostics runbook UI | **Adapt** | [10 §runbooks](../ops/10-infrastructure#operational-runbooks) |
| `/model` | Model picker in agent header | **Adapt** | [19 §model](../parity/19-command-surface.md#model-commands) |
| `/cost` | Token/cost panel | **Adapt** | [19](../parity/19-command-surface.md) |
| `/usage` | Usage stats | **Adapt** | [11 §engagement](../ops/11-metrics-and-success#engagement) |
| `/login` | CueCode cloud sign-in (optional) | **Adapt** | [cloud/01 §runtime](../harness/cloud/01-overview.md#runtime-modes) |
| `/logout` | Sign out cloud | **Adapt** | [19](../parity/19-command-surface.md) |
| `/init` | Project onboarding wizard | **Adapt** | [19 §init](../parity/19-command-surface.md#onboarding-commands) |
| `/ide` | — (CueCode *is* the IDE) | **Reject** | [20 §ide-native](../parity/20-platform-integrations.md#ide-native) |
| `/bridge` | — | **Reject** | [20](../parity/20-platform-integrations.md) |
| `/desktop` | — | **Reject** | [20](../parity/20-platform-integrations.md) |
| `/mobile` | — | **Reject** | [20](../parity/20-platform-integrations.md) |
| `/vim` | Editor vim mode | **Adopt** | Zed existing |
| `/theme` | Theme settings | **Adopt** | Zed existing |
| `/voice` | Voice input | **Defer** | [20 §voice](../parity/20-platform-integrations.md#voice) |
| `/hooks` | Cursor hooks + stop hooks | **Adapt** | [20 §hooks](../parity/20-platform-integrations.md#hooks) |
| Plugin marketplace | `/plugin`, etc. | **Reject** | [20 §plugins](../parity/20-platform-integrations.md#plugins) |
| `/security-review` | Verification + Review intent | **Adapt** | [16 flow G](../parity/16-end-to-end-flows.md#flow-g-security-review) |
| `/brief` | Session brief attachment | **Adapt** | [17 §brief](../parity/17-memory-and-context.md#session-brief) |
| `/fast` | Fast model hint on lane | **Adapt** | [local §builtin-agents](../harness/local/01-agent-harness.md#builtin-agents) |
| `/effort` | Model effort setting | **Adapt** | [19 §model](../parity/19-command-surface.md#model-commands) |
| `/rename` | Thread title edit | **Adopt** | `agent_ui` thread rename |
| `/branch` | Git branch UI | **Adopt** | Zed git |
| `/pr_comments` | GitHub PR comments | **Defer** | [20 §integrations](../parity/20-platform-integrations.md#github) |
| `/add-dir` | Additional workspace roots | **Adopt** | Zed workspace |
| `/files` | @-mention file picker | **Adopt** | Composer mentions |
| `/clear` caches | — | **Reject** | Dev-only |
| `/heapdump` | — | **Reject** | Dev-only |
| `/stats` | Local metrics dashboard | **Adapt** | [11 §dashboard](../ops/11-metrics-and-success#weekly-sdsW-dashboard) |
| `/feedback` | Feedback form | **Defer** | Phase 6 |
| `/release-notes` | What's new | **Adapt** | Onboarding |
| `/upgrade` | CueCode cloud upgrade | **Adapt** | Cloud billing |
| `/extra-usage` | Usage limits | **Adapt** | Cloud |
| `/rate-limit-options` | Rate limit UX | **Adapt** | [10 §models](../ops/10-infrastructure) |
| `/remote-setup` | Remote dev env | **Defer** | [20 §remote](../parity/20-platform-integrations.md#remote-sessions) |
| `/install-github-app` | GitHub app | **Defer** | [20 §github](../parity/20-platform-integrations.md#github) |
| `/install-slack-app` | Slack | **Reject** | v1 non-goal |
| `/chrome` | Browser extension | **Reject** | — |
| `/stickers` | Easter egg | **Reject** | — |
| `/btw` | Easter egg | **Reject** | — |
| `/thinkback` | Internal feature | **Reject** | — |
| `/passes` | Billing passes | **Reject** | CueCode billing model TBD |
| `/tag` | Session tags | **Defer** | [19 §session](../parity/19-command-surface.md#session-commands) |
| `/copy` | Copy transcript | **Adapt** | Conversation export |
| `/exit` | Close panel | **Adopt** | GPUI |

---

## Decision summary {#decision-summary}

| Decision | Tools | Commands (top 60) | Services | Harness |
|----------|-------|-------------------|----------|---------|
| **Adopt** | 12 | 8 | 2 | 0 |
| **Adapt** | 28 | 38 | 7 | 10 |
| **Defer** | 8 | 7 | 1 | 1 |
| **Reject** | 2 | 10 | 1 | 0 |
| **CueCode-only** | 7 | — | — | — |

See [01-parity-decisions](./01-parity-decisions.md) for policy and acceptance gates.

---

## Related specs {#related}

| Doc | Role |
|-----|------|
| [15-competitive-parity](../parity/15-competitive-parity.md) | Program thesis + phases |
| [16-end-to-end-flows](../parity/16-end-to-end-flows.md) | E2E acceptance |
| [17-memory-and-context](../parity/17-memory-and-context.md) | Memory gap |
| [18-teams-and-tasks](../parity/18-teams-and-tasks.md) | Teams/tasks gap |
| [19-command-surface](../parity/19-command-surface.md) | Command mapping |
| [20-platform-integrations](../parity/20-platform-integrations.md) | Platform gap |
| [21-ai-surfaces](../parity/21-ai-surfaces.md) | Copilot vs agent split |

---

## Document status {#status}

| Field | Value |
|-------|-------|
| Status | Living inventory |
| Last updated | 2026-06-17 |
| Source revision | claude-code-main leak 2026-03-31 |
