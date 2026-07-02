#!/usr/bin/env python3
"""Generate build-plans/phases/*.md from embedded phase definitions.

Source of truth for phase content: .cursor/specs/delivery/build-plans/00-master-build-plan.md
Template: .cursor/specs/delivery/build-plans/TEMPLATE-subphase.md

Run from anywhere:
    python3 script/generate-build-phase-docs.py
"""

from __future__ import annotations

import textwrap
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / ".cursor/specs/delivery/build-plans/phases"

SPEC_INDEX_TYPES = textwrap.dedent(
    """\
    Create `crates/cuecode_specs/` with these public types (from [06 §new-crates](../../core/06-system-design.md#new-crates)):

    ```rust
    use std::path::{Path, PathBuf};

    /// Compact index for system prompt injection.
    pub struct SpecIndex {
        pub worktree_id: project::WorktreeId,
        pub entries: Vec<SpecEntry>,
        pub updated_at: chrono::DateTime<chrono::Utc>,
    }

    pub struct SpecEntry {
        pub path: PathBuf,
        pub title: String,
        pub status: Option<SpecStatus>,
        pub tags: Vec<String>,
        pub summary: Option<String>,
        pub anchor_ids: Vec<String>,
    }

    pub enum SpecStatus {
        Draft,
        Active,
        Done,
        Deprecated,
    }

    pub fn load_spec_index(worktree_root: &Path, cx: &App) -> SpecIndex;
    pub fn resolve_spec_query(index: &SpecIndex, query: &str) -> Vec<SpecEntry>;
    ```

    - Scan only `.cursor/specs/**/*.md` under the worktree root.
    - YAML frontmatter parser: lenient (missing fields OK).
    - Filesystem watcher + debounce ≤2s → rebuild index in-memory (`Arc<SpecIndex>` per worktree).
    - Index payload budget: titles + paths + status (~4KB target in system prompt)."""
)

INTENT_TYPES = textwrap.dedent(
    """\
    Create `crates/cuecode_sandbox/` (or module under `agent` first, migrate later) with:

    ```rust
    pub enum Intent {
        Explore,
        Fix,
        Ship,
        Review,
        Orchestrate,  // feature-flag off until 3b/5
    }

    pub enum ExecutionContext {
        Active,
        Async,
        Hybrid,
    }

    pub struct IntentProfile {
        pub intent: Intent,
        pub tool_overlay: ToolPermissionOverlay,
        pub network: NetworkPolicy,
        pub fs_write: FsWritePolicy,
        pub sandbox_enabled: bool,
        pub default_execution: ExecutionContext,
        pub system_prompt_suffix: &'static str,
    }

    pub fn default_profiles() -> Vec<IntentProfile>;
    pub fn apply_intent(session: &mut SandboxSession, intent: Intent, cx: &mut App) -> anyhow::Result<()>;
    ```

    Wire intent → `tool_permissions` ([08 §permissions](../../agent/08-agent-tools-and-skills.md#permissions))
    and `agent::sandboxing` ([10 §terminal-sandbox](../../ops/10-infrastructure.md#terminal-sandbox)).
    Persist per workspace; overrides at `~/.config/cuecode/intent_profiles.json`."""
)


def md_table(rows: list[tuple[str, str, str, str]]) -> str:
    lines = [
        "| ID | Task | File(s) | Done |",
        "|----|------|---------|------|",
    ]
    for rid, task, files, done in rows:
        lines.append(f"| {rid} | {task} | {files} | `{done}` |")
    return "\n".join(lines)


def spec_links(links: list[tuple[str, str]]) -> str:
    if not links:
        return "| — | — |"
    return "\n".join(f"| {t} | {u} |" for t, u in links)


def render(p: dict) -> str:
    anchor = p["anchor"]
    status = p.get("status", "[ ] Not started")
    verified = p.get("verified", "—")
    tasks_md = md_table(p["tasks"])
    exit_lines = "\n".join(f"- [ ] {e}" for e in p["exit"])
    qa_lines = "\n".join(f"{i}. {s}" for i, s in enumerate(p["qa"], 1))
    oos = "\n".join(f"- {x}" for x in p.get("out_of_scope", []))
    impl = p.get("impl", "_See tasks table — implement exactly what is listed._")
    verify = p.get("verify", "cd CueCode-IDE\n# See QA section")
    changelog = p.get("changelog", "| 2026-06-20 | Initial sub-phase doc |")
    oos_block = oos if oos else "- _(none — see next sub-phase in [README](../README.md#phase-index))_"

    return f"""# Build phase {p["id"]} — {p["title"]} {{#{anchor}}}

> **Invoke:** `Build phase {p["id"]}` — open **this file only**; implement tasks in order; check boxes when verified.

| Field | Value |
|-------|-------|
| **Status** | `{status}` |
| **Last verified** | {verified} |
| **Duration** | {p["duration"]} |
| **Track** | {p["track"]} |
| **Roadmap** | [{p["roadmap_link"]}](../07-implementation-roadmap{p["roadmap_anchor"]}) |
| **QA script** | {p.get("qa_script", "—")} |

## Deliverable {{#{anchor}-deliverable}}

{p["deliverable"]}

## Depends / blocks {{#{anchor}-deps}}

| | Phase |
|---|-------|
| **Depends on** | {p["depends"]} |
| **Blocks** | {p["blocks"]} |

## Out of scope {{#{anchor}-out-of-scope}}

{oos_block}

---

## Tasks {{#{anchor}-tasks}}

Implement in order. Paths relative to `CueCode-IDE/` unless noted.

{tasks_md}

---

## Implementation notes {{#{anchor}-impl}}

{impl}

---

## Verify {{#{anchor}-verify}}

```bash
{verify}
```

---

## Exit criteria {{#{anchor}-exit}}

{exit_lines}

---

## QA {{#{anchor}-qa}}

Manual steps before marking **Status** `[x]`:

{qa_lines}

**Full script:** [07 §manual-qa-scripts](../07-implementation-roadmap#manual-qa-scripts) — {p.get("qa_script", "—")}

---

## PR checklist {{#{anchor}-pr}}

- [ ] PR title/body cites **Build phase {p["id"]}** and this file
- [ ] All task **Done** columns `[x]`
- [ ] Exit criteria checked
- [ ] Update status in [build-plans README](../README.md#phase-index)
- [ ] Sync [07 §progress](../07-implementation-roadmap#progress) if parent phase milestone complete

---

## Deep specs (reference) {{#{anchor}-specs}}

Optional reading for design rationale — not required to start tasks.

| Topic | Doc |
|-------|-----|
{spec_links(p.get("specs", []))}

---

## Changelog {{#{anchor}-changelog}}

| Date | Change |
|------|--------|
{changelog}
"""


