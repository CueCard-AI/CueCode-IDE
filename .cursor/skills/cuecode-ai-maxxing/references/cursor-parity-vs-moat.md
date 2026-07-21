# Cursor parity vs CueCode moat

Use when deciding whether to build a feature because "Cursor has it."

## Match (baseline expectations)

Users expect these from any serious AI IDE — CueCode must not feel broken:

- Agent chat with tool use (read, edit, grep, terminal)
- Model selection / BYOK or local
- Multi-file edits with review
- Skills or rules for project context
- Inline assistance in editor (optional but expected)
- MCP / external context (growing expectation)

Implement via existing Zed surfaces; improve review and trust, don't remove.

## Leap (CueCode moat — prioritize here)

Build here instead of incremental chat tweaks:

| Moat | Why Cursor is weaker |
|------|----------------------|
| **Session-first UX** | File-tree + composer mental model |
| **Spec-driven loop** | Specs live outside repo or in rules only |
| **Intent profiles** | One global permission mode |
| **Trust graph** | Static allowlists |
| **Checkpoint / rewind** | Mostly file-level undo |
| **Multi-lane agents** | Single thread default |
| **Unified review surface** | Diff in chat stream |
| **Native performance** | Electron |
| **OS sandbox** | Limited terminal isolation |
| **Local-first default** | Vendor account pressure |

If a feature doesn't strengthen a moat row, deprioritize vs innovation backlog.

## Skip or defer

- Pure Cursor UI clone with no sandbox angle
- zed.dev billing / account dependencies
- Another LLM provider for provider-count bragging
- Autonomous unsupervised agent mode
- Features better as extensions (languages, themes)

## Decision template

For proposal "Build X because Cursor has X":

1. **Parity or moat?** (table above)
2. **Loop closure?** plan → act → review → checkpoint
3. **Native advantage used?** GPUI, sandbox, deep editor integration
4. **Spec section?** link in PR

If parity-only: implement minimally, then move on to moat work.
