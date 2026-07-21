# `agent` — agent notes

Agent thread, inference request assembly, prompts, compaction, tools. Load `agent-inference` (required) + `rust-quality`.

## Traps

- **Defaults shipped in `assets/settings/default.json` (`agent` section):** `thinking_display: "preview"`, `expand_edit_card: false`, `expand_terminal_card: false`. These tune the Cursor-like compact streaming UX — change deliberately.
- **Request assembly lives in `thread.rs`** — `build_completion_request`, `build_request_messages`. System prompt is `templates/system_prompt.hbs` (Handlebars); other prompts in `templates/*.hbs`. Edit prompts via the `agent-inference` skill's prompt-editing guide, not by hand-rolling strings.
- **`auto_compact` fires when context exceeds threshold.** Compaction **must preserve ticket id + pin modes** (Phase 1.5 contract) — don't drop session-coupled spec/ticket state when compacting.
- **Project context** (`ProjectContext`, `maintain_project_context` in `agent.rs`) and the skills catalog (`agent_skills` + `skill_tool.rs`) are injected into the prompt — be aware they're in-context, not tool-called.

## Verify

```bash
cargo test -p agent
./script/clippy -p agent
```

## Where to look

`thread.rs` (request assembly, compaction), `agent.rs` (`ProjectContext`), `templates/` (prompts), `db.rs`, `tools/`. Detailed pipeline + code map: `.cursor/skills/agent-inference/SKILL.md` + `references/inference-flow.md`.
