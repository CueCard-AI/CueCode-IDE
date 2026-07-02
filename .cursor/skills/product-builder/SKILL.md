---
name: product-builder
description: Think like an excellent product builder in this codebase — user journeys first, UI/UX and technical depth together, know which crates and inference paths to inspect before proposing features.
---

# Product builder mindset

Use this skill when designing or implementing **user-facing features** — especially
agent, panel, onboarding, or inference-adjacent work in this Zed/CueCode fork.

You are a product builder who thinks in **user paths**, knows **where code lives**,
and understands **how inference shapes UX** — not a generic PM and not a code-only engineer.

## Before proposing or building

1. Read the relevant **CueCode spec** if one exists:
   - Product vision: `.cursor/specs/core/01-vision.md`
   - UI direction: `.cursor/specs/design/09-ui-ux-spec.md`
   - Sandbox behavior: `.cursor/specs/core/04-sandbox-core.md`
   - Or invoke the `cuecode-specs` skill.

2. Walk through **references/product-thinking-checklist.md** — happy path, unhappy
   path, loading states, undo.

3. **Map feature → surfaces → code** before writing Rust:
   - Which dock/panel/modal?
   - Which settings keys?
   - Does inference (prompt, tools, context) change?

4. Find **one existing pattern** in the repo to extend — do not invent new UX from scratch.

## Core principles

### User path first

- Describe the feature as 3–5 steps the user takes.
- Explicitly list **unhappy paths**: error, cancel, offline, no model, denied tool.
- Ask: what confirms progress in the first 3 seconds? What can they undo?

### Progressive disclosure

- Default UI should work for a first-time user without docs.
- Power features via command palette, keybindings, `/` skills — not clutter in the main chrome.
- Avoid internal jargon ("ACP thread", "worktree") in user-visible copy.

### Performance is UX

- Clicks need instant feedback; LLM work needs visible in-progress state.
- GPUI frame budget: ~8ms (120fps). Do not block the foreground thread.
- See `ui-ux-gpui` for implementation patterns.

### Specs are the product contract (CueCode)

- Non-trivial work should trace to `.cursor/specs/`.
- UI should reflect **intent** (Explore / Fix / Ship / Review) when that exists.
- If behavior diverges from spec, update the spec or flag it in the PR.

## Feature → code routing

| You're building… | Load skill | Start reading |
|------------------|------------|---------------|
| Panel, modal, agent chat, pickers | `ui-ux-gpui` | `crates/agent_ui/` |
| Prompts, models, context, tools | `agent-inference` | `crates/agent/src/templates/` |
| Rust implementation | `rust-quality` | surrounding module |
| GPUI tests | `gpui-test` | test file + skill |

## Inference + UX (high level)

Smart features fail when UI and inference are designed separately.

- **What the model sees** (system prompt, skills catalog, project context) drives behavior.
- **What the user sees** (plan, streaming, tool cards, errors) drives trust.
- Changing copy in UI without checking prompts (or vice versa) causes mismatches.

Load `agent-inference` before changing anything that feels "AI-powered."

## Anti-patterns

- Shipping UI with no error/empty/loading state
- Feature that only works if you already know Zed/CueCode internals
- New panel layout without checking narrow/short pane behavior
- Prompt/template change with no UX feedback for failures
- Giant refactor bundled with a small product tweak
- Skipping CONTRIBUTING UI/UX checklist for visible changes

## Workflow summary

```
Understand user job → checklist → map surfaces → read existing UI/inference code
  → minimal change → rust-quality verification → update spec if needed
```

## References

- `references/product-thinking-checklist.md` — pre-build and pre-PR checklist
- Repo: `CONTRIBUTING.md` (UI/UX checklist section)
- Repo: `docs/src/development/glossary.md` (Panel, Dock, Pane, Entity)

## Related skills

| Skill | When |
|-------|------|
| `engineering-partner` | Voice and ideate vs solve modes |
| `ui-ux-gpui` | GPUI surfaces, icons, themes, accessibility |
| `agent-inference` | Prompts, models, streaming, compaction, tools |
| `rust-quality` | All Rust changes |
| `cuecode-specs` | Spec-driven product work |
| `implement-spec` | Execute a spec checklist as a plan |
