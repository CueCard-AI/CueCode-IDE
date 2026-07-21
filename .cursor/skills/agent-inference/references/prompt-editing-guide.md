# Prompt editing guide

## Which file to edit

| Goal | File(s) |
|------|---------|
| Core agent personality + tool rules | `crates/agent/src/templates/system_prompt.hbs` |
| Experimental agent behavior | `experimental_system_prompt.hbs` |
| Edit file tool prompts | `edit_file_prompt_*.hbs`, `create_file_prompt.hbs` |
| Diff judging | `diff_judge.hbs` |
| Thread compaction | `crates/agent_settings/src/prompts/summarize_thread_prompt.txt`, `compaction_prompt.txt` |
| Terminal inline assist | `prompt_store` / `PromptBuilder` — grep `terminal_assistant` |
| Inline editor assist | grep `inline_assistant` in `agent_ui` |

Templates are rendered via `SystemPromptTemplate` in `thread.rs` — check struct fields for available Handlebars variables.

## Before editing a template

1. Read the full template — small wording changes affect tool use and tone
2. Grep for template name in `crates/agent/` to find render site
3. Check if `CompletionIntent` uses a different template path
4. Consider prompt cache: stable prefix is better for cost/latency

## Adding context to the agent

| Mechanism | Use when |
|-----------|----------|
| Edit `system_prompt.hbs` | Always-on instructions |
| `ProjectContext` | Repo-specific dynamic context |
| Agent skills | Task-specific playbooks (progressive disclosure) |
| `UserAgentsMd` / rules files | User-level persistent rules |
| MCP / context servers | External tools and data |
| CueCode specs (planned) | Product specs via `cuecode_specs` |

Prefer skills or on-demand reads over stuffing system prompt.

## Adding a tool

1. Create `crates/agent/src/tools/my_tool.rs`
2. Implement tool trait pattern from sibling tools (e.g. `grep_tool.rs`)
3. Register in agent tool list (grep `register` / tool collection in `agent.rs`)
4. Add JSON schema for model via tool's `input_schema`
5. Set `agent.tool_permissions` defaults if destructive
6. Document in tool description — model reads this to decide when to call

## Model / provider changes

- Provider impl: `crates/language_models/src/provider/<name>.rs`
- Registry wiring: `language_models/src/language_models.rs`, settings UI
- Default model: `assets/settings/default.json` → `agent.default_model`

For CueCode fork: point default away from `zed.dev` to local provider.

## Debugging inference issues

| Symptom | Check |
|---------|-------|
| Wrong tone/behavior | `system_prompt.hbs`, profile settings |
| Tool not called | Tool description, `available_tools` in request |
| Tool always called | Tool description too broad |
| Context overflow | `auto_compact`, message count, project context size |
| Streaming jank | `StreamingTextBuffer`, ConversationView update frequency |
| ACP mismatch | `agent_servers/acp.rs`, external agent version |

Enable `RUST_LOG=agent=debug,language_model=debug` for verbose logs (dev builds).

## Tests

- Tool evals: `crates/agent/src/tools/evals/`
- Thread logic: grep `#[test]` in `thread.rs`, `agent.rs`
- UI: `agent_ui` test_support + `gpui-test` skill

Do not rely only on manual chat testing for tool changes.
