# Build phase 1.3-rev — Planning Hub v0 (P-H0) {#phase-1-3-rev}

> **Invoke:** `Build phase 1.3-rev` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `[x]` Done |
| **Last verified** | 2026-06-19 |
| **Duration** | 5–7 days |
| **Track** | 1 — Spec foundation · Planning Hub |
| **Roadmap** | [Phase 1](../07-implementation-roadmap#phase-1) · [16-planning-hub §P-H0](../../design/16-planning-hub.md#delivery-phases) |
| **QA script** | QA-P1 step 8 (revised), full QA-P1 |

## Deliverable {#phase-1-3-rev-deliverable}

**Planning Hub** shell replaces legacy 1.3 spec linker + spec browser: list, preview, Open, palette entry, read-only pin chip. Header spec **dropdown removed**.

## Depends / blocks {#phase-1-3-rev-deps}

| | Phase |
|---|-------|
| **Depends on** | 1.2 |
| **Blocks** | 1.4 |
| **Supersedes** | [1-3-spec-ui-stub](./1-3-spec-ui-stub.md) (do not implement legacy 1.3 tasks) |

## Out of scope {#phase-1-3-rev-out-of-scope}

- `project.yaml` / Build track / Implement (1.4)
- Organize pipeline (1.6)
- Multi-root `.cuecode/specs` (1.5)
- Pin modes summary/full/section (1.5)

---

## Tasks {#phase-1-3-rev-tasks}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

| ID | Task | File(s) | Done |
|----|------|---------|------|
| 1.3r.1 | Remove header spec dropdown (`SpecLinker` popover from toolbar) | `crates/agent_ui/src/spec_pin_chip.rs`, `agent_panel.rs` | `[x]` |
| 1.3r.2 | Read-only **pin chip** when `active_spec_path` set — `📎 {title}` | `crates/agent_ui/`, `conversation_view.rs` | `[x]` |
| 1.3r.3 | `PlanningHubModal` — refactor from `SpecBrowserModal`: list + markdown preview + Open | `crates/agent_ui/src/planning_hub.rs` | `[x]` |
| 1.3r.4 | Palette: **CueCode: Planning Hub** (replace `OpenSpecBrowser` action or alias) | `crates/cuecode_actions/`, `crates/workspace/` | `[x]` |
| 1.3r.5 | Hub tabs stub: Build track · All artifacts (single list from `cuecode_specs` index OK for v0) | `crates/agent_ui/src/planning_hub.rs` | `[x]` |
| 1.3r.6 | Empty state when no manifest: **Organize this project** + **Skip for now** (Organize disabled or stub until 1.6) | `crates/agent_ui/src/planning_hub.rs` | `[x]` |
| 1.3r.7 | Retarget `implement-spec` skill → Implement build phase (doc only until 1.4) | `.cursor/skills/implement-spec/SKILL.md` | `[x]` |
| 1.3r.8 | Delete or gate dead code paths for legacy browser/linker QA strings | `crates/agent_ui/`, onboarding copy | `[x]` |

---

## Implementation notes {#phase-1-3-rev-impl}

- Reuse `cuecode_specs::merged_spec_entries`, index load, open-in-editor from `spec_browser.rs`.
- Chip click opens Planning Hub focused on pinned artifact (optional v0).
- Do **not** ship header dropdown — spike code is reference only.
- Design: [16-planning-hub §hub-ui](../../design/16-planning-hub.md#hub-ui).

---

## Verify {#phase-1-3-rev-verify}

```bash
cd CueCode-IDE
cargo build -p agent_ui -p cuecode
# Manual: palette → CueCode: Planning Hub — list + preview
# Manual: no "Spec: none ▼" dropdown in agent header
# Manual: pin chip visible when active_spec_path set
```

---

## Exit criteria {#phase-1-3-rev-exit}

- [ ] [16-planning-hub E5](../../design/16-planning-hub.md#exit-criteria) — no header spec dropdown; one hub entry point
- [ ] Hub lists specs from index; Open opens editor
- [ ] Legacy 1.3 tasks **not** required (superseded)

---

## QA {#phase-1-3-rev-qa}

Manual steps before marking **Status** `[x]`:

1. Command palette → **CueCode: Planning Hub** — list + preview render
2. Agent header has **no** spec dropdown; pin chip when spec linked
3. Empty state shows Organize CTA (stub OK) + Skip
4. Run full QA-P1 script — revise step 8 for hub (not legacy browser)

**Script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts)

---

## PR checklist {#phase-1-3-rev-pr}

- [ ] PR links **Build phase 1.3-rev** and this file
- [ ] Tasks `1.3r.N` all `[x]` above
- [ ] Exit criteria checked
- [ ] Update [build-plans README](../README.md#phase-index) status column
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress)

---

## Deep specs (reference) {#phase-1-3-rev-specs}

| Topic | Doc |
|-------|-----|
| Planning Hub design | [16-planning-hub](../../design/16-planning-hub.md) |
| Supersedes 1.3 | [16 §supersedes-1-3](../../design/16-planning-hub.md#supersedes-1-3) |
| UI spec | [09-ui-ux-spec](../../design/09-ui-ux-spec.md) |

---

## Changelog {#phase-1-3-rev-changelog}

| Date | Change |
|------|--------|
| 2026-06-19 | Initial sub-phase — supersedes legacy 1.3 spec UI stub |
