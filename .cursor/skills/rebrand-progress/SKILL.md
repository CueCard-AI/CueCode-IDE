---
name: rebrand-progress
description: Tracks and verifies CueCode Phase 0 rebrand and Zed cleanup passes A–E. Use when checking off rebrand tasks, updating progress in specs, after completing a cleanup pass, or when the user asks what's done on fork/rebrand work.
---

# Rebrand progress tracker

Canonical human checklist: `CueCode-IDE/.cursor/specs/core/03-zed-reference-cleanup-phases.md#progress`

Machine verification: `CueCode-IDE/script/rebrand-progress.sh`, `CueCode-IDE/script/qa-agent-ollama.sh`

## When to use

- User asks what's done / what's left on rebrand or cleanup passes
- After finishing Pass A–E (or Phase 0) work
- Before marking roadmap or spec checkboxes
- When progress in specs feels out of sync with reality

## Quick verify

```bash
cd CueCode-IDE
./script/rebrand-progress.sh          # rebrand-check summary
./script/rebrand-progress.sh --full   # + qa-p0.sh (~1 min)
./script/qa-agent-ollama.sh           # headless agent + Ollama (requires local Ollama)
```

Exit 0 = all automated gates green. Exit non-zero = fix failures before checking boxes.

## Update checklist (required workflow)

1. Run `./script/rebrand-progress.sh --full` from `CueCode-IDE/`.
2. Open `#progress` in `03-zed-reference-cleanup-phases.md` (repo root and `CueCode-IDE/.cursor/specs/` mirrors — keep both in sync).
3. Mark items `[x]` only when:
   - **(gate)** — corresponding `rebrand-check.sh` / `qa-p0.sh` check passes, or
   - **(manual)** — you performed the human step (note in checkbox text if ambiguous).
4. Update **Last verified** date at top of `#progress`.
5. Sync related boxes in `build-plans/phases/X-Y-*.md` Status, `07-implementation-roadmap.md#progress`, and cleanup phases for Phase 0 gates.
6. Add a one-line entry to `#changelog` in the cleanup phases doc if a pass completed.

Do **not** check boxes from memory. Gates are the source of truth for `(gate)` items.

## Three trackers — how they relate

| Doc | Role |
|-----|------|
| `build-plans/phases/*.md` | **Check off here for Build phase X.Y** — one file per sub-phase |
| `03-zed-reference-cleanup-phases.md#progress` | Phase 0 rebrand passes A–E |
| `07-implementation-roadmap.md#progress` | Product phase rollup P0–P6 |
| `03-fork-and-rebrand.md#file-checklist` | File-by-file reference — link to `#progress`, don't duplicate status |

## Checkbox legend

| Label | Meaning |
|-------|---------|
| `(gate)` | Proven by `rebrand-check.sh` or `qa-p0.sh` |
| `(manual)` | Human-only (icon review, E2E auto-update, packet capture) |

## Pass → gate mapping

| Pass | rebrand-check section | Extra |
|------|----------------------|-------|
| A | `Pass A — settings and keymap comments` | |
| B | `Pass B — remaining user-visible strings` | |
| C | `Pass C — filename and artifact renames` | E2E auto-update = manual → `03-pass-c-auto-update-smoke-test.md` |
| D | `Pass D — docs and legal` | `rg '\bZed\b' docs/` → allowlist only |
| E | *(none yet)* | Document exceptions → `03-pass-e-upstream-ci-exceptions.md` |

Phase 0 core: all of `rebrand-check.sh` + `qa-p0.sh --full`.

## Do not fix (never check off as "rebranded")

See `#do-not-fix` in the cleanup phases doc: `zed://`, `.zed/`, `ZED_*`, WIT `zed:extension`, GPL About upstream line, legacy icon theme alias, internal crate names.

## Reporting to the user

Summarize as:

1. **Automated:** pass/fail from `rebrand-progress.sh`
2. **Progress doc:** which pass sections are fully `[x]`
3. **Open manual items:** unchecked `(manual)` and Pass E
4. **Next action:** one concrete step

## Related files

- `CueCode-IDE/script/rebrand-check.sh` — gate definitions
- `CueCode-IDE/script/qa-p0.sh` — launch smoke (+ Step 4b agent when Ollama up)
- `CueCode-IDE/script/qa-agent-ollama.sh` — headless agent via eval-cli + Ollama
- `.cursor/specs/delivery/build-plans/phases/` — sub-phase execution docs
- `.github/workflows/cuecode_rebrand_check.yml` — CI (repo root)
- `.cursor/specs/core/03-pass-e-upstream-ci-exceptions.md` — Pass E upstream defer list
