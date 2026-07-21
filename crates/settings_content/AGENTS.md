# `settings_content` — agent notes

The **typed settings schema** — the parsed-content model for `settings.json` (what's *parseable*), not the UI or storage. Load `rust-quality`.

## Where to look

`settings_content.rs` (root), `agent.rs` (agent settings), `editor.rs`, `language.rs`, `language_model.rs`, `workspace.rs`, `terminal.rs`, `theme.rs`, `title_bar.rs`, `project.rs`, `action.rs`, `extension.rs`, `fallible_options.rs`, `merge_from.rs`, `serde_helper.rs`.

## Traps

- **This is the schema, not the UI or storage.** Rendering lives in `settings_ui`; persistence/loading in `schemer` + `settings`. Changing what's parseable changes user settings compat — be conservative with renames/defaults.
- **`agent.rs` holds the agent UX defaults** the app ships — `thinking_display`, `expand_edit_card`, `expand_terminal_card`, etc. The *shipped* defaults live in `assets/settings/default.json`; this file defines the *shape* + documented defaults. Keep the two consistent.
- **`merge_from.rs` is the layering engine** (defaults ← local ← project ← …). New settings must merge correctly across layers — don't bypass it.
- **`fallible_options.rs`** is for validated/parse-with-error options; prefer it over silent `Option<T>` when a bad value should produce a user-visible error.
- **Adding a setting = touch the schema here + `default.json` + often `settings_ui` + maybe an `all-settings` doc.** Don't add a setting in only one place.

## Verify

```bash
cargo test -p settings_content
./script/clippy -p settings_content
```
