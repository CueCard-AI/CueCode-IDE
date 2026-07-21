# `language` — agent notes

Language registry, buffers, syntax, diagnostics, toolchains — and the **model provider settings** (BYOK). Load `agent-inference` + `rust-quality`.

## Where to look

`language_settings.rs` (provider config — the BYOK surface), `language_registry.rs`, `language.rs`, `buffer.rs` (+ `buffer/`), `syntax_map.rs` (+ `syntax_map/`), `diagnostic.rs` / `diagnostic_set.rs`, `toolchain.rs`, `outline.rs`, `manifest.rs`, `modeline.rs`, `task_context.rs`, `proto.rs`.

## Traps

- **`language_settings.rs` holds the `language_models.openai_compatible` provider map** — this is what edit predictions fall back to when `edit_predictions.provider` is `open_ai_compatible_api` and no dedicated endpoint is set. Don't break that fallback (shares keychain credentials per URL).
- **Edit-prediction provider config (`edit_predictions.*`) also parses here** — keep the schema in sync with `settings_content/src/language.rs` / `language_model.rs`.
- **Language registry is lazy** — languages load on first use; don't assume a language is registered at startup.
- **Syntax/buffer work is hot** — keep parsing off the foreground thread; use the existing chunked/incremental paths.

## Verify

```bash
cargo test -p language
./script/clippy -p language
```
