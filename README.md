# CueCode

CueCode is a fork of [Zed](https://github.com/zed-industries/zed) — a spec-driven agent IDE with local/BYOK models by default, no zed.dev sign-in wall, and separate user data from Zed.

**Upstream:** Zed (GPL-3.0-or-later). See About in the app for full attribution.

---

## Specs & build plan

In the **CueInference monorepo**, canonical specs live at repo-root `.cursor/specs/`
and are synced into this package as `.cursor/specs/` before public publish.
Prefer the in-package copies (works here and in the public shadow):

- [Spec index](.cursor/specs/00-README.md)
- [Fork & rebrand](.cursor/specs/core/03-fork-and-rebrand.md)
- [Master build plan](.cursor/specs/delivery/build-plans/00-master-build-plan.md)
- [Implementation roadmap](.cursor/specs/delivery/07-implementation-roadmap.md)

Phase 0 exit criteria and QA scripts are in [03-fork-and-rebrand §manual QA](.cursor/specs/core/03-fork-and-rebrand.md#manual-qa).

**Monorepo orientation:** [`../../AGENTS.md`](../../AGENTS.md) · [`../../MONOREPO.md` § nested IDE](../../MONOREPO.md#working-in-the-nested-ide).

---

## Developing CueCode

**How to run end to end:** [`RUNBOOK.md`](./RUNBOOK.md) (J0 first build, J1 Plan
E2E fixture, J2 harness-stub, J9 spec sync). GPL-safe only — full monorepo
journeys stay in CueInference’s private `RUNBOOK.md`.

From this package directory (`apps/CueCode-IDE/` in CueInference; repo root in
the public [CueCard-AI/CueCode-IDE](https://github.com/CueCard-AI/CueCode-IDE) shadow):

```bash
cargo run -p cuecode --bin cuecode
```

Or use the local helper:

```bash
./script/cuecode-local
```

Fresh config paths (no collision with Zed):

| Platform | Config / data |
|----------|----------------|
| macOS | `~/Library/Application Support/CueCode/` |
| Linux | `~/.config/cuecode/` |
| Windows | `%LOCALAPPDATA%\CueCode\` |

### Quickstart

```bash
# 1. build + run (first build is slow — thousands of crates)
# from CueInference root:
cd apps/CueCode-IDE
cargo run -p cuecode --bin cuecode

# 2. open a project at startup
./target/debug/cuecode /path/to/project

# 3. run the agent panel: Cmd+R (macOS) / Ctrl+R (linux) → "agent: open"
```

Config lives in `assets/settings/default.json` (defaults shipped with the app)
overlaid by user settings in the per-platform config dir above. Edit user
settings via **Cmd+,** → Settings Editor, or by editing the JSON directly.

For the canonical Plan E2E loop with the QA fixture (CueInference monorepo):

```bash
# from CueInference root
./script/clone-qa-fixture-repo
cd apps/CueCode-IDE && ./script/run-cuecode-dev ../../cuecode-testing-repo
```

## Bring your own key (BYOK)

CueCode is bring-your-own-key: the agent and inline edit predictions run against
**your own** model providers — no zed.dev sign-in wall, no hosted billing.

- **Agent chat** — configure an OpenAI-compatible provider (GLM via Z.ai,
  OpenAI, Ollama, etc.) under `language_models.openai_compatible` in settings.
  Add the API key via the in-app keychain (Settings → LLM Providers).
- **Edit predictions (inline Tab completions)** — by default reuse the agent's
  configured OpenAI-compatible provider, so a single API setup powers both chat
  and inline predictions. The provider endpoint must support legacy
  `/completions` (FIM-style) requests; a coder/FIM model gives the best results.

To use a dedicated endpoint/model for predictions:

```json [settings]
{
  "edit_predictions": {
    "provider": "open_ai_compatible_api",
    "open_ai_compatible_api": {
      "api_url": "https://your-provider.example/v1",
      "model": "your-coder-model",
      "prompt_format": "infer",
      "max_output_tokens": 64
    }
  }
}
```

Full setup, modes (`eager` / `subtle`), and provider list:
[`docs/src/ai/edit-prediction.md`](docs/src/ai/edit-prediction.md) and
[`docs/src/ai/llm-providers.md`](docs/src/ai/llm-providers.md).

### Agent UX defaults

Shipped defaults (`assets/settings/default.json`, `agent` section) are tuned for
a Cursor-like compact streaming experience:

- `thinking_display: "preview"` — compact thinking blocks
- `expand_edit_card: false` — edit cards collapsed by default
- `expand_terminal_card: false` — terminal cards collapsed by default

## Agent surfaces

- **Agent panel** — Threads (chat) and Plan surfaces; see
  [`docs/src/ai/agent-panel.md`](docs/src/ai/agent-panel.md).
- **Planning Hub / Layout Studio** — spec-driven build plans, multi-column
  PulseBoard layout. Specs: `.cursor/specs/design/16-planning-hub.md`,
  `.cursor/specs/design/17-layout-studio.md`.
- **Terminal threads** — `docs/src/ai/terminal-threads.md`.

---

## CLI

```bash
cargo build -p cuecode --bin cuecode
./target/debug/cuecode --help    # product name should say CueCode
```

Install CLI to PATH (macOS, from the app): **Command Palette → install cli**.

---

## Packaging

| Platform | Script |
|----------|--------|
| macOS | `script/bundle-mac` |
| Linux | `script/bundle-linux` |
| Windows | `script/bundle-windows.ps1` |
| Flatpak | `script/flatpak/bundle-flatpak` |

Rebrand regression gate:

```bash
./script/rebrand-check.sh
```

Phase 0 smoke (automated QA-P0):

```bash
./script/qa-p0.sh
```

Runs with `--user-data-dir /tmp/cuecode-qa-*` so your real CueCode config is not modified.

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Upstream Zed contribution docs remain below for reference where the fork has not diverged.

---

## Licensing

CueCode is based on Zed and inherits GPL-3.0-or-later obligations. Third-party license compliance uses `script/generate-licenses` and `script/licenses/zed-licenses.toml` (upstream naming).
