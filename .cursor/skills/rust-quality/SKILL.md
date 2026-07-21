---
name: rust-quality
description: Write high-quality Rust in this Zed/CueCode fork — match repo conventions, GPUI patterns, error handling, tests, and ./script/clippy before finishing any Rust change.
---

# High-quality Rust in this codebase

Use this skill whenever writing or reviewing Rust in this repository — new crates,
agent tools, GPUI views, or edits to existing crates.

## Before writing code

1. **Read surrounding code** in the same file and sibling modules. Match naming,
   error types, async patterns, and import style.
2. **Prefer extending existing files** unless the change is a new logical component.
   Avoid many small new files.
3. **Check scope** — if the task is in `.cursor/specs/`, read the relevant spec
   section first (or invoke the `cuecode-specs` skill).
4. **Identify the layer**:
   - GPUI view / panel → follow entity, `cx.notify()`, no nested `update`
   - Agent tool → propagate errors to UI; respect tool permissions
   - Pure logic → keep UI-agnostic; test without GPUI when possible

## Non-negotiable rules (from repo `.rules` / `AGENTS.md`)

### Correctness and errors

- Prioritize **correctness and clarity** over micro-optimizations unless asked.
- **No `unwrap()` / `expect()`** in production paths. Use `?`, `match`, or
  `if let Err(...)`.
- **No silent error drops** — never `let _ =` on fallible operations.
  - Propagate with `?` when the caller should handle
  - Use `.log_err()` when intentionally ignoring but want visibility
- **No panicking indexing** — use `.get()`, iterators, or explicit bounds checks.
- Async failures must reach the **UI layer** with meaningful feedback where applicable.

### Style and structure

- **Full words** for variables (`queue`, not `q`).
- **No `mod.rs`** — use `src/module_name.rs`, not `src/module_name/mod.rs`.
- New crates: set `[lib] path = "descriptive_name.rs"` in `Cargo.toml` (not default `lib.rs`).
- **Comments**: only explain non-obvious *why*, never restate what the code does.
- **No drive-by refactors** or creative extras beyond the requested change.

### Async and clones

When spawning async work, scope clones narrowly:

```rust
cx.spawn({
    let handle = handle.clone();
    async move |cx| {
        // use handle
        Ok(())
    }
});
```

Use **variable shadowing** to limit clone lifetimes in async blocks.

## GPUI-specific (read `.rules` GPUI section for full detail)

Summary of traps:

| Trap | Do instead |
|------|------------|
| Nested `entity.update` while already in `update` | Use inner `cx` from closure; never re-enter same entity |
| Holding `Entity` across await incorrectly | Use `WeakEntity` in `cx.spawn(async move \|this, cx\| ...)` |
| Dropping `Task` accidentally | `.detach()`, `.detach_and_log_err(cx)`, await, or store in field |
| Forgetting to rerender | Call `cx.notify()` after state changes that affect UI |
| `window` vs `cx` order | `window` before `cx` when both present |
| Callbacks after `cx` | Event handlers and actions come **after** `cx` parameter |

For GPUI **tests**, invoke or follow the **`gpui-test`** skill — do not use
`smol::Timer::after` for timeouts when using `run_until_parked()`; use
`cx.background_executor().timer(...)`.

## Agent / CueCode-specific

When editing under `crates/agent*`, `crates/acp_*`, `crates/agent_ui`:

- Tools return errors the conversation can surface — avoid opaque `anyhow!` only.
- Respect existing tool permission patterns in `tool_permissions`.
- System prompt / template changes: keep diffs minimal; check Handlebars templates in `crates/agent/src/templates/`.
- Skills live in `.cursor/skills/<name>/SKILL.md` — strict frontmatter validation.

## Workflow before calling a change "done"

1. **Format** — match existing file (rustfmt via editor or `cargo fmt` on touched crates).
2. **Lint** — run `./script/clippy` (not bare `cargo clippy`).
3. **Test** — run targeted tests:
   ```sh
   cargo test -p <crate-name> <test_filter> -- --nocapture
   ```
   For GPUI tests, see `gpui-test` skill.
4. **Scope check** — diff should be one logical change; no unrelated files.
5. **New public API** — doc comments on public items if the crate already documents them.

See `references/pre-submit-checklist.md` for a copy-paste checklist.

## Patterns to copy from this repo

When unsure, grep for similar code:

| Need | Look in |
|------|---------|
| GPUI view | `crates/agent_ui/src/*.rs`, `crates/workspace/` |
| Entity + subscription | `_subscriptions: Vec<Subscription>` pattern in panel code |
| Agent tool | `crates/agent/src/tools/*_tool.rs` |
| Settings | `crates/agent_settings/`, `assets/settings/default.json` |
| Error propagation in async UI | `Task::ready(Err(...))`, `.log_err()`, `cx.emit` |

## Anti-patterns (reject in review)

- New dependency for a one-liner utility
- `unwrap()` "because it can't fail" without comment in test-only code
- Giant refactor bundled with a bugfix
- New file per tiny helper
- Ignoring failing clippy in touched code
- GPUI state mutation without `cx.notify()`
- `mod.rs` or abbreviated variable names

## When to load other skills

| Situation | Skill |
|-----------|-------|
| Tone, brainstorm vs fix | `engineering-partner` |
| GPUI test failure, seeds, parking | `gpui-test` |
| Product scope, user journeys, specs | `product-builder` |
| Panels, modals, agent UI, icons | `ui-ux-gpui` |
| Prompts, models, tools, streaming | `agent-inference` |
| CueCode product / specs / rebrand | `cuecode-specs` |
| Creating a new agent skill | `create-skill` (built-in) |
| Implementing from a spec checklist | `implement-spec` |

## Reference

Full guidelines: repository root `AGENTS.md` and `.rules` (GPUI section is authoritative for UI code).

Build: `./script/clippy`, `cargo run` from repo root.
