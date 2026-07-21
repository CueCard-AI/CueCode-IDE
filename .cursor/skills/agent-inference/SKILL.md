---
name: agent-inference
description: Understand and change agent inference in this fork — system prompts, context assembly, model requests, streaming, compaction, tools, and ACP — with a code map to the right crates.
---

# Agent inference architecture

Use when changing **how the agent thinks** — prompts, context, models, tools,
streaming, compaction, or external ACP agents — not just UI chrome around chat.

Load `product-builder` if the change is user-facing; load `ui-ux-gpui` if you
also change conversation UI. Load `rust-quality` for all code changes.

## Pipeline (mental model)

```
User input (MessageEditor / ACP prompt)
    → Thread state (messages, plan, running tools)
    → build_completion_request (native) OR ACP AgentConnection
    → LanguageModelRequest { system, history, tools, temperature, … }
    → Provider (Ollama, Anthropic, OpenAI, …)
    → Stream chunks → AcpThread / Thread → ConversationView
    → Tool calls → agent tools → results → loop
    → auto_compact when context exceeds threshold
```

Detailed diagram: `references/inference-flow.md`

Template guide: `references/prompt-editing-guide.md`

## Code map

| Concern | Location |
|---------|----------|
| System prompt (Handlebars) | `crates/agent/src/templates/system_prompt.hbs` |
| Other prompts | `crates/agent/src/templates/*.hbs` |
| Request assembly | `crates/agent/src/thread.rs` — `build_completion_request`, `build_request_messages` |
| Project context in prompt | `ProjectContext`, `maintain_project_context` in `agent.rs` |
| Skills catalog in prompt | `agent_skills` + `skill_tool.rs` |
| User rules / AGENTS.md | `UserAgentsMd` in agent settings |
| Native agent server | `crates/agent/src/native_agent_server.rs` |
| ACP bridge | `crates/agent_servers/src/acp.rs`, `acp_thread::AgentConnection` |
| Thread UI state | `crates/acp_thread/src/acp_thread.rs` |
| Streaming buffer | `StreamingTextBuffer` in `acp_thread.rs` |
| Tool registry + exec | `crates/agent/src/tools/*.rs` |
| Tool permissions | `agent_settings`, `assets/settings/default.json` → `agent.tool_permissions` |
| Model registry | `crates/language_model/src/registry.rs` |
| Providers | `crates/language_models/src/provider/*.rs` |
| Request types | `crates/language_model_core/src/request.rs` |
| Agent settings | `crates/agent_settings/` |
| Compaction prompts | `crates/agent_settings/src/prompts/` |
| Inline assist inference | `inline_assistant.rs`, `terminal_inline_assistant.rs` |
| Buffer codegen | `buffer_codegen.rs` |
| Edit prediction (separate) | `crates/edit_prediction*` |

## Design questions (answer before coding)

1. **System vs user vs tool messages** — what belongs where? (affects caching and behavior)
2. **Always-on vs on-demand context** — skills use catalog-only in system, body via `skill` tool
3. **Which tools this turn?** — filtered in `build_completion_request` from running turn
4. **CompletionIntent** — user prompt vs subagent vs inline assist (grep `CompletionIntent`)
5. **Compaction** — `agent.auto_compact` in `default.json`; threshold and enabled flag
6. **Failure UX** — no model, rate limit, tool deny — must reach UI (`agent_ui`, toasts)

## Prompt cache (skills)

From `crates/agent_skills/README.md`:

- Skill **catalog** (name + description) lives in system prompt
- Skill **body** loads separately — editing body doesn't invalidate cache
- Changing skill name/description **does** invalidate cache

Avoid bloating system prompt with full spec bodies — index + on-demand read.

## Changing inference safely

1. Read current `system_prompt.hbs` and trace one request in `thread.rs`
2. Grep for `CompletionIntent` if behavior differs by intent
3. If adding tools, implement in `crates/agent/src/tools/` + register in agent
4. Update `tool_permissions` defaults if new tool is risky
5. Test with local model (Ollama) — small thread, watch tool loop
6. If user-visible strings change, sync `agent_ui` copy

## ACP external agents

External agents (Cursor CLI, etc.) connect via `agent_servers::acp`.

CueCode/Zed still applies **deny/confirm** overrides on top of external permission modes.

Native agent is the integration point for CueCode-specific prompt/context (specs, intents).

## Settings keys (common)

`assets/settings/default.json` → `"agent"`:

- `default_model` — provider + model id
- `model_parameters` — temperature, etc.
- `tool_permissions` — default + per-tool regex rules
- `auto_compact` — enabled, threshold
- `inline_assistant_use_streaming_tools`

User overrides: `~/.config/zed/settings.json` (→ `cuecode` after rebrand)

## Anti-patterns

- Huge new system prompt section without measuring context cost
- Tool that bypasses `tool_permissions`
- Silent failure when model unavailable
- Prompt change without checking unhappy-path UI
- Duplicating inference logic outside `thread.rs` / ACP path

## References

- `references/inference-flow.md`
- `references/prompt-editing-guide.md`

## Related skills

| Skill | When |
|-------|------|
| `product-builder` | User-facing behavior and specs |
| `ui-ux-gpui` | ConversationView, streaming UX, model picker |
| `rust-quality` | Implementation quality |
| `cuecode-specs` | Spec-driven context injection (planned) |
