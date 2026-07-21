---
name: implement-spec
description: Load and follow a CueCode product spec or build phase from `.cursor/specs/` (and `.cuecode/` when present). Use when the user links a spec, mentions `@spec`, asks to implement a build phase, or invokes `Build phase X.Y`.
---

# Implement from spec

Use when a session has an **active spec** (pinned path), the user points at a build phase, or references `.cursor/specs/`.

## Before coding

1. **Find the artifact** — pinned chip, `@spec` mention, **Planning Hub** (palette: `CueCode: Planning Hub`), or `list_specs`.
2. **Read the full body** — not just the index row. Specs use markdown anchors (`{#anchor-id}`); jump to the section that matches the task.
3. **Check build phase** — delivery docs under `.cursor/specs/delivery/build-plans/phases/` list ordered tasks with file paths and exit criteria. Prefer invoking **`Build phase X.Y`** on that file only.
4. **Note dependencies** — do not skip blocked phases; mark docs done only after manual QA when the phase file requires it.

## While implementing

- Match **existing crate conventions** (GPUI, agent tools, settings).
- Keep diffs **scoped to the spec** — no drive-by refactors.
- Wire **telemetry** when the spec names `cuecode.*` events.
- If the spec conflicts with code, **call it out** before guessing.

## Verify

- Run the phase **Verify** block (usually `cargo build -p …` from `apps/CueCode-IDE/`).
- Complete **manual QA** steps in the build-phase file before checking boxes.
- Update phase status + roadmap only when exit criteria are actually met.

## Related

- Planning Hub design: `.cursor/specs/design/16-planning-hub.md`
- Spec index + tools: `.cursor/specs/agent/08-agent-tools-and-skills.md`
- UI patterns: `.cursor/specs/design/09-ui-ux-spec.md`
- Rust quality: `rust-quality` skill, `./script/clippy`
