# `editor` — agent notes

The core `Editor` type — drives both the code editor and all input fields; display layer for LSP inlays/completions; edit-prediction rendering. Load `ui-ux-gpui` + `rust-quality`.

## Where to look

`editor.rs` (the core), `display_map.rs` (+ `display_map/`), `completions.rs`, `edit_prediction.rs`, `editor_settings.rs`, `diagnostics.rs`, `code_actions.rs`, `code_context_menus.rs`, `code_lens.rs`, `bookmarks.rs`, `bracket_colorization.rs`, `clangd_ext.rs`, `clipboard.rs`, `actions.rs`.

## Traps

- **`Editor` is reused for input fields**, not just code — changes to default behavior ripple into the agent composer / prompts. Check both paths.
- **Edit predictions render here** (`edit_prediction.rs`) — the *provider* lives in `edit_prediction/` + `language/`; this crate is the inline rendering/accept path. Don't put provider logic here.
- **`display_map` is the display layer** for inlay hints, completions, hovers, fold — many "editor shows wrong thing" bugs are display-map layering, not buffer content.
- **Don't panic on indexing** — editor ranges can be stale across edits; use the range-transform helpers, not raw indices.

## Verify

```bash
cargo test -p editor
./script/clippy -p editor
```
