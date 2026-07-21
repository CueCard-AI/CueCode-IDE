---
title: How to Migrate from Cursor to CueCode
description: "Guide for migrating from Cursor to CueCode — keybindings, command palette, agent UX, and what has no equivalent."
---

# How to Migrate from Cursor to CueCode

This guide helps you move from Cursor without losing muscle memory. CueCode is **not** a Cursor clone — it is a GPUI-native, agent-first editor — but you can get most day-to-day shortcuts and palette habits working quickly.

## Install CueCode

Download from [cuecode.dev/download](https://cuecode.dev/download) or run from source:

```bash
cd CueCode-IDE
cargo run -p cuecode
```

Open a project:

```bash
cuecode .
```

## Enable the Cursor keymap

CueCode ships a **Cursor (beta)** base keymap. Enable it in your settings file (`{#action cuecode::OpenSettingsFile}`):

```json
{
  "base_keymap": "Cursor"
}
```

Or open {#action cuecode::OpenSettings}, search for **Base Keymap**, and choose **Cursor**.

This loads `assets/keymaps/macos/cursor.json` (or the Linux variant) on top of defaults. Common bindings:

| Cursor habit | CueCode action | Shortcut (macOS, Cursor keymap) |
|--------------|----------------|----------------------------------|
| Open/focus chat | `agent::ToggleFocus` | `Cmd+L`, `Cmd+I` |
| Add selection to chat | `agent::AddSelectionToThread` | `Cmd+L` (in editor) |
| Inline edit | `assistant::InlineAssist` | `Cmd+K` |
| Agent settings | `agent::OpenSettings` | `Cmd+Shift+J` |
| New chat thread | `agent::NewThread` | `Cmd+R` (in agent panel) |
| Model picker | `agent::ToggleModelSelector` | `Cmd+/` (in agent panel) |
| Profile/mode | `agent::ToggleProfileSelector` | `Cmd+.` (in agent panel) |

See [Agent Panel](../ai/agent-panel.md) and [Agent Settings](../ai/agent-settings.md) for configuring LLM providers (including Z.ai GLM via OpenAI-compatible endpoints).

## Command palette: Cursor names that work

CueCode's palette (`{#action command_palette::Toggle}`, `Cmd+Shift+P`) indexes **Cursor-style search aliases** for common commands. You can type partial queries like **developer** or **reload window**:

| Cursor / VS Code habit | CueCode command |
|------------------------|-----------------|
| **Developer: Reload Window** | {#action cuecode::ReloadWindow} — restarts the app |
| **Agent / Cursor settings** | {#action agent::OpenSettings} |
| **Preferences: Open Settings** | {#action cuecode::OpenSettings} |
| **Open Settings (JSON)** | {#action cuecode::OpenSettingsFile} |
| **Command Palette** | {#action command_palette::Toggle} |
| **Open Chat** | {#action agent::ToggleFocus} |
| **Inline Edit** | {#action assistant::InlineAssist} |
| **Keyboard Shortcuts** | {#action cuecode::OpenKeymap} |
| **New Chat** | {#action agent::NewThread} |

Aliases live in `crates/command_palette/src/cursor_palette_aliases.rs`. Add more there as users report gaps.

### Exact command aliases (optional)

For mnemonic shortcuts (type an alias and press Enter), use `command_aliases` in settings — same mechanism as [Vim command mnemonics](../vim.md#command-mnemonics):

```json
{
  "command_aliases": {
    "reload": "cuecode::ReloadWindow",
    "agentsettings": "agent::OpenSettings"
  }
}
```

## Settings and projects

| Cursor | CueCode |
|--------|---------|
| `.cursor/` rules & specs | `.cursor/` (same layout in this fork) + `.cuecode/settings.json` for project settings |
| Cursor subscription LLM | No direct path — use [API access](../ai/use-api-access.md), OpenAI-compatible providers, or [External Agents](../ai/use-an-existing-subscription.md#cursor) |
| Workspace `.code-workspace` | No equivalent — use **Add Folder to Project** or multiple projects in one window ([Windows & Projects](../windows-and-projects.md)) |
| Reload Window (soft) | Settings hot-reload for most JSON changes; use {#action cuecode::ReloadWindow} when you need a full restart |

Import VS Code editor settings during onboarding if you used Cursor's VS Code base — see [VS Code migration](./vs-code.md#import-settings-from-vs-code).

## Agent UX differences (intentional)

Do not expect 1:1 parity with Cursor Composer, subagent UI, or Cursor Cloud. CueCode focuses on:

- Native GPUI agent panel (not Electron)
- Local/BYOK models by default
- Spec index + `@spec` in agent context (this fork)
- Planned sandbox / checkpoint loop (roadmap)

| Cursor feature | CueCode status |
|----------------|----------------|
| Composer modes | Agent **profiles** (Ask / Write / etc.) — different model |
| Cursor Tab / predictions | CueCode **edit predictions** — separate system |
| Cursor subscription models | Use BYOK or compatible providers |
| Background agents / cloud harness | Cloud harness on roadmap — see harness specs |
| `@` mentions (files, docs) | Supported — `@spec`, `@file`, images when model supports vision |
| Screenshots in chat | Paste or drag — requires vision-capable model with `images: true` |

## What to do first

1. Set `base_keymap: "Cursor"`.
2. Configure an agent model in {#action agent::OpenSettings} (e.g. Z.ai GLM via OpenAI-compatible).
3. Open a **Write** profile thread and try `@spec` + a spec question.
4. Use **Developer: Reload Window** (palette) after large config changes if behavior looks stale.

## Related guides

- [VS Code migration](./vs-code.md) — editor settings import
- [Agent quick start](../ai/quick-start.md)
- [Use API access](../ai/use-api-access.md) — OpenAI-compatible, Anthropic-compatible providers
