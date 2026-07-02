# Build phase 1.2 — Agent integration (@spec + system prompt) {#phase-1-2}

> **Invoke:** `Build phase 1.2` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x] Done` |
| **Last verified** | 2026-06-20 (manual QA: Z.ai GLM, `@spec`, SDAL) |
| **Duration** | 5–7 days |
| **Track** | 1 — Spec foundation |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) |
| **QA script** | QA-P1 steps 3–4 |

## Deliverable {#phase-1-2-deliverable}

Agent system prompt includes compact spec index; `@spec` attaches full spec body; compaction preserves linked spec.

## Depends / blocks {#phase-1-2-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.1 |
| **Blocks** | 1.3 |

## Out of scope {#phase-1-2-out-of-scope}

- Spec linker header UI, command palette browser (1.3)
- Intent profiles (2.1)

---

## Tasks {#phase-1-2-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.2.1 | Spec index block in system prompt | `crates/agent/templates/` | `[x]` |
| 1.2.2 | `@spec` fuzzy completion | `crates/agent_ui/` composer | `[x]` |
| 1.2.3 | Session field `active_spec_path` | `crates/acp_thread/` | `[x]` |
| 1.2.4 | Compaction preserves index + linked spec | `crates/agent/` | `[x]` |
| 1.2.5 | `/list-specs` stub or tool | `crates/agent/` tools | `[x]` |
| 1.2.6 | Analytics: `cuecode.spec.*` events | `crates/telemetry/` | `[x]` |

---

## Implementation notes {#phase-1-2-impl}

- Inject compact `SpecIndex` markdown table into agent template (titles + paths + status, ~4KB budget).
- `@spec` uses `resolve_spec_query` — inject full `SpecDocument` body for the turn.
- `active_spec_path` on `AcpThread` session — survives compaction (EC-15).
- `/list-specs` tool returns index entries for agent self-query.

---

## Verify {#phase-1-2-verify}

```bash
cd CueCode-IDE
cargo build -p agent -p agent_ui
# Manual: @spec 05 in composer
```

---

## Exit criteria {#phase-1-2-exit}

- [x] `@spec 05` attaches body; agent answers "What is SDAL?" from spec
- [x] [07 §phase-1-acceptance](../07-implementation-roadmap.md#phase-1-acceptance) rows 1–5 pass

---

## QA {#phase-1-2-qa}

### Model prerequisites {#phase-1-2-qa-models}

Do **not** run this QA on local Ollama base models or small-context chat models.

| Requirement | Minimum |
|-------------|---------|
| **Provider** | Z.ai GLM via `openai_compatible` (`zai`) or equivalent cloud model |
| **Context** | ≥ 200k tokens (1M preferred for `@spec 05`) |
| **Tools** | Required (Write profile) |
| **Vision** | Optional — enable `capabilities.images` to QA screenshot paste |

Example `~/.config/cuecode/settings.json` block (API key via Agent Settings or `ZAI_API_KEY` env — never in settings):

```json
"language_models": {
  "openai_compatible": {
    "zai": {
      "api_url": "https://api.z.ai/api/coding/paas/v4",
      "available_models": [{
        "name": "glm-5.2",
        "max_tokens": 1000000,
        "capabilities": { "tools": true, "images": true }
      }]
    }
  }
}
```

### Manual steps {#phase-1-2-qa-steps}

Before marking **Status** `[x]`:

1. New agent thread — ask "List spec titles you know about" — expect ≥5 titles
2. Type `@spec 07` — pick roadmap spec — ask about Phase 2 — response references spec content
3. `@spec 05` — ask "What is SDAL?" — answer must come from spec body, not filename guess
4. Paste a screenshot — model describes visible content (vision sanity check)
5. Long session compaction — linked spec still present in context

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — QA-P1 steps 3–4

---

## PR checklist {#phase-1-2-pr}

- [x] PR title/body cites **Build phase 1.2** and this file (or doc-only closeout)
- [x] All task **Done** columns `[x]`
- [x] Exit criteria checked
- [x] Update status in [build-plans README](../README.md#phase-index)
- [x] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {#phase-1-2-specs}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
| System prompt | ../../agent/08-agent-tools-and-skills.md#system-prompt |
| list_specs tool | ../../agent/08-agent-tools-and-skills.md#tool-list-specs |
| Compaction | ../../parity/17-memory-and-context.md#compact |

---

## Changelog {#phase-1-2-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Implemented 1.2.1–1.2.6: spec index in system prompt, `@spec` completion, `active_spec_path`, compaction re-inject, `list_specs`/`link_spec` tools, `cuecode.spec.*` telemetry |
| 2026-06-20 | Initial sub-phase doc |
| 2026-06-20 | Manual QA complete (Z.ai GLM, `@spec` 05/07, SDAL, compaction) — phase closed |
