# AI surface map — CueCode / Zed fork

Use this when implementing or reviewing AI features to find the right crate fast.

## Core agent sandbox

| Crate | Role |
|-------|------|
| `agent_ui` | Agent panel, conversation, composer, model/mode pickers, diff review UI |
| `acp_thread` | ACP thread state, plan, streaming, terminals, session updates |
| `agent` | Native agent, tools, thread, system prompt assembly |
| `agent_settings` | Agent profiles, permissions, compaction prompts |
| `agent_skills` | SKILL.md discovery and catalog |
| `agent_servers` | Native + ACP external agent connections |
| `acp_tools` | ACP tooling helpers |
| `action_log` | Agent edit tracking, reject/undo |

### Key files

- `agent_ui/src/agent_panel.rs` — panel shell
- `agent_ui/src/conversation_view.rs` — thread UI
- `agent_ui/src/message_editor.rs` — composer
- `acp_thread/src/acp_thread.rs` — session entity
- `agent/src/thread.rs` — native completion request build
- `agent/src/templates/system_prompt.hbs` — base agent instructions

## Models and inference

| Crate | Role |
|-------|------|
| `language_model` | Registry, configured model |
| `language_model_core` | Request/message types |
| `language_models` | Provider implementations (Ollama, Anthropic, …) |
| `language_models_cloud` | zed.dev cloud models — de-emphasize for CueCode |
| `prompt_store` | Prompt builder for inline/terminal assist |

## Inline and complementary AI

| Crate / module | Role |
|----------------|------|
| `agent_ui/inline_assistant.rs` | Editor inline assist |
| `agent_ui/terminal_inline_assistant.rs` | Terminal assist |
| `agent_ui/buffer_codegen.rs` | Buffer generation |
| `edit_prediction` | Tab-style prediction (Zeta/Mercury) |
| `edit_prediction_ui` | Prediction UI in editor |

## Context and extensions

| Crate | Role |
|-------|------|
| `context_server` | MCP / context server registry |
| `agent_ui/context_server_configuration.rs` | MCP config UI |
| `agent/src/tools/context_server_registry.rs` | Tool bridge |

## Legacy / de-emphasize for CueCode

| Crate | Note |
|-------|------|
| `copilot`, `copilot_chat`, `copilot_ui` | Parallel AI path — hide or disable in CueCode |
| `ai_onboarding` | Replace zed.dev account onboarding |
| `language_models_cloud` | Not default for CueCode |

## Planned CueCode crates

| Crate | Role |
|-------|------|
| `cuecode_specs` | `.cursor/specs/` index + plan sync |
| `cuecode_sandbox` | Intent profiles, trust graph, checkpoints |

See `.cursor/specs/core/06-system-design.md`.

## Settings

- `assets/settings/default.json` — `"agent"`, model defaults, tool_permissions, auto_compact
- User: `~/.config/cuecode/settings.json` (after rebrand)

## Specs

- Strategy: `.cursor/specs/agent/13-ai-maxxing.md`
- Sandbox: `.cursor/specs/core/04-sandbox-core.md`
- Innovations: `.cursor/specs/core/05-innovations.md`
- UI: `.cursor/specs/design/09-ui-ux-spec.md`
