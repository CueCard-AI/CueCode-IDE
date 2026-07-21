# `edit_prediction` — agent notes

Inline Tab completions (edit predictions). BYOK bridge to OpenAI-compatible providers. Load `agent-inference` + `rust-quality`.

## Traps

- **`provider: "open_ai_compatible_api"` reuses the agent's configured `language_models.openai_compatible` provider** (URL + first model) when no dedicated endpoint is set — so one API setup powers both chat and inline predictions. Credentials are shared per-URL via the keychain. Don't break this fallback by requiring an explicit EP endpoint.
- **`open_ai_compatible.rs` appends `/completions` to the base URL** if not already present. The provider endpoint **must support legacy `/completions` (FIM-style)** requests. A code/coder/FIM model gives the best results.
- **GLM prompt format is inferred for any model starting with `glm`** (`cuecode/src/app/edit_prediction_registry.rs`). If you add a new model family, add its prompt-format inference there rather than hardcoding in the provider.
- **`EditPredictionProvider::CueCode` (hosted Zeta) was removed from the provider picker.** Don't reintroduce a hosted/built-in provider — edit predictions are BYOK-only now.
- **A chat endpoint that lacks `/completions` FIM will surface visible errors, not a silent no-op.** That's intentional — don't swallow it.

## Verify

```bash
cargo check -p edit_prediction -p edit_prediction_ui -p cuecode
cargo test -p edit_prediction
```

## Where to look

`edit_prediction/src/open_ai_compatible.rs` (URL handling, prompt format), `edit_prediction_ui/src/edit_prediction_button.rs` (provider picker), `cuecode/src/app/edit_prediction_registry.rs` (GLM/family inference), `language/src/language_settings.rs` (BYOK fallback). User docs: `docs/src/ai/edit-prediction.md`.
