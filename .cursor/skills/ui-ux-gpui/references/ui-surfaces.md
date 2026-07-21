# UI surfaces — code map

## Window layout (Zed / CueCode)

```
┌──────────────────────────────────────────────────────────────┐
│ Title bar (platform_title_bar, title_bar)                    │
├──────────┬─────────────────────────────────────┬───────────┤
│ Left     │ Center (Panes)                      │ Right     │
│ Dock     │  └─ Editor / multibuffer / terminal  │ Dock      │
│          │                                     │           │
│ Project  │                                     │ Agent     │
│ Panel    │                                     │ Panel     │
│ (etc.)   │                                     │ (etc.)    │
├──────────┴─────────────────────────────────────┴───────────┤
│ Bottom Dock — Terminal, Debug, …                             │
└──────────────────────────────────────────────────────────────┘
```

## Crate → responsibility

| Crate | UI role |
|-------|---------|
| `workspace` | Window, docks, panes, panel registration |
| `panel` | Panel trait, dock behavior |
| `agent_ui` | Agent panel, conversation, message editor, agent settings UI |
| `project_panel` | File tree |
| `terminal_view` | Terminal panel |
| `title_bar` / `platform_title_bar` | Window chrome |
| `ui` | Buttons, lists, popovers, shared widgets |
| `component` | Component layout helpers |
| `theme` | Colors, typography tokens |
| `icons` | Icon name → SVG mapping |

## agent_ui file index

| File | UI surface |
|------|------------|
| `agent_panel.rs` | Main agent dock panel |
| `conversation_view.rs` | Thread messages, tool cards, plan UI |
| `message_editor.rs` | Composer, mentions, attachments |
| `agent_model_selector.rs` | Model picker |
| `mode_selector.rs` | Agent mode |
| `profile_selector.rs` | Agent profiles |
| `agent_configuration.rs` | Agent settings entity |
| `agent_diff.rs` | Inline diff review |
| `model_selector_popover.rs` | Model popover |
| `ui/model_selector_components.rs` | Shared model selector pieces |
| `threads_archive_view.rs` | Archived threads |
| `agent_registry_ui.rs` | External agent registry page |

## Settings that affect agent UI

File: `assets/settings/default.json` — `"agent"` section

- `dock`, `default_width`, `default_height`, `flexible`
- `limit_content_width`, `max_content_width`
- `single_file_review`
- `button` (status bar agent toggle)

## Glossary pointers

`docs/src/development/glossary.md` defines:

- Entity, View, Element, Render
- Panel, Dock, Pane, Center, Workspace
- Picker, Modal, Action, Focus

## Storybook

```sh
./script/storybook              # all previews
./script/storybook <name>       # one component under components/
```

Use when adding reusable visual components.
