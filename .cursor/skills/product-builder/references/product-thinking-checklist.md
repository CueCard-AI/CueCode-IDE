# Product thinking checklist

Use at the start of a feature and again before opening a PR.

## User and job

- [ ] Who is the user in this moment (first-time / returning / power)?
- [ ] What job are they hiring this feature to do (one sentence)?
- [ ] Happy path in ≤5 concrete steps
- [ ] Unhappy paths documented: error, cancel, empty state, offline, no model, permission denied

## UX quality

- [ ] First 3 seconds: user knows something happened (feedback or spinner)
- [ ] Loading state for anything slower than ~200ms (especially LLM turns)
- [ ] Clear way to undo, go back, or dismiss
- [ ] Works in light and dark theme (if UI)
- [ ] Works in narrow agent panel / short laptop screen (if UI)
- [ ] Keyboard path exists; shortcut or palette discoverable (if applicable)
- [ ] Copy is concise; no insider jargon in user-visible strings

## Technical mapping

- [ ] Which **Panel / Dock / Modal / Editor** owns this? (see `ui-ux-gpui`)
- [ ] Which **settings keys** in `assets/settings/default.json` or user settings?
- [ ] Does **inference** change (prompt, tools, context)? (see `agent-inference`)
- [ ] Relevant **`.cursor/specs/`** section read and linked in PR

## Ship bar

- [ ] Closest existing pattern in repo identified and followed
- [ ] `./script/clippy` + targeted tests (see `rust-quality`)
- [ ] CONTRIBUTING UI/UX checklist satisfied for visible changes
- [ ] Spec updated if behavior differs from plan
