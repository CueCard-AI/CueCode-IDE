# UX review checklist

From `CONTRIBUTING.md` — use before PRs that touch UI.

## Accessibility / ergonomics

- [ ] Keyboard shortcuts work as intended
- [ ] Shortcuts discoverable (tooltips, menus, docs)
- [ ] Usable without a mouse (keyboard-only navigation)
- [ ] Mouse actions work (drag, context menus, resize, scroll)
- [ ] Looks good in light and dark mode
- [ ] Hover states and focus indicators clear and consistent

## Responsiveness

- [ ] Scales in narrow panes (side-by-side splits)
- [ ] Scales on short panes (13" laptops)
- [ ] High-DPI / Retina displays
- [ ] Resizing panes/windows keeps UI usable
- [ ] Dialogs/modals centered and within viewport

## Platform consistency

- [ ] Usable on Windows, Linux, macOS (if shipping cross-platform)
- [ ] Respects system fonts, scaling, input methods

## Performance

- [ ] Instant feedback on user interaction
- [ ] Slow work (LLM generation) shows progress indication
- [ ] Large files/projects don't degrade unduly
- [ ] Frames ≤8ms where possible (120fps target)

## Consistency

- [ ] Matches design language (spacing, typography, icons)
- [ ] Icons follow `crates/icons/README.md`
- [ ] Terminology and tone consistent with app
- [ ] Interactions match existing patterns (tabs, modals, errors)

## Internationalization and text

- [ ] Strings concise, clear, unambiguous
- [ ] No internal jargon in user-visible copy

## User paths and edge cases

- [ ] Happy path defined
- [ ] Unhappy path defined (errors, rejections, invalid states)
- [ ] Offline vs online behavior
- [ ] Missing/corrupted/delayed data handled
- [ ] Error messages actionable and on-brand

## Discoverability and learning

- [ ] First-time user can figure it out without docs (or onboarding exists)
- [ ] Undo/redo path where applicable
- [ ] Power features discoverable but not intrusive
- [ ] Progressive disclosure from beginner → expert