PHASES: list[dict] = [
    {
        "file": "0-1-identity.md",
        "id": "0.1",
        "title": "Identity & paths",
        "anchor": "phase-0-1",
        "status": "[x] Done",
        "verified": "2026-06-20 — rebrand-check + qa-p0 (task 0.1.7 icons still open)",
        "duration": "2–3 days",
        "track": "0 — Rebrand & decouple",
        "roadmap_link": "Phase 0",
        "roadmap_anchor": "#phase-0",
        "qa_script": "QA-P0 partial, QA-RB-1 steps 1–4",
        "deliverable": "Binary name, `APP_NAME`, window title, and OS paths all say **CueCode**. Compile assert passes. No config collision with Zed on first launch.",
        "depends": "—",
        "blocks": "0.2",
        "out_of_scope": [
            "Cloud decouple, Ollama defaults, sign-in removal (0.2)",
            "CLI rebrand, bundle scripts, CI grep (0.3)",
            "Rename internal `crates/zed` crate — [03 §rename-depth L3](../../core/03-fork-and-rebrand.md#rename-depth)",
            "Rename `.zed/` project dirs — [12-open-questions](../../ops/12-open-questions.md)",
        ],
        "tasks": [
            ("0.1.1", "`APP_NAME = \"CueCode\"`", "`crates/paths/src/paths.rs`", "[x]"),
            ("0.1.2", "Binary `cuecode`", "`crates/cuecode/Cargo.toml` `[[bin]]`", "[x]"),
            ("0.1.3", "Compile-time assert passes", "`crates/cuecode/src/main.rs`", "[x]"),
            ("0.1.4", "Display names + app IDs", "`crates/release_channel/src/lib.rs`", "[x]"),
            ("0.1.5", "Window/title bar strings", "`crates/title_bar/`, `crates/cuecode/src/zed.rs`", "[x]"),
            ("0.1.6", "Windows metadata", "`crates/windows_resources/`", "[x]"),
            ("0.1.7", "App icons (minimal swap OK)", "`assets/icons/`, `crates/icons/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - `paths::APP_NAME` must be `"CueCode"` — config under `~/.config/cuecode/`.
            - Release channel display: Stable → `"CueCode"`, Dev → `"CueCode Dev"`, etc.
            - macOS bundle IDs: `dev.cuecode.CueCode`, etc. (see `release_channel::app_id`).
            - Title bar must not show bare product name **Zed**.
            - ERR-002 compile assert in `main.rs` must pass on every build."""
        ),
        "verify": "cd CueCode-IDE\n./script/rebrand-check.sh\n./script/qa-p0.sh",
        "exit": [
            "`cargo run --bin cuecode` launches",
            "Window title = **CueCode**",
            "First launch writes only under `cuecode` paths — [03 §G1–G5](../../core/03-fork-and-rebrand.md#goal-acceptance)",
            "[07 §phase-0-acceptance](../07-implementation-roadmap.md#phase-0-acceptance) rows 1, 3, 6 pass",
        ],
        "qa": [
            "Launch app — title bar reads CueCode (light + dark)",
            "Inspect `~/.config/` — only `cuecode/` created, not `zed/`",
            "Run `./script/rebrand-check.sh` — Tier 1 identity gates green",
            "Run `./script/qa-p0.sh` steps 1–2, 5 — pass",
        ],
        "specs": [
            ("Rebrand tiers", "../../core/03-fork-and-rebrand.md"),
            ("Paths architecture", "../../core/02-current-architecture.md#paths"),
            ("UI title bar", "../../design/09-ui-ux-spec.md"),
        ],
        "changelog": "| 2026-06-20 | Generated; 0.1.7 icons deferred |",
    },
    {
        "file": "0-2-cloud-decouple.md",
        "id": "0.2",
        "title": "Cloud decouple & defaults",
        "anchor": "phase-0-2",
        "status": "[x] Done",
        "verified": "2026-06-20",
        "duration": "4–7 days",
        "track": "0 — Rebrand & decouple",
        "roadmap_link": "Phase 0",
        "roadmap_anchor": "#phase-0",
        "qa_script": "QA-P0, QA-RB-1 steps 5–7, QA-RB-2",
        "deliverable": "Agent works on first prompt with local/BYOK model. No sign-in wall, no Zed Pro upsell, no zed.dev default traffic.",
        "depends": "0.1",
        "blocks": "0.3",
        "out_of_scope": [
            "CLI rebrand, bundle scripts, CI grep (0.3)",
            "Spec index, intent profiles (Track 1+)",
        ],
        "tasks": [
            ("0.2.1", "Default model → Ollama/BYOK", "`assets/settings/default.json`", "[x]"),
            ("0.2.2", "Replace CueCode onboarding", "`crates/ai_onboarding/`, `crates/onboarding/`", "[x]"),
            ("0.2.3", "Remove Zed Pro / trial upsell", "`crates/agent_ui/src/end_trial_upsell.rs`, `edit_prediction_ui/`, `thread_view.rs`", "[x]"),
            ("0.2.4", "Hide sign-in + account UI", "`crates/title_bar/`, `crates/cuecode/src/zed.rs`", "[x]"),
            ("0.2.5", "Hide collab/channels menus", "`crates/collab_ui/`, `crates/app_menus/src/app_menus.rs`", "[x]"),
            ("0.2.6", "Stub/replace billing URLs", "`crates/client/src/zed_urls.rs`, `crates/client/src/client.rs`", "[x]"),
            ("0.2.7", "Disable auto-update to zed.dev", "`crates/auto_update/`", "[x]"),
            ("0.2.8", "Telemetry off by default", "`crates/telemetry/`, `assets/settings/default.json`", "[x]"),
            ("0.2.9", "About dialog: CueCode + GPL", "About view in `crates/zed/`", "[x]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Default agent model must not require zed.dev auth — Ollama or BYOK path.
            - Onboarding screens: CueCode branding, no "Sign in to Zed".
            - Grep target after changes:

            ```bash
            rg -n "zed\\.dev|Sign in to Zed|Zed Pro" crates --glob '!target/**'
            ```

            Expect **zero hits** in user-facing agent/onboarding paths."""
        ),
        "verify": "cd CueCode-IDE\nrg -n \"zed\\.dev|Sign in to Zed|Zed Pro\" crates --glob '!target/**'\n./script/qa-p0.sh",
        "exit": [
            "Fresh install: agent prompt streams without auth — [07 §phase-0-acceptance](../07-implementation-roadmap.md#phase-0-acceptance) rows 2, 4, 7",
            "No new writes to `~/.config/zed/` — [03 §ERR-001](../../core/03-fork-and-rebrand.md#err-001)",
            "Idle app: no zed.dev traffic — [03 §ERR-006](../../core/03-fork-and-rebrand.md#err-006)",
            "[03 §first-launch-flow](../../core/03-fork-and-rebrand.md#first-launch-flow) reaches **SUCCESS: Phase 0 exit**",
        ],
        "qa": [
            "Delete `~/.config/cuecode/` on test machine — fresh launch",
            "Skip onboarding — open agent panel — no sign-in wall",
            "Configure Ollama — send prompt — verify stream",
            "Grep agent/onboarding paths for `zed.dev` — zero hits",
            "Run `./script/qa-p0.sh` steps 3–4, 7 — pass",
        ],
        "specs": [
            ("Decouple defaults", "../../core/03-fork-and-rebrand.md#decouple-defaults"),
            ("Onboarding mockup", "../../core/03-fork-and-rebrand.md#mockup-onboarding"),
            ("Models infra", "../../ops/10-infrastructure.md#models"),
        ],
        "changelog": "| 2026-06-20 | Generated from master build plan |",
    },
    {
        "file": "0-3-packaging-qa.md",
        "id": "0.3",
        "title": "Packaging, CLI & rebrand QA",
        "anchor": "phase-0-3",
        "status": "[x] Done mostly",
        "verified": "2026-06-20 — full QA-P0 pending final bundle pass",
        "duration": "2–4 days",
        "track": "0 — Rebrand & decouple",
        "roadmap_link": "Phase 0",
        "roadmap_anchor": "#phase-0",
        "qa_script": "QA-P0 full, QA-RB-3–5",
        "deliverable": "Shippable `cuecode` CLI, bundle scripts, CI rebrand gate, and full Phase 0 QA pass. Safe to start Track 1.",
        "depends": "0.2",
        "blocks": "1.1",
        "out_of_scope": [
            "`cuecode_specs` crate (1.1)",
            "Docs site, release DMG (6.1)",
        ],
        "tasks": [
            ("0.3.1", "CLI rebrand (`cuecode --help`)", "`crates/cuecode/`, CLI entrypoints", "[x]"),
            ("0.3.2", "Bundle scripts (macOS `.app`, Linux `.desktop`)", "`script/bundle-*.sh`, packaging configs", "[x]"),
            ("0.3.3", "Flatpak/snap/Windows installer strings", "installer metadata, `crates/windows_resources/`", "[~]"),
            ("0.3.4", "CI grep job for rebrand regression", "`.github/workflows/` or CI scripts", "[x]"),
            ("0.3.5", "Link CONTRIBUTING/README to specs", "`CONTRIBUTING.md`, `README.md`", "[x]"),
            ("0.3.6", "Optional: strip/replace `docs/` Zed copy", "`docs/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - `cuecode --help` must show CueCode branding, not Zed.
            - Bundle IDs and `.desktop` `Name=` fields must match `APP_NAME`.
            - CI job runs `rebrand-check.sh` or equivalent grep gates on every PR.
            - CONTRIBUTING links to `.cursor/specs/00-README.md`."""
        ),
        "verify": "cd CueCode-IDE\n./script/rebrand-check.sh\n./script/qa-p0.sh\ncuecode --help | head -20",
        "exit": [
            "All [07 §phase-0-exit](../07-implementation-roadmap.md#phase-0-exit) checkboxes",
            "All [07 §phase-0-acceptance](../07-implementation-roadmap.md#phase-0-acceptance) rows pass",
            "**Phase 0 complete** — safe to start 1.1",
        ],
        "qa": [
            "Run full `./script/qa-p0.sh` — all steps green",
            "Run `cuecode --help` — CueCode strings only",
            "Spot-check macOS `.app` bundle name in Finder",
            "Confirm CI rebrand grep job exists and passes on main",
        ],
        "specs": [
            ("QA-RB-3–5", "../../core/03-fork-and-rebrand.md#qa-rb-3"),
            ("Build release", "../../ops/10-infrastructure.md#build-release"),
            ("Assets packaging", "../../core/03-fork-and-rebrand.md#assets-packaging"),
        ],
        "changelog": "| 2026-06-20 | Generated; 0.3.3/0.3.6 partial |",
    },
    {
        "file": "1-1-cuecode-specs.md",
        "id": "1.1",
        "title": "`cuecode_specs` crate",
        "anchor": "phase-1-1",
        "status": "[ ] Not started",
        "duration": "5–7 days",
        "track": "1 — Spec foundation",
        "roadmap_link": "Phase 1",
        "roadmap_anchor": "#phase-1",
        "qa_script": "QA-P1 steps 1–2, 5–7",
        "deliverable": "`.cursor/specs/` is indexed at runtime: scan, parse frontmatter, watch for changes, expose `SpecIndex` for agent and UI.",
        "depends": "0.3",
        "blocks": "1.2",
        "out_of_scope": [
            "System prompt injection, `@spec` composer (1.2)",
            "Spec browser UI stub (1.3)",
        ],
        "tasks": [
            ("1.1.1", "Create `crates/cuecode_specs/`", "`crates/cuecode_specs/Cargo.toml`, `src/lib.rs`", "[ ]"),
            ("1.1.2", "`SpecIndex` scan `.cursor/specs/**/*.md`", "`crates/cuecode_specs/src/index.rs`", "[ ]"),
            ("1.1.3", "YAML frontmatter parser (lenient)", "`crates/cuecode_specs/src/parse.rs`", "[ ]"),
            ("1.1.4", "Filesystem watcher + debounce ≤2s", "`crates/cuecode_specs/src/watch.rs`", "[ ]"),
            ("1.1.5", "`SpecEntry { path, title, summary, anchor_ids }`", "`crates/cuecode_specs/src/index.rs`", "[ ]"),
            ("1.1.6", "Unit tests with fixture tree", "`crates/cuecode_specs/tests/`", "[ ]"),
        ],
        "impl": SPEC_INDEX_TYPES,
        "verify": "cd CueCode-IDE\ncargo test -p cuecode_specs\ncargo build -p cuecode_specs",
        "exit": [
            "Index loads ≥3 specs from workspace",
            "Watcher refresh on new file without restart",
            "`cargo test -p cuecode_specs` green",
        ],
        "qa": [
            "Open workspace with `.cursor/specs/` — index loads ≥3 entries",
            "Create `.cursor/specs/qa-scratch.md` — index updates within 2s",
            "Delete scratch file — entry removed on next index read",
            "Run `cargo test -p cuecode_specs` — all green",
        ],
        "specs": [
            ("New crates", "../../core/06-system-design.md#new-crates"),
            ("Spec integration", "../../core/04-sandbox-core.md#spec-integration"),
            ("Conventions", "../../00-README.md#conventions"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "1-2-agent-spec-integration.md",
        "id": "1.2",
        "title": "Agent integration (@spec + system prompt)",
        "anchor": "phase-1-2",
        "status": "[ ] Not started",
        "duration": "5–7 days",
        "track": "1 — Spec foundation",
        "roadmap_link": "Phase 1",
        "roadmap_anchor": "#phase-1",
        "qa_script": "QA-P1 steps 3–4",
        "deliverable": "Agent system prompt includes compact spec index; `@spec` attaches full spec body; compaction preserves linked spec.",
        "depends": "1.1",
        "blocks": "1.3",
        "out_of_scope": [
            "Spec linker header UI, command palette browser (1.3)",
            "Intent profiles (2.1)",
        ],
        "tasks": [
            ("1.2.1", "Spec index block in system prompt", "`crates/agent/templates/`", "[ ]"),
            ("1.2.2", "`@spec` fuzzy completion", "`crates/agent_ui/` composer", "[ ]"),
            ("1.2.3", "Session field `active_spec_path`", "`crates/acp_thread/`", "[ ]"),
            ("1.2.4", "Compaction preserves index + linked spec", "`crates/agent/`", "[ ]"),
            ("1.2.5", "`/list-specs` stub or tool", "`crates/agent/` tools", "[ ]"),
            ("1.2.6", "Analytics: `cuecode.spec.*` events", "`crates/telemetry/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Inject compact `SpecIndex` markdown table into agent template (titles + paths + status, ~4KB budget).
            - `@spec` uses `resolve_spec_query` — inject full `SpecDocument` body for the turn.
            - `active_spec_path` on `AcpThread` session — survives compaction (EC-15).
            - `/list-specs` tool returns index entries for agent self-query."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p agent -p agent_ui\n# Manual: @spec 05 in composer",
        "exit": [
            "`@spec 05` attaches body; agent answers \"What is SDAL?\" from spec",
            "[07 §phase-1-acceptance](../07-implementation-roadmap.md#phase-1-acceptance) rows 1–5 pass",
        ],
        "qa": [
            "New agent thread — ask \"List spec titles you know about\" — expect ≥5 titles",
            "Type `@spec 07` — pick roadmap spec — ask about Phase 2 — response references spec content",
            "Long session compaction — linked spec still present in context",
        ],
        "specs": [
            ("System prompt", "../../agent/08-agent-tools-and-skills.md#system-prompt"),
            ("list_specs tool", "../../agent/08-agent-tools-and-skills.md#tool-list-specs"),
            ("Compaction", "../../parity/17-memory-and-context.md#compact"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "1-3-spec-ui-stub.md",
        "id": "1.3",
        "title": "Spec UI stub + skills meta",
        "anchor": "phase-1-3",
        "status": "[ ] Not started",
        "duration": "2–3 days",
        "track": "1 — Spec foundation",
        "roadmap_link": "Phase 1",
        "roadmap_anchor": "#phase-1",
        "qa_script": "QA-P1 step 8, full QA-P1",
        "deliverable": "Spec linker in agent header, command palette spec browser, `implement-spec` skill stub. **Phase 1 complete.**",
        "depends": "1.2",
        "blocks": "2.1",
        "out_of_scope": [
            "Intent switcher (2.2)",
            "Plan ↔ spec checkbox sync (3.2)",
        ],
        "tasks": [
            ("1.3.1", "Spec linker in agent header (\"Spec: none ▼\")", "`crates/agent_ui/`", "[ ]"),
            ("1.3.2", "Command palette \"CueCode: Open Spec Browser\"", "`crates/workspace/`, `crates/agent_ui/`", "[ ]"),
            ("1.3.3", "`implement-spec` skill stub", "`.cursor/skills/` or project skills template", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Header dropdown lists `SpecIndex` entries; selecting sets `active_spec_path`.
            - Spec browser: read-only list from index; open spec in editor on click.
            - `implement-spec` skill stub documents how agents should load and follow specs."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p agent_ui\n# Manual: Command palette → CueCode: Open Spec Browser",
        "exit": [
            "[07 §phase-1-exit](../07-implementation-roadmap.md#phase-1-exit) all rows",
            "**Phase 1 complete**",
        ],
        "qa": [
            "Command palette → \"CueCode: Open Spec Browser\" — list renders",
            "Select spec from header linker — `active_spec_path` updates",
            "Run full QA-P1 script — all steps green",
        ],
        "specs": [
            ("UI spec", "../../design/09-ui-ux-spec.md"),
            ("Project agent skills", "../../00-README.md#project-agent-skills"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "2-1-intent-core.md",
        "id": "2.1",
        "title": "`cuecode_sandbox` intent core",
        "anchor": "phase-2-1",
        "status": "[ ] Not started",
        "duration": "7–10 days",
        "track": "2 — Intent profiles",
        "roadmap_link": "Phase 2",
        "roadmap_anchor": "#phase-2",
        "qa_script": "QA-P2 steps 1–2, 5–6",
        "deliverable": "Explore / Fix / Ship / Review intents reconfigure tool permissions, sandbox policy, and system prompt suffix. Persisted per workspace.",
        "depends": "1.3",
        "blocks": "2.2, 3.1, 3b.1, 4.1, C.0",
        "out_of_scope": [
            "Intent switcher UI, sandbox badge (2.2)",
            "Checkpoints, review panel (Track 3)",
            "Background spawn (3b.1)",
        ],
        "tasks": [
            ("2.1.1", "Create `cuecode_sandbox` (or module → migrate)", "`crates/cuecode_sandbox/`", "[ ]"),
            ("2.1.2", "`Intent` enum + `IntentProfile` struct", "`crates/cuecode_sandbox/src/intent.rs`", "[ ]"),
            ("2.1.3", "Wire intent → `tool_permissions`", "`crates/agent/`, `crates/cuecode_sandbox/src/policy.rs`", "[ ]"),
            ("2.1.4", "Wire intent → `agent::sandboxing`", "`crates/agent/src/sandboxing.rs`", "[ ]"),
            ("2.1.5", "Intent block in agent templates", "`crates/agent/templates/`", "[ ]"),
            ("2.1.6", "Persist intent per workspace", "`crates/cuecode_sandbox/src/persistence.rs`", "[ ]"),
            ("2.1.7", "`~/.config/cuecode/intent_profiles.json` overrides", "`crates/cuecode_sandbox/src/persistence.rs`", "[ ]"),
            ("2.1.8", "Feature-flag Orchestrate (off until 3b/5)", "`crates/cuecode_sandbox/src/intent.rs`", "[ ]"),
        ],
        "impl": INTENT_TYPES,
        "verify": "cd CueCode-IDE\ncargo test -p cuecode_sandbox\ncargo build -p cuecode_sandbox -p agent",
        "exit": [
            "Explore denies `edit_file` at permission layer — [07 §phase-2-acceptance](../07-implementation-roadmap.md#phase-2-acceptance) row 1",
            "Fix enables macOS sandbox when flag on — row 3",
        ],
        "qa": [
            "Set Explore via API/settings — agent `edit_file` denied at permission layer",
            "Switch Fix — edit path available (confirm or execute)",
            "Close workspace — reopen — intent persisted",
        ],
        "specs": [
            ("Intent profiles", "../../core/04-sandbox-core.md#intent-profiles"),
            ("Permissions", "../../agent/08-agent-tools-and-skills.md#permissions"),
            ("Terminal sandbox", "../../ops/10-infrastructure.md#terminal-sandbox"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "2-2-intent-ui.md",
        "id": "2.2",
        "title": "Intent UI + sandbox badge",
        "anchor": "phase-2-2",
        "status": "[ ] Not started",
        "duration": "4–5 days",
        "track": "2 — Intent profiles",
        "roadmap_link": "Phase 2",
        "roadmap_anchor": "#phase-2",
        "qa_script": "QA-P2 full",
        "deliverable": "Intent switcher in agent header, sandbox badge popover, keyboard cycle, settings stub. **Phase 2 complete** — unlocks 3.x, 3b.x, 4.x, C.0.",
        "depends": "2.1",
        "blocks": "3.1, 3b.1, 4.1, C.0",
        "out_of_scope": [
            "Checkpoint store (3.1)",
            "Multi-lane tabs (5.1)",
        ],
        "tasks": [
            ("2.2.1", "Intent switcher in agent header", "`crates/agent_ui/`", "[ ]"),
            ("2.2.2", "Sandbox badge + popover", "`crates/agent_ui/`", "[ ]"),
            ("2.2.3", "Hide write UI in Explore", "`crates/agent_ui/`", "[ ]"),
            ("2.2.4", "`cmd-shift-i` cycle intent", "`crates/workspace/src/keymap.rs`, `crates/agent_ui/`", "[ ]"),
            ("2.2.5", "Settings UI stub: Agent → Intent Profiles", "`crates/settings_ui/` or settings panel", "[ ]"),
            ("2.2.6", "Analytics: `cuecode.intent.*`, `cuecode.sandbox.*`", "`crates/telemetry/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Intent switcher: segmented control or dropdown in agent panel header.
            - Sandbox badge shows active policy; popover lists network + FS write rules.
            - Explore: hide \"Apply patch\" and write affordances in composer toolbar.
            - `cmd-shift-i` cycles Explore → Fix → Ship → Review with accessibility announcement."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p agent_ui\n# Manual: cmd-shift-i cycle, sandbox badge click",
        "exit": [
            "[07 §phase-2-exit](../07-implementation-roadmap.md#phase-2-exit) + all [phase-2-acceptance](../07-implementation-roadmap.md#phase-2-acceptance) rows",
            "**Phase 2 complete** — unlocks 3.x, 3b.x, 4.x, C.0",
        ],
        "qa": [
            "Set Explore — ask agent to edit `README.md` — verify deny in tool card",
            "Switch Fix — repeat — verify confirm or execute path",
            "Click sandbox badge — popover shows network + FS policy",
            "Press `cmd-shift-i` four times — cycle all intents",
            "Explore — verify no \"Apply patch\" in composer toolbar",
            "Run full QA-P2 — all pass steps green",
        ],
        "specs": [
            ("Intent UI", "../../design/09-ui-ux-spec.md#intent-ui"),
            ("Intent switcher innovation", "../../core/05-innovations.md#intent-switcher"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "3-1-checkpoint-store.md",
        "id": "3.1",
        "title": "Checkpoint store",
        "anchor": "phase-3-1",
        "status": "[ ] Not started",
        "duration": "7–10 days",
        "track": "3 — Review & checkpoints",
        "roadmap_link": "Phase 3",
        "roadmap_anchor": "#phase-3",
        "qa_script": "QA-P3 steps 6–7",
        "deliverable": "Session-scoped checkpoints: snapshot action_log, plan, spec path, terminal IDs; create on turn complete; restore/rewind.",
        "depends": "2.2",
        "blocks": "3.2",
        "out_of_scope": [
            "Unified review panel tabs (3.2)",
            "Git stash integration default-on (flag, default off)",
        ],
        "tasks": [
            ("3.1.1", "`CheckpointStore` in `cuecode_sandbox`", "`crates/cuecode_sandbox/src/checkpoint.rs`", "[ ]"),
            ("3.1.2", "Snapshot: action_log, plan, spec path, terminal IDs", "`crates/cuecode_sandbox/src/checkpoint.rs`", "[ ]"),
            ("3.1.3", "Create on turn complete (configurable)", "`crates/agent_ui/`, `crates/cuecode_sandbox/`", "[ ]"),
            ("3.1.4", "Restore / rewind", "`crates/cuecode_sandbox/src/checkpoint.rs`", "[ ]"),
            ("3.1.5", "Optional git stash integration (flag, default off)", "`crates/cuecode_sandbox/src/checkpoint.rs`", "[ ]"),
            ("3.1.6", "Checkpoint timeline UI (minimal)", "`crates/agent_ui/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            ```rust
            pub fn create_checkpoint(session: &SandboxSession, cx: &App) -> anyhow::Result<Checkpoint>;
            pub fn list_checkpoints(session_id: &acp_thread::ThreadId) -> Vec<CheckpointMeta>;
            pub fn restore_checkpoint(id: CheckpointId, opts: RestoreOptions, cx: &App) -> anyhow::Result<()>;
            ```

            - Store under `~/.local/share/cuecode/checkpoints/<session_id>/cp_*.json`.
            - Snapshot includes: pending + applied edits from `action_log`, plan, linked spec path, terminal session IDs."""
        ),
        "verify": "cd CueCode-IDE\ncargo test -p cuecode_sandbox -- checkpoint",
        "exit": [
            "Rewind entire turn, not just last hunk — [07 §phase-3-acceptance](../07-implementation-roadmap.md#phase-3-acceptance) row 4",
        ],
        "qa": [
            "Fix intent — agent changes two files — turn completes",
            "Checkpoint timeline shows new entry",
            "Restore previous checkpoint — confirm dialog — files rewind",
            "Reject all in review — files restored to pre-turn state",
        ],
        "specs": [
            ("Checkpoint stack", "../../core/05-innovations.md#checkpoint-stack"),
            ("Review lifecycle", "../../core/04-sandbox-core.md#review"),
            ("Checkpoint flow", "../../core/06-system-design.md#checkpoint-flow"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "3-2-review-panel.md",
        "id": "3.2",
        "title": "Unified review panel",
        "anchor": "phase-3-2",
        "status": "[ ] Not started",
        "duration": "7–10 days",
        "track": "3 — Review & checkpoints",
        "roadmap_link": "Phase 3",
        "roadmap_anchor": "#phase-3",
        "qa_script": "QA-P3 full",
        "deliverable": "Review panel with Plan | Diffs | Terminal | Spec tabs; accept/reject partial; triggers on turn complete and `cmd-shift-r`. **Alpha milestone** reachable.",
        "depends": "3.1",
        "blocks": "5.1",
        "out_of_scope": [
            "Notification rail, VERDICT (3b.2)",
            "Multi-lane (5.1)",
        ],
        "tasks": [
            ("3.2.1", "`review_panel.rs` — Plan | Diffs | Terminal | Spec tabs", "`crates/agent_ui/src/review_panel.rs`", "[ ]"),
            ("3.2.2", "Accept all / reject all / partial", "`crates/agent_ui/src/review_panel.rs`", "[ ]"),
            ("3.2.3", "Triggers: turn complete, `cmd-shift-r`, notification", "`crates/agent_ui/`", "[ ]"),
            ("3.2.4", "Plan ↔ spec checkbox mapping (v1, confirm)", "`crates/cuecode_specs/src/sync.rs`", "[ ]"),
            ("3.2.5", "Analytics: `cuecode.review.*`, `cuecode.checkpoint.*`", "`crates/telemetry/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Four tabs: Plan (ACP plan entries), Diffs (action_log hunks), Terminal (commands run), Spec (linked spec excerpt).
            - Partial accept: per-file or per-hunk where supported.
            - `cmd-shift-r` opens review when pending edits exist.
            - Plan ↔ spec sync v1: propose checkbox toggles; user must confirm before apply (Q6)."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p agent_ui\n# Manual: QA-P3 full script",
        "exit": [
            "[07 §phase-3-exit](../07-implementation-roadmap.md#phase-3-exit) + all acceptance rows",
            "**Alpha milestone** reachable — [07 §alpha-milestone](../07-implementation-roadmap.md#alpha-milestone)",
        ],
        "qa": [
            "Fix intent — agent changes two files — review panel opens or badge shows count",
            "Plan tab — verify entries; Terminal tab — commands listed",
            "Accept one file, reject one — partial state correct",
            "Reject all — files restored",
            "`cmd-shift-r` opens review when pending edits exist",
            "Run full QA-P3 — pass steps 2, 4, 5, 7 green",
        ],
        "specs": [
            ("Review panel UI", "../../design/09-ui-ux-spec.md#review-panel"),
            ("Flow G security review", "../../parity/16-end-to-end-flows.md#flow-g-security-review"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "3b-1-background-spawn.md",
        "id": "3b.1",
        "title": "`run_in_background` + builtin agents",
        "anchor": "phase-3b-1",
        "status": "[ ] Not started",
        "duration": "7–10 days",
        "track": "3b — Active / Async harness (local)",
        "roadmap_link": "Phase 3b",
        "roadmap_anchor": "#phase-3b",
        "qa_script": "QA-P3b step 5",
        "deliverable": "Background agent spawn with `ExecutionContext` Active/Async/Hybrid; builtin explore + verification agents; sidechain JSONL on disk.",
        "depends": "2.2",
        "blocks": "3b.2, C.2",
        "out_of_scope": [
            "Notification rail UI (3b.2)",
            "VERDICT parse + Ship block (3b.2)",
            "Cloud CHP spawn (C.2)",
        ],
        "tasks": [
            ("3b.1.1", "`ExecutionContext` Active/Async/Hybrid", "`crates/cuecode_sandbox/src/execution.rs`", "[ ]"),
            ("3b.1.2", "`spawn_agent` + `run_in_background: bool`", "`crates/agent/` tools, `crates/cuecode_sandbox/`", "[ ]"),
            ("3b.1.3", "Builtin defs: explore, verification (tool walls)", "`crates/cuecode_sandbox/src/builtin_agents.rs`", "[ ]"),
            ("3b.1.4", "Sidechain JSONL under session dir", "`crates/cuecode_sandbox/src/execution.rs`", "[ ]"),
            ("3b.1.5", "Resume via `session_id`", "`crates/acp_thread/`, `crates/agent_ui/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - `ExecutionContext`: Active (blocks composer optional), Async (background), Hybrid (handoff artifacts).
            - Builtin explore agent: read-only tool wall — no `edit_file`.
            - Builtin verification agent: test/run tools only.
            - Sidechain transcripts: JSONL under session dir per [local §B.5](../../harness/local/01-agent-harness.md#b-5-async-artifacts-on-disk).
            - Resume child thread by `session_id` without duplicate."""
        ),
        "verify": "cd CueCode-IDE\ncargo test -p cuecode_sandbox -- execution\ncargo build -p agent -p cuecode_sandbox",
        "exit": [
            "Explore subagent cannot call `edit_file` — [07 §phase-3b-acceptance](../07-implementation-roadmap.md#phase-3b-acceptance) row 1",
        ],
        "qa": [
            "Spawn background explore — completes without blocking main composer (per setting)",
            "Attempt explore subagent `edit_file` — blocked",
            "Resume via `session_id` — no duplicate thread",
        ],
        "specs": [
            ("Three contexts", "../../harness/local/01-agent-harness.md#three-contexts"),
            ("Part B async", "../../harness/local/01-agent-harness.md#part-b-async"),
            ("Spawn tool", "../../agent/08-agent-tools-and-skills.md"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "3b-2-notification-verdict.md",
        "id": "3b.2",
        "title": "Notification rail + VERDICT",
        "anchor": "phase-3b-2",
        "status": "[ ] Not started",
        "duration": "7–10 days",
        "track": "3b — Active / Async harness (local)",
        "roadmap_link": "Phase 3b",
        "roadmap_anchor": "#phase-3b",
        "qa_script": "QA-P3b full",
        "deliverable": "Notification rail for async completions; VERDICT parse; FAIL blocks Ship complete; stop hooks v1 stub.",
        "depends": "3b.1 (3.2 recommended)",
        "blocks": "5.1, C.3",
        "out_of_scope": [
            "Cloud VERDICT service (C.3)",
            "Multi-lane coordinator (5.1)",
        ],
        "tasks": [
            ("3b.2.1", "Notification rail UI", "`crates/agent_ui/`", "[ ]"),
            ("3b.2.2", "Payload schema (XML/JSON)", "`crates/cuecode_sandbox/src/execution.rs`", "[ ]"),
            ("3b.2.3", "Verification agent + VERDICT parse", "`crates/cuecode_sandbox/src/builtin_agents.rs`", "[ ]"),
            ("3b.2.4", "FAIL blocks Ship complete (override audit)", "`crates/agent_ui/`, `crates/cuecode_sandbox/`", "[ ]"),
            ("3b.2.5", "Stop hooks v1 (memory extract stub)", "`crates/agent/`", "[ ]"),
            ("3b.2.6", "Analytics: harness events", "`crates/telemetry/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Notification rail: task pills + completion cards; click opens review Terminal tab on FAIL.
            - VERDICT format: structured PASS/FAIL/PARTIAL — not prose inference.
            - FAIL unacknowledged blocks session complete (EC-16); override writes audit entry.
            - Stop hooks v1: stub memory extract on session end."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p agent_ui -p cuecode_sandbox\n# Manual: QA-P3b full",
        "exit": [
            "[07 §phase-3b-exit](../07-implementation-roadmap.md#phase-3b-exit) + acceptance rows",
            "[15 §P2](../../parity/15-competitive-parity.md#phases) deliverables checked",
        ],
        "qa": [
            "Spawn background verification on small test crate",
            "Wait for VERDICT — notification appears in rail",
            "Click Review on FAIL — lands in review Terminal tab",
            "FAIL blocks Ship complete until override",
            "Run full QA-P3b — steps 3, 5 green",
        ],
        "specs": [
            ("Notification rail", "../../harness/local/01-agent-harness.md#notification-rail-ui"),
            ("Verification prompt", "../../harness/local/01-agent-harness.md#verification-prompt-outline"),
            ("Stop hooks", "../../parity/17-memory-and-context.md#stop-hooks"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "4-1-trust-store.md",
        "id": "4.1",
        "title": "Trust store + UI",
        "anchor": "phase-4-1",
        "status": "[ ] Not started",
        "duration": "7–10 days",
        "track": "4 — Trust graph",
        "roadmap_link": "Phase 4",
        "roadmap_anchor": "#phase-4",
        "qa_script": "QA-P4",
        "deliverable": "Per-repo trust JSON; promotion thresholds; Settings trust UI; auto-approve proven-safe actions. **Phase 4 complete.**",
        "depends": "2.2",
        "blocks": "C.4",
        "out_of_scope": [
            "Cloud BYOK trust routing (C.4)",
            "Multi-lane (5.1)",
        ],
        "tasks": [
            ("4.1.1", "Trust JSON per repo hash", "`crates/cuecode_sandbox/src/trust.rs`", "[ ]"),
            ("4.1.2", "Promotion thresholds + hard deny", "`crates/cuecode_sandbox/src/trust.rs`", "[ ]"),
            ("4.1.3", "Settings → Agent → Trust UI", "`crates/settings_ui/` or settings panel", "[ ]"),
            ("4.1.4", "Tool card labels: auto-approved vs you approved", "`crates/agent_ui/`", "[ ]"),
            ("4.1.5", "Unit tests in `cuecode_sandbox`", "`crates/cuecode_sandbox/tests/`", "[ ]"),
            ("4.1.6", "Analytics: `cuecode.trust.*`", "`crates/telemetry/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            ```rust
            pub fn load_trust_store(repo_hash: &str) -> anyhow::Result<TrustStore>;
            pub fn record_trust_evidence(store: &mut TrustStore, evidence: TrustEvidence);
            pub fn evaluate_trust(request: &ToolRequest, store: &TrustStore) -> TrustDecision;
            ```

            - Store at `~/.config/cuecode/trust/<repo_hash>.json`.
            - Hard deny: `.env` writes, destructive rm, etc. — never auto-promote.
            - Tool cards show \"auto-approved\" vs \"you approved\" labels."""
        ),
        "verify": "cd CueCode-IDE\ncargo test -p cuecode_sandbox -- trust",
        "exit": [
            "[07 §phase-4-exit](../07-implementation-roadmap.md#phase-4-exit) all rows",
            "**Phase 4 complete**",
        ],
        "qa": [
            "Deny then allow same `cargo test` five times — sixth run auto-approved",
            "Settings → Trust — rule visible with evidence",
            "Revoke rule — next run confirms again",
            "Attempt `.env` write — always blocked",
            "Run full QA-P4 — steps 2, 3, 5 green",
        ],
        "specs": [
            ("Trust graph", "../../core/05-innovations.md#trust-graph"),
            ("Trust UI", "../../design/09-ui-ux-spec.md#trust-ui"),
            ("Permissions", "../../agent/08-agent-tools-and-skills.md#permissions"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "5-1-multi-lane.md",
        "id": "5.1",
        "title": "Multi-lane model",
        "anchor": "phase-5-1",
        "status": "[ ] Not started",
        "duration": "2–3 weeks",
        "track": "5 — Multi-lane & polish",
        "roadmap_link": "Phase 5",
        "roadmap_anchor": "#phase-5",
        "qa_script": "QA-P5 steps 2–4",
        "deliverable": "Parallel agent lanes with Explorer + Implementer presets, reviewer read-only lane, write conflict detection.",
        "depends": "3.2, 3b.2",
        "blocks": "5.2",
        "out_of_scope": [
            "Composer-first layout preset (5.2)",
            "Visual regression baselines (5.2)",
        ],
        "tasks": [
            ("5.1.1", "Lane tabs or split in agent panel", "`crates/agent_ui/`", "[ ]"),
            ("5.1.2", "Explorer + Implementer presets", "`crates/cuecode_sandbox/src/builtin_agents.rs`", "[ ]"),
            ("5.1.3", "Reviewer lane (read-only intent)", "`crates/agent_ui/`, `crates/cuecode_sandbox/`", "[ ]"),
            ("5.1.4", "Write conflict detection + banner", "`crates/agent_ui/`", "[ ]"),
            ("5.1.5", "`parent_session_id` / thread grouping", "`crates/acp_thread/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Lane tabs or split view in agent panel; each lane = separate `AcpThread`.
            - Explorer preset: Explore intent; Implementer: Fix/Ship intent.
            - Reviewer lane: Review intent — write tools denied.
            - Same-file edit from two lanes → conflict banner; no silent overwrite.
            - `parent_session_id` groups coordinator + worker threads."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p agent_ui -p acp_thread",
        "exit": [
            "Two lanes active without silent write conflicts — [07 §phase-5-acceptance](../07-implementation-roadmap.md#phase-5-acceptance) rows 1, 3",
        ],
        "qa": [
            "Open Explorer + Implementer lanes — shared spec in header",
            "Trigger same-file edit conflict — banner appears",
            "Reviewer lane — write tool denied",
        ],
        "specs": [
            ("Multi-lane", "../../core/05-innovations.md#multi-lane"),
            ("Coordinator", "../../parity/18-teams-and-tasks.md#coordinator"),
            ("Flow C", "../../parity/16-end-to-end-flows.md#flow-c-coordinator"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "5-2-composer-polish.md",
        "id": "5.2",
        "title": "Composer-first layout + polish",
        "anchor": "phase-5-2",
        "status": "[ ] Not started",
        "duration": "2–3 weeks",
        "track": "5 — Multi-lane & polish",
        "roadmap_link": "Phase 5",
        "roadmap_anchor": "#phase-5",
        "qa_script": "QA-P5 full",
        "deliverable": "Composer-first layout, terminal replay, context budget UI optional, notification polish, visual baselines. **Beta milestone.**",
        "depends": "5.1",
        "blocks": "6.1",
        "out_of_scope": [
            "Release DMG, docs site (6.1)",
            "Competitive 1.0 gate (6.2)",
        ],
        "tasks": [
            ("5.2.1", "Composer-first layout preset", "`crates/workspace/`, `crates/agent_ui/`", "[ ]"),
            ("5.2.2", "Project panel collapsed default", "`crates/project_panel/`", "[ ]"),
            ("5.2.3", "Terminal replay (basic re-run)", "`crates/agent_ui/`, `crates/terminal/`", "[ ]"),
            ("5.2.4", "Context budget UI (optional)", "`crates/agent_ui/`", "[ ]"),
            ("5.2.5", "Notification rail polish", "`crates/agent_ui/`", "[ ]"),
            ("5.2.6", "Visual regression baselines", "`crates/agent_ui/tests/` or screenshot CI", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Composer-first: agent panel ≥60% width on relaunch when preset enabled.
            - Project panel collapsed by default in composer-first mode.
            - Terminal replay: re-run captured commands from review Terminal tab.
            - Context budget UI: optional token/context meter in agent header."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p agent_ui -p workspace\n# Manual: QA-P5 full",
        "exit": [
            "[07 §phase-5-exit](../07-implementation-roadmap.md#phase-5-exit) all rows",
            "**Beta milestone** — [07 §beta-milestone](../07-implementation-roadmap.md#beta-milestone)",
        ],
        "qa": [
            "Enable composer-first — relaunch — agent panel ≥60% width",
            "Terminal replay — re-run command from review tab",
            "Run full QA-P5 — all pass steps green",
        ],
        "specs": [
            ("Composer-first layout", "../../design/09-ui-ux-spec.md#composer-first-layout"),
            ("Context budget", "../../core/05-innovations.md#context-budget"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "6-1-release-docs.md",
        "id": "6.1",
        "title": "Release artifacts + docs",
        "anchor": "phase-6-1",
        "status": "[ ] Not started",
        "duration": "Ongoing (first slice ~4 weeks)",
        "track": "6 — Ship",
        "roadmap_link": "Phase 6",
        "roadmap_anchor": "#phase-6",
        "qa_script": "QA-P6",
        "deliverable": "Rebranded release builds, docs site, example template repo, GPL source release process, crash reporting opt-in.",
        "depends": "5.2",
        "blocks": "6.2",
        "out_of_scope": [
            "Competitive 1.0 inventory gate (6.2)",
            "Enterprise BYOK cloud (C.4)",
        ],
        "tasks": [
            ("6.1.1", "macOS DMG / Linux tarball build scripts rebranded", "`script/bundle-*.sh`, release CI", "[ ]"),
            ("6.1.2", "Windows best-effort build notes", "`docs/`, `CONTRIBUTING.md`", "[ ]"),
            ("6.1.3", "CueCode docs site", "`docs/`", "[ ]"),
            ("6.1.4", "Example template repo with `.cursor/specs/` + skills", "external template repo", "[ ]"),
            ("6.1.5", "GPL source release process", "`CONTRIBUTING.md`, release docs", "[ ]"),
            ("6.1.6", "Crash reporting opt-in", "`crates/telemetry/`", "[ ]"),
            ("6.1.7", "Release notes template", "`AGENTS.md` or `docs/release-notes.md`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Release artifacts must not reference Zed branding or zed.dev update URLs.
            - Docs site covers: onboarding, intent profiles, review + rewind, `@spec`.
            - Template repo: minimal project with `.cursor/specs/` and `.cursor/skills/`.
            - GPL checklist: source offer, LICENSE, third-party notices."""
        ),
        "verify": "cd CueCode-IDE\n./script/bundle-mac.sh  # or platform equivalent\n# QA-P6 on clean VM",
        "exit": [
            "[07 §phase-6-exit](../07-implementation-roadmap.md#phase-6-exit) rows 1–3",
            "[07 §phase-6-acceptance](../07-implementation-roadmap.md#phase-6-acceptance) rows 1–3 pass",
        ],
        "qa": [
            "Install from release artifact (no `cargo build`)",
            "Complete onboarding all four screens",
            "Clone template repo — `@spec` works",
            "Follow docs \"review + rewind\" page — matches UI copy",
            "Run full QA-P6 on clean VM — all steps green",
        ],
        "specs": [
            ("GPL checklist", "../../core/03-fork-and-rebrand.md#gpl-checklist"),
            ("Build release", "../../ops/10-infrastructure.md#build-release"),
            ("Release gates", "../../ops/11-metrics-and-success.md#release-gates"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "6-2-competitive-gate.md",
        "id": "6.2",
        "title": "Competitive 1.0 gate",
        "anchor": "phase-6-2",
        "status": "[ ] Not started",
        "duration": "After 6.1 + parity backlog",
        "track": "6 — Ship",
        "roadmap_link": "Phase 6",
        "roadmap_anchor": "#phase-6",
        "qa_script": "Flows A–H",
        "deliverable": "Competitive 1.0: inventory complete, command surface mapped, Flows A–H pass, surface matrix published.",
        "depends": "6.1, 15 §P4",
        "blocks": "— (release gate)",
        "out_of_scope": [
            "Post-1.0 parity backlog items marked Defer",
            "Cloud M4 enterprise (C.4) — parallel track",
        ],
        "tasks": [
            ("6.2.1", "100% inventory rows Adopt/Adapt/Defer/Reject", "`research/00-claude-code-inventory.md`, `research/01-parity-decisions.md`", "[ ]"),
            ("6.2.2", "≥90% top-60 commands mapped in GPUI", "`parity/19-command-surface.md`", "[ ]"),
            ("6.2.3", "Flows A–H manual QA pass", "`parity/16-end-to-end-flows.md`", "[ ]"),
            ("6.2.4", "[21 §surface-matrix](../../parity/21-ai-surfaces.md#surface-matrix) published", "`parity/21-ai-surfaces.md`", "[ ]"),
            ("6.2.5", "Link all PRs to inventory row IDs", "PR process, `parity/15-competitive-parity.md`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Every Claude Code inventory row must have Adopt/Adapt/Defer/Reject decision in `01-parity-decisions.md`.
            - Top-60 commands: map to GPUI command palette / agent tools / settings.
            - Flows A–H: manual QA with recorded build hash, date, tester.
            - Surface matrix published in `21-ai-surfaces.md`."""
        ),
        "verify": "# Manual QA — Flows A–H\n# See parity/16-end-to-end-flows.md",
        "exit": [
            "[15 §competitive-gate](../../parity/15-competitive-parity.md#competitive-gate) all **Then** clauses pass",
        ],
        "qa": [
            "Run Flow A (daily coding) — pass",
            "Run Flow B (ship verify) — pass",
            "Run Flows C–H — pass",
            "Verify inventory 100% decided",
            "Verify surface matrix published",
        ],
        "specs": [
            ("Competitive gate", "../../parity/15-competitive-parity.md#competitive-gate"),
            ("End-to-end flows", "../../parity/16-end-to-end-flows.md"),
            ("Command surface", "../../parity/19-command-surface.md"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "c-0-chp-stub.md",
        "id": "C.0",
        "title": "CHP stub + one tool",
        "anchor": "phase-c-0",
        "status": "[~] In progress",
        "verified": "Partial — `cuecode-cloud-m0` repo exists",
        "duration": "~3 weeks",
        "track": "C — Cloud harness (parallel)",
        "roadmap_link": "Cloud M0",
        "roadmap_anchor": "#phase-3b",
        "qa_script": "M0 exit — CHP integration test",
        "deliverable": "CHP v0 message types; stub `harness-api`; `cuecode_cloud` crate; `read_file` round-trip integration test.",
        "depends": "2.1",
        "blocks": "C.1",
        "out_of_scope": [
            "Real model gateway, streaming (C.1)",
            "Subagent async cloud scheduler (C.2)",
        ],
        "tasks": [
            ("C.0.1", "CHP v0 message types", "`crates/cuecode_cloud/`, [03-protocol](../../harness/cloud/03-protocol.md)", "[~]"),
            ("C.0.2", "Stub harness-api", "private `cuecode-harness` / `cuecode-cloud-m0` repo", "[~]"),
            ("C.0.3", "`cuecode_cloud` crate + adapter", "`crates/cuecode_cloud/`", "[~]"),
            ("C.0.4", "`read_file` round-trip integration test", "`crates/cuecode_cloud/tests/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - CHP loop: `SessionCreate` → `UserMessage` → `ToolRequest` → `ToolResult` → `TurnComplete`.
            - Local dev: in-process stub server via feature flag `cloud_harness_stub`.
            - Client renders tool card identically to local agent.
            - Server rejects `edit_file` for explore `agent_type` in stub allowlist.
            - No proprietary prompt bodies in `cuecode_sandbox` — types only."""
        ),
        "verify": "cd CueCode-IDE\ncargo test -p cuecode_cloud\n./script/clippy -p cuecode_cloud",
        "exit": [
            "CHP `SessionCreate` → `UserMessage` → `ToolRequest` → `ToolResult` → `TurnComplete`",
            "Client renders tool card identically to local agent",
            "Server rejects `edit_file` for explore agent_type in stub allowlist",
            "`./script/clippy` clean on `cuecode_cloud`",
            "[08 §M0 exit](../../harness/cloud/08-roadmap.md#m0-exit) all rows",
        ],
        "qa": [
            "Enable `cloud_harness_stub` — send prompt — stub returns fake assistant + read_file",
            "Permission modal appears — approve — file contents return upstream",
            "Turn completes — no real model spend",
            "Integration test: mock HTTP server + read_file — green",
        ],
        "specs": [
            ("CHP protocol", "../../harness/cloud/03-protocol.md"),
            ("Cloud M0", "../../harness/cloud/08-roadmap.md#m0"),
            ("Open client", "../../harness/cloud/04-open-client.md"),
            ("Tool host", "../../harness/cloud/06-tool-host.md"),
        ],
        "changelog": "| 2026-06-20 | Generated; partial M0 in cuecode-cloud-m0 |",
    },
    {
        "file": "c-1-streaming-cloud.md",
        "id": "C.1",
        "title": "Streaming default cloud",
        "anchor": "phase-c-1",
        "status": "[ ] Not started",
        "duration": "~4 weeks",
        "track": "C — Cloud harness (parallel)",
        "roadmap_link": "Cloud M1",
        "roadmap_anchor": "#phase-3b",
        "qa_script": "M1 exit — stream latency",
        "deliverable": "Production-shaped streaming; cloud agent default in CueCode cloud build; model-gateway v1; offline Model A fallback.",
        "depends": "C.0, 1.2",
        "blocks": "C.2",
        "out_of_scope": [
            "Background spawn cloud (C.2)",
            "BYOK enterprise (C.4)",
        ],
        "tasks": [
            ("C.1.1", "Gateway router + fast/quality tiers", "private `cuecode-harness` model-gateway", "[ ]"),
            ("C.1.2", "Turn engine persistent transcript store", "private harness-api", "[ ]"),
            ("C.1.3", "Client stream decoder → `acp_thread`", "`crates/cuecode_cloud/`", "[ ]"),
            ("C.1.4", "Onboarding: cloud vs local explainer", "`crates/ai_onboarding/`", "[ ]"),
            ("C.1.5", "Cloud build flag (`release_channel = cloud`)", "`crates/release_channel/`", "[ ]"),
            ("C.1.6", "Context assembly v1 — spec index from client snapshot", "`crates/cuecode_cloud/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - model-gateway v1: CueCode-managed keys only (BYOK deferred to C.4).
            - Streaming `SessionUpdate` → client decoder → `acp_thread` token stream.
            - `release_channel = cloud` enables harness default; GPL tarball stays Model A.
            - Offline / air-gap: fallback to local agent when no auth.
            - Compaction v0 preserves linked spec path."""
        ),
        "verify": "cd CueCode-IDE\ncargo build -p cuecode_cloud --features cloud\n# Measure first token < 3s p95",
        "exit": [
            "First token < 3s p95 (managed route, US)",
            "Rate limit UX matches [07 §rate-limits](../../harness/cloud/07-model-gateway.md#rate-limits)",
            "Compaction v0 preserves linked spec path",
            "Cloud build launches with harness default; GPL build unchanged",
            "[08 §M1 exit](../../harness/cloud/08-roadmap.md#m1-exit) all rows",
        ],
        "qa": [
            "Cloud build — sign in — default agent streams via harness-api",
            "GPL build — no cloud sign-in required — local agent works",
            "Air-gap / no auth — falls back to Model A",
            "Rate limit hit — UX matches spec",
        ],
        "specs": [
            ("Model gateway", "../../harness/cloud/07-model-gateway.md"),
            ("Turn engine", "../../harness/cloud/05-cloud-services.md#turn-engine"),
            ("Cloud M1", "../../harness/cloud/08-roadmap.md#m1"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "c-2-subagent-async.md",
        "id": "C.2",
        "title": "Subagent async",
        "anchor": "phase-c-2",
        "status": "[ ] Not started",
        "duration": "~4 weeks",
        "track": "C — Cloud harness (parallel)",
        "roadmap_link": "Cloud M2",
        "roadmap_anchor": "#phase-3b",
        "qa_script": "M2 exit — QA-P3b cloud variant",
        "deliverable": "Cloud scheduler Async queue; `SpawnSubagent` CHP; `SessionNotification` push; sidechain transcript sync.",
        "depends": "C.1, 3b.1",
        "blocks": "C.3",
        "out_of_scope": [
            "VERDICT hybrid handoffs (C.3)",
            "BYOK (C.4)",
        ],
        "tasks": [
            ("C.2.1", "Server subagent spawner + builtin registry", "private harness-api scheduler", "[ ]"),
            ("C.2.2", "CHP notification subscription (WebSocket or SSE)", "`crates/cuecode_cloud/`", "[ ]"),
            ("C.2.3", "Client `ExecutionContext` on spawn from composer", "`crates/cuecode_cloud/`, `crates/agent_ui/`", "[ ]"),
            ("C.2.4", "EC-20: duplicate `session_id` resume handling", "`crates/cuecode_cloud/`", "[ ]"),
            ("C.2.5", "Task pills + rail — data from CHP", "`crates/agent_ui/`", "[ ]"),
            ("C.2.6", "Sidechain transcript sync (cloud SoT)", "`crates/cuecode_cloud/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Same `ExecutionContext` enum on wire as local `cuecode_sandbox`; server implements queues.
            - `SpawnSubagent` CHP: explore + verification builtins.
            - `SessionNotification` push maps to local notification kinds (reuse 3b.2 rail UI).
            - Async lane does not block composer (configurable block flag for Active spawn).
            - Sidechain view loads from cloud sync — cloud is source of truth."""
        ),
        "verify": "cd CueCode-IDE\ncargo test -p cuecode_cloud -- spawn\n# Manual: background explore + notification",
        "exit": [
            "Background explore completes with notification (matches QA-P3b)",
            "`session_id` resume without duplicate thread",
            "Async lane does not block composer (configurable)",
            "Sidechain view loads from cloud sync",
            "[08 §M2 exit](../../harness/cloud/08-roadmap.md#m2-exit) all rows",
        ],
        "qa": [
            "Spawn background explore while typing in main composer",
            "Notification on complete — pill + rail",
            "Resume child via `session_id` — no duplicate thread",
            "Explore subagent cannot edit — enforced server + client",
        ],
        "specs": [
            ("Scheduler", "../../harness/cloud/05-cloud-services.md#scheduler"),
            ("Part B async (local)", "../../harness/local/01-agent-harness.md#part-b-async"),
            ("Cloud M2", "../../harness/cloud/08-roadmap.md#m2"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "c-3-verdict-hybrid.md",
        "id": "C.3",
        "title": "VERDICT Hybrid",
        "anchor": "phase-c-3",
        "status": "[ ] Not started",
        "duration": "~3 weeks",
        "track": "C — Cloud harness (parallel)",
        "roadmap_link": "Cloud M3",
        "roadmap_anchor": "#phase-3b",
        "qa_script": "M3 exit — VERDICT dogfood",
        "deliverable": "Cloud VERDICT parser; FAIL blocks session complete; hybrid handoff artifacts; plan→implement→verify pipeline.",
        "depends": "C.2, 3b.2",
        "blocks": "C.4",
        "out_of_scope": [
            "BYOK enterprise SSO (C.4)",
            "Full Orchestrate coordinator (minimal v1 only)",
        ],
        "tasks": [
            ("C.3.1", "Verification agent outline + parser tests", "private harness-api verdict service", "[ ]"),
            ("C.3.2", "Artifact store for evidence markdown", "private harness-api", "[ ]"),
            ("C.3.3", "Client override audit log entry", "`crates/cuecode_cloud/`", "[ ]"),
            ("C.3.4", "Coordinator spawn (Orchestrate intent) — minimal v1", "`crates/cuecode_sandbox/`, harness-api", "[ ]"),
            ("C.3.5", "Plan import to `AcpThread.plan`", "`crates/cuecode_cloud/`", "[ ]"),
            ("C.3.6", "Unified review on VERDICT click", "`crates/agent_ui/`", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - Ship flow: approve plan (active) → implement worker (async) → verify (async) → VERDICT PASS → complete.
            - VERDICT FAIL: structured notification — not prose inference; blocks session complete (EC-16).
            - Hybrid handoff artifacts per [local §C.5](../../harness/local/01-agent-harness.md#c-5-hybrid-handoff-artifacts-required).
            - Override writes append-only audit event (local + server)."""
        ),
        "verify": "# Dogfood on internal repo\n# VERDICT PASS and FAIL paths",
        "exit": [
            "VERDICT FAIL shows structured notification — not prose inference",
            "FAIL unacknowledged blocks session complete",
            "Hybrid flow produces plan + checkpoint + VERDICT artifact chain",
            "PASS path dogfood on internal repo",
            "[08 §M3 exit](../../harness/cloud/08-roadmap.md#m3-exit) all rows",
        ],
        "qa": [
            "Ship flow end-to-end — VERDICT PASS — mark complete",
            "Inject FAIL — blocks complete until override",
            "Click VERDICT notification — lands in unified review",
            "Override audit entry written",
        ],
        "specs": [
            ("VERDICT service", "../../harness/cloud/05-cloud-services.md#verdict"),
            ("Part C hybrid", "../../harness/local/01-agent-harness.md#part-c-hybrid"),
            ("Cloud M3", "../../harness/cloud/08-roadmap.md#m3"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
    {
        "file": "c-4-byok-enterprise.md",
        "id": "C.4",
        "title": "BYOK enterprise",
        "anchor": "phase-c-4",
        "status": "[ ] Not started",
        "duration": "~6 weeks",
        "track": "C — Cloud harness (parallel)",
        "roadmap_link": "Cloud M4",
        "roadmap_anchor": "#phase-3b",
        "qa_script": "M4 exit — SSO + BYOK smoke",
        "deliverable": "BYOK passthrough gateway; org admin console; enterprise SSO; regional routing; audit export.",
        "depends": "C.3, 4.1",
        "blocks": "— (cloud track complete)",
        "out_of_scope": [
            "GPL tarball cloud features",
            "Post-M4 compliance certifications",
        ],
        "tasks": [
            ("C.4.1", "BYOK encrypted key upload", "model-gateway, settings UI", "[ ]"),
            ("C.4.2", "Org admin console (seats, usage, route policy)", "private admin console", "[ ]"),
            ("C.4.3", "SSO (SAML/OIDC)", "private auth service", "[ ]"),
            ("C.4.4", "Regional routing (EU pool)", "model-gateway", "[ ]"),
            ("C.4.5", "Audit export (transcript + tool log JSONL)", "harness-api + admin console", "[ ]"),
            ("C.4.6", "Usage dashboard: tokens per team", "admin console", "[ ]"),
        ],
        "impl": textwrap.dedent(
            """\
            - BYOK: encrypted key upload; gateway passthrough with audit; key not stored at rest after session.
            - SSO login → org JWT → harness access.
            - Regional routing: EU pool for data residency.
            - GDPR export: transcript JSONL download for compliance officers."""
        ),
        "verify": "# SSO smoke test\n# BYOK session completes without key at rest\n# Audit export download",
        "exit": [
            "BYOK session completes without storing key at rest",
            "SSO login → org JWT → harness access",
            "Usage dashboard: tokens per team",
            "GDPR export: transcript JSONL download",
            "[08 §M4 exit](../../harness/cloud/08-roadmap.md#m4-exit) all rows",
        ],
        "qa": [
            "Upload BYOK key — session completes — key not at rest after",
            "SSO login — org JWT — harness access granted",
            "Admin console — usage dashboard shows tokens per team",
            "Export transcript JSONL — complete audit trail",
        ],
        "specs": [
            ("BYOK flow", "../../harness/cloud/07-model-gateway.md#byok-flow"),
            ("Keys BYOK", "../../harness/cloud/07-model-gateway.md#keys-byok"),
            ("Cloud M4", "../../harness/cloud/08-roadmap.md#m4"),
            ("Trust store (local)", "../../core/05-innovations.md#trust-graph"),
        ],
        "changelog": "| 2026-06-20 | Initial sub-phase doc |",
    },
]

EXPECTED_FILES = {p["file"] for p in PHASES}


def main() -> int:
    OUT.mkdir(parents=True, exist_ok=True)
    written: list[str] = []
    for p in PHASES:
        path = OUT / p["file"]
        path.write_text(render(p), encoding="utf-8")
        written.append(str(path.relative_to(ROOT)))
    print(f"Wrote {len(written)} phase docs to {OUT.relative_to(ROOT)}/")
    for name in sorted(written):
        print(f"  {name}")
    if len(PHASES) != 22:
        print(f"WARNING: expected 22 phases, defined {len(PHASES)}")
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
