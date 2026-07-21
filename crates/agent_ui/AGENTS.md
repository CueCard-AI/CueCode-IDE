# `agent_ui` — agent notes

Agent panel, conversation view, planning hub, thread UI. Load `ui-ux-gpui` + `agent-inference` + `rust-quality`.

## Traps

- **`AgentPanel::set_base_view`: no global `window.refresh()`.** Scope refresh to Plan↔Threads only. A global refresh was the root cause of empty agent bodies / nested-update risk.
- **Agent body: one `flex_1 min_h_0 overflow_hidden` child keyed by surface id** (`agent-surface-threads` / `-plan` / `-terminal`). Don't stack flex wrappers.
- **`ThreadView` is two-zone:** `[flex_1 scroll]` (messages **+ callouts inside the scroll`) + `[flex_shrink_0 composer]` pinned at bottom. Callouts must **not** sit between list and composer as flex siblings — that breaks scroll.
- **`ConversationView`: one `flex_1 min_h_0` wrapper** around the active thread child.
- **Plan tab in the panel uses flat `panel_background` only — no `elevation_3`.** (Detached windows keep elevation; the docked panel does not.)
- **Panel surfaces are Threads | Plan.** Terminal was removed from the agent panel tabs — terminal threads live in `terminal_view`, not here.

## Verify (AP1–AP5)

Composer expand (Cmd-Option-Esc) works; expanded composer bottom buttons display; font size Cmd-+ / Cmd--; scroll works in messages + Plan; files droppable into panel. Plus Plan ↔ Threads ×5 with no layout corruption.

## Where to look

`agent_panel.rs` (`set_base_view`, `render`), `conversation_view/` (`conversation_view.rs`, `thread_view.rs`), `planning_hub.rs`.
