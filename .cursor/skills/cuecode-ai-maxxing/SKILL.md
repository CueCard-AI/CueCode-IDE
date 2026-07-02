---
name: cuecode-ai-maxxing
description: CueCode AI-maxxing doctrine — agentic sandbox strategy, beat sidebar-first IDEs, full AI surface map, and checklist for features that close the agent loop.
---

# CueCode AI-maxxing

Use when **designing, ideating, or implementing AI/agent features** in this fork —
especially when the goal is to turbo-charge Zed **beyond sidebar-first tools**
(Cursor-class UX).

**Strategic spec:** `.cursor/specs/agent/13-ai-maxxing.md` (read for north star and checklist)

**Harness specs:** `.cursor/specs/harness/README.md` — **cloud (Model B, default):** `harness/cloud/` · **local (in-process):** `harness/local/01-agent-harness.md`

## Doctrine (one paragraph)

CueCode is an **agentic coding sandbox**, not an editor with chat bolted on.
Max AI by tightening: **intent → spec → act → review → checkpoint** — with
native GPUI speed, local models, progressive trust, and multi-lane sessions.
Every feature must tighten that loop or leverage native/sandbox moats — not
add chat for chat's sake.

## vs sidebar-first (Cursor-class)

| Maxx here | Skip blind parity |
|-----------|-------------------|
| Session-first (plan, checkpoint, rewind) | Another composer tab |
| Spec-driven loop (`.cursor/specs/`) | Rules only in IDE settings |
| Intent + trust graph | Binary allow/confirm forever |
| Unified review (plan + diff + terminal + spec) | Diff buried in chat |
| Multi-lane agents | Single thread does everything |
| Native GPUI + OS sandbox | — |
| Local-first / BYOK models | Account-gated default model |

Details: `references/cursor-parity-vs-moat.md`

## Full AI surface map

When touching AI, know which layer you're in:

| Surface | Start here |
|---------|------------|
| Agent panel / chat | `crates/agent_ui/`, `acp_thread` |
| Tools + native loop | `crates/agent/`, `action_log` |
| ACP external agents | `crates/agent_servers/` |
| Prompts / system template | `crates/agent/src/templates/` |
| Models / providers | `language_model*`, `language_models/` |
| Skills | `agent_skills`, `.cursor/skills/` |
| MCP | `context_server*` |
| Permissions / sandbox | `agent_settings`, `agent/sandboxing.rs` |
| Inline / terminal AI | `inline_assistant`, `terminal_inline_assistant`, `buffer_codegen` |
| Edit prediction | `edit_prediction*` |
| Onboarding | `ai_onboarding` — replace zed.dev wall |

Full index: `references/ai-surface-map.md`

## Maxxing checklist (every AI feature)

Before calling it done:

- [ ] Closes loop: plan → act → **review** → checkpoint
- [ ] Intent-aware (Explore / Fix / Ship / Review) — see [04-sandbox-core](../../.cursor/specs/core/04-sandbox-core.md)
- [ ] Trust / permissions considered
- [ ] Inference **and** UI updated together
- [ ] Context cost sane (catalog vs body; compaction)
- [ ] Works local / BYOK — no required zed.dev
- [ ] Failure states visible and actionable
- [ ] Spec section linked ([13-ai-maxxing](../../.cursor/specs/agent/13-ai-maxxing.md) or [05-innovations](../../.cursor/specs/core/05-innovations.md))

## Ideating AI features

Load `engineering-partner` in **Ideate** mode. Ask:

1. What's the **session artifact** (plan entry, checkpoint, review batch)?
2. What's the **trust boundary**?
3. Does Cursor already do this — is our version **native / sandbox / spec** better?

Then narrow and implement with `agent-inference` + `ui-ux-gpui` + `rust-quality`.

## Innovation priorities

P0: zero-account, SDAL (spec-driven agent loop)
P1: intent switcher, checkpoints + unified review
P2: trust graph
P3: multi-lane, composer-first layout

See `.cursor/specs/core/05-innovations.md` for detail.

## Skill stack

| Layer | Skill |
|-------|-------|
| AI doctrine (this) | `cuecode-ai-maxxing` |
| Voice / ideate vs solve | `engineering-partner` |
| User paths | `product-builder` |
| Prompts / models / tools | `agent-inference` |
| Agent UI | `ui-ux-gpui` |
| Rust | `rust-quality` |

## References

- `references/ai-surface-map.md`
- `references/cursor-parity-vs-moat.md`
- `.cursor/specs/agent/13-ai-maxxing.md`
- `.cursor/specs/harness/README.md`
- `.cursor/specs/harness/cloud/README.md`
- `.cursor/specs/harness/local/01-agent-harness.md`
- `.cursor/specs/harness/14-agent-harness.md` (redirect stub)
