# Inference flow

## Native agent path

```mermaid
sequenceDiagram
    participant User
    participant MessageEditor
    participant AgentPanel
    participant Thread as agent Thread
    participant LM as LanguageModel provider
    participant Tools as agent tools

    User->>MessageEditor: type + send
    MessageEditor->>AgentPanel: submit prompt
    AgentPanel->>Thread: append user message
    Thread->>Thread: build_completion_request
    Note over Thread: system_prompt.hbs + ProjectContext + skills catalog
    Thread->>LM: LanguageModelRequest
    LM-->>Thread: stream tokens / tool calls
    Thread->>Tools: execute tool
    Tools-->>Thread: tool result
    Thread->>LM: continue with tool output
    Thread->>AgentPanel: notify UI
    AgentPanel->>User: ConversationView update
```

## ACP external agent path

```mermaid
sequenceDiagram
    participant User
    participant ConversationView
    participant ACP as AgentConnection
    participant AcpThread
    participant External as External ACP agent

    User->>ConversationView: prompt
    ConversationView->>ACP: prompt(session_id, content)
    ACP->>External: ACP protocol
    External-->>AcpThread: SessionUpdate chunks
    Note over AcpThread: plan, tool_call, message, thought
    AcpThread->>ConversationView: render entries
```

## Compaction path

When token usage exceeds `agent.auto_compact.threshold`:

1. Thread identifies compaction point
2. Summarization prompt from `agent_settings/src/prompts/`
3. Older messages replaced with summary
4. User may see compaction indicator in UI

Grep: `auto_compact`, `compact`, `summarize_thread`

## Key types

| Type | Crate |
|------|-------|
| `LanguageModelRequest` | `language_model_core` |
| `LanguageModelRequestMessage` | `language_model_core` |
| `CompletionIntent` | `language_model` / agent |
| `SessionUpdate` | `agent_client_protocol` |
| `AcpThread` | `acp_thread` |

## Streaming

- Native: provider stream → thread messages
- ACP: `SessionUpdate::AgentMessageChunk`, `AgentThoughtChunk`
- UI smoothing: `StreamingTextBuffer` in `acp_thread.rs` (gradual reveal)

When changing streaming, test perceived latency and flicker — product concern.

## Context sources in system prompt

Typical native agent system prompt includes:

- Base instructions (`system_prompt.hbs`)
- Available tools list
- Project context (files, diagnostics summary)
- Skills catalog (name + description + path)
- User `AGENTS.md` / rules content
- Sandboxing notice (if enabled)
- Date, model name

Subagent and inline assist use different templates/intents.
