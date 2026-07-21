---
name: ui-ux-gpui
description: Design and implement UI in this GPUI/Zed fork — workspace, panel, dock model, agent surfaces, icons, themes, accessibility, and the CONTRIBUTING UX checklist.
---

# UI/UX in GPUI (this codebase)

Use when building or changing **visible UI** — agent panel, modals, pickers,
settings screens, workspace chrome, icons, or themes.

Load `product-builder` first if the feature scope is unclear.

## Mental model: where things live on screen

```
Window
├── Docks (left / right / bottom) → Panels (Agent, Project, Terminal, …)
└── Center → Panes → Editors, multibuffers, terminals
```

- **Panel** — docked sidebar tool (`AgentPanel`, project panel). Implements `Panel` trait.
- **Pane** — center area tabs (editors).
- **Modal / Picker** — floats above UI; picker selects from a list.

Full glossary: `docs/src/development/glossary.md`

Detailed file map: `references/ui-surfaces.md`

## Code map (start here)

| UX area | Primary files |
|---------|----------------|
| Agent panel + threads | `crates/agent_ui/src/agent_panel.rs`, `conversation_view.rs` |
| Message composer | `message_editor.rs`, `inline_prompt_editor.rs` |
| Model / mode / profile | `agent_model_selector.rs`, `mode_selector.rs`, `profile_selector.rs` |
| Agent settings UI | `agent_configuration.rs`, modals under `agent_configuration/` |
| Diff / review | `agent_diff.rs` |
| Shared agent UI bits | `crates/agent_ui/src/ui/` |
| Workspace layout | `crates/workspace/` |
| Generic UI kit | `crates/ui/`, `crates/component/` |
| Title bar | `crates/title_bar/` |
| Icons | `crates/icons/`, `assets/icons/` — see `crates/icons/README.md` |
| Themes | `crates/theme/`, `theme_settings` |
| Default settings copy | `assets/settings/default.json` |
| Component previews | `./script/storybook`, `crates/component_preview/` |

CueCode-specific UX spec: `.cursor/specs/design/09-ui-ux-spec.md`

## GPUI implementation rules

From `.rules` / `AGENTS.md` — violations cause panics or broken render:

| Rule | Detail |
|------|--------|
| `cx.notify()` | After state changes that affect rendering |
| No nested `update` | Don't `entity.update` while already inside its update closure |
| `window` before `cx` | When both are parameters |
| Callbacks after `cx` | `.on_click`, `.on_action` come after `cx` in signature |
| Async + entities | Use `WeakEntity` in `cx.spawn(async move \|this, cx\| ...)` |
| Tasks | Don't drop `Task` silently — detach, await, or store |
| Strings in UI | Prefer `SharedString` for cheap cloned text |

Implement `Render` on `Entity<T>` views, or `RenderOnce` for one-shot components.

## Design language

- Spacing/typography: match neighboring elements in the same panel — read before writing.
- Icons: 16×16 viewBox, ~1.2px stroke, optical alignment — **no random SVG imports**.
  See `crates/icons/README.md`.
- Light + dark: test both; use theme tokens, not hardcoded colors.
- Terminology: consistent with rest of app (rebrand to CueCode where applicable).

## UX checklist

Before shipping UI changes, complete `references/ux-review-checklist.md`
(sourced from `CONTRIBUTING.md`).

High-signal items:

- Instant feedback on interaction; spinner/streaming for LLM work
- Keyboard + mouse; focus indicators; tooltips for shortcuts where helpful
- Narrow and short panes; dialogs stay in viewport
- Actionable error messages

## Workflow

1. **Find analog** — grep `agent_ui` for similar component (popover, list, header bar).
2. **User path** — 3–5 steps; note empty/error/loading (product-builder checklist).
3. **Implement** — smallest GPUI change; reuse `ui` / `component` patterns.
4. **Verify** — light/dark; resize panel; keyboard path.
5. **Test** — GPUI tests if behavior is non-trivial (`gpui-test` skill).
6. **Lint** — `./script/clippy` (`rust-quality` skill).

## CueCode UI direction (from specs)

When building agent UX for this fork, align with `.cursor/specs/design/09-ui-ux-spec.md`:

- Intent switcher + sandbox badge in agent header (planned)
- Spec linker / spec browser (planned)
- Unified review panel: Plan | Diffs | Terminal | Spec (planned)
- Composer-first layout option (planned)

Extend existing `AgentPanel` / `ConversationView` rather than parallel UI.

## Anti-patterns

- New icon from Lucide without cleanup pass and design consistency
- Modal with no escape / dismiss path
- State change without `cx.notify()`
- User-visible "Zed" strings in CueCode-branded areas (during rebrand)
- One-off CSS-like magic numbers when neighbors use design tokens

## References

- `references/ui-surfaces.md` — structure diagram + file index
- `references/ux-review-checklist.md` — full CONTRIBUTING UI/UX checklist

## Related skills

| Skill | When |
|-------|------|
| `product-builder` | Scope, user paths, spec alignment |
| `agent-inference` | UI tied to streaming, tools, model picker |
| `rust-quality` | Rust/GPUI correctness |
| `gpui-test` | Test failures, seeds, parking |
