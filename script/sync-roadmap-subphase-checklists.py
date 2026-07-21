#!/usr/bin/env python3
"""Inject per-sub-phase task checklists from build-plans/phases/ into 07-implementation-roadmap.md."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PHASES_DIR = ROOT / ".cursor/specs/delivery/build-plans/phases"
ROADMAP = ROOT / ".cursor/specs/delivery/07-implementation-roadmap.md"

# (roadmap phase key, list of phase file stems without .md)
PHASE_GROUPS: list[tuple[str, list[str], str]] = [
    (
        "phase-0-tasks",
        ["0-1-identity", "0-2-cloud-decouple", "0-3-packaging-qa"],
        "Track cleanup passes in [03-zed-reference-cleanup-phases → Progress](../core/03-zed-reference-cleanup-phases#progress). "
        "Verify with `./script/rebrand-progress.sh --full`.",
    ),
    (
        "phase-1-tasks",
        ["1-1-cuecode-specs", "1-2-agent-spec-integration", "1-3-spec-ui-stub"],
        "",
    ),
    (
        "phase-2-tasks",
        ["2-1-intent-core", "2-2-intent-ui"],
        "",
    ),
    (
        "phase-3-tasks",
        ["3-1-checkpoint-store", "3-2-review-panel"],
        "",
    ),
    (
        "phase-3b-tasks",
        ["3b-1-background-spawn", "3b-2-notification-verdict"],
        "Cloud track (parallel): [C.0](./build-plans/phases/c-0-chp-stub.md) · [C.1](./build-plans/phases/c-1-streaming-cloud.md) · "
        "[C.2](./build-plans/phases/c-2-subagent-async.md) · [C.3](./build-plans/phases/c-3-verdict-hybrid.md) · "
        "[C.4](./build-plans/phases/c-4-byok-enterprise.md) — see [#progress-cloud](#progress-cloud).",
    ),
    (
        "phase-4-tasks",
        ["4-1-trust-store"],
        "",
    ),
    (
        "phase-5-tasks",
        ["5-1-multi-lane", "5-2-composer-polish"],
        "",
    ),
    (
        "phase-6-tasks",
        ["6-1-release-docs", "6-2-competitive-gate"],
        "",
    ),
]

CLOUD_STEMS = [
    "c-0-chp-stub",
    "c-1-streaming-cloud",
    "c-2-subagent-async",
    "c-3-verdict-hybrid",
    "c-4-byok-enterprise",
]

TASK_ROW = re.compile(
    r"^\| (?P<id>[\w.]+) \| (?P<task>[^|]+) \| (?P<files>[^|]+) \| `(?P<done>\[[ x~]\])` \|"
)


def parse_phase_file(path: Path) -> dict:
    text = path.read_text()
    title_m = re.search(r"^# Build phase ([^\s]+) — (.+?) \{#", text, re.M)
    status_m = re.search(r"\*\*Status\*\* \| `(.*?)`", text)
    anchor_m = re.search(r"\{#(phase-[^}]+)\}", text)

    tasks: list[tuple[str, str, str, str]] = []
    in_tasks = False
    for line in text.splitlines():
        if line.startswith("## Tasks "):
            in_tasks = True
            continue
        if in_tasks and line.startswith("---"):
            break
        if in_tasks:
            m = TASK_ROW.match(line)
            if m:
                done = m.group("done")
                # normalize [~] to [ ] for roadmap open items, keep [x]
                if done == "[~]":
                    done = "[ ]"
                tasks.append((m.group("id"), m.group("task").strip(), m.group("files").strip(), done))

    return {
        "id": title_m.group(1) if title_m else path.stem,
        "title": title_m.group(2).strip() if title_m else path.stem,
        "status": status_m.group(1) if status_m else "[ ] Not started",
        "anchor": anchor_m.group(1) if anchor_m else path.stem,
        "tasks": tasks,
        "path": f"./build-plans/phases/{path.name}",
    }


def clean_cell(text: str) -> str:
    return text.replace("`", "").strip()


def checklist_item(task_id: str, task: str, files: str, done: str) -> str:
    files_clean = clean_cell(files)
    return f"- {done} **{task_id}** {task} — `{files_clean}`"


def render_subphase_block(meta: dict) -> str:
    lines = [
        f"#### Build phase {meta['id']} — {meta['title']} `{meta['status']}`",
        "",
        f"Full doc: [{meta['path'].split('/')[-1]}]({meta['path']})",
        "",
    ]
    for tid, task, files, done in meta["tasks"]:
        lines.append(checklist_item(tid, task, files, done))
    lines.append("")
    return "\n".join(lines)


def render_tasks_section(section_id: str, stems: list[str], intro: str) -> str:
    parts = [
        f"### Tasks {{#{section_id}}}",
        "",
        "> **Check boxes here and in [build-plans/phases/](./build-plans/phases/)** — keep both in sync when a task completes.",
        "",
    ]
    if intro:
        parts.append(intro)
        parts.append("")

    for stem in stems:
        meta = parse_phase_file(PHASES_DIR / f"{stem}.md")
        parts.append(render_subphase_block(meta))

    if section_id == "phase-0-tasks":
        parts.extend(
            [
                "#### Phase 0 cross-cutting (roadmap)",
                "",
                "- [x] Verify data dir isolation: no shared KV store keys with upstream Zed — `qa-config-isolation.sh` + `qa-p0.sh` Step 5 *(gate)*",
                "- [x] Document BYOK setup in onboarding copy — [09-ui-ux-spec](../design/09-ui-ux-spec#onboarding) · `crates/ai_onboarding/`",
                "- [x] `./script/clippy` passes on touched crates — rebrand crate set *(gate)*",
                "- [x] Create this `.cursor/specs/` directory",
                "- [x] Link specs from CONTRIBUTING / README — `CueCode-IDE/CONTRIBUTING.md`",
                "- [x] Agent Ollama + network idle gates — see [#progress-phase-0](#progress-phase-0)",
                "",
            ]
        )

    return "\n".join(parts)


def render_progress_rollup(stems: list[str], heading: str, anchor: str) -> str:
    lines = [f"### {heading} {{#{anchor}}}", ""]
    for stem in stems:
        meta = parse_phase_file(PHASES_DIR / f"{stem}.md")
        done_count = sum(1 for *_, d in meta["tasks"] if d == "[x]")
        total = len(meta["tasks"])
        lines.append(
            f"**{meta['id']}** `{meta['status']}` — [{meta['title']}]({meta['path']}) "
            f"({done_count}/{total} tasks)"
        )
        for tid, task, files, done in meta["tasks"]:
            lines.append(checklist_item(tid, task, files, done))
        lines.append("")
    return "\n".join(lines)


def replace_section(content: str, start_marker: str, end_marker: str, new_body: str) -> str:
    pattern = re.compile(
        rf"(### Tasks \{{#{start_marker}\}}.*?)(?=### Exit criteria \{{#{end_marker}\}})",
        re.S,
    )
    m = pattern.search(content)
    if not m:
        raise SystemExit(f"Could not find section {start_marker}")
    return content[: m.start()] + new_body + "\n\n" + content[m.end() :]


def replace_progress_block(content: str, new_block: str) -> str:
    pattern = re.compile(
        r"(## Progress \{#progress\}.*?)(?=---\n\n## Timeline overview)",
        re.S,
    )
    m = pattern.search(content)
    if not m:
        raise SystemExit("Could not find ## Progress section")
    return content[: m.start()] + new_block + content[m.end() :]


def build_progress_section() -> str:
    lines = [
        "## Progress {#progress}",
        "",
        "**Last verified:** 2026-06-20 (`./script/rebrand-progress.sh` → rebrand-check green; run `--full` for qa-p0)",
        "",
        "**Workflow:** finish a sub-phase → check tasks in [#phase-N-tasks](#phase-0-tasks) **and** "
        "[build-plans/phases/](./build-plans/phases/) → update **Status** in the sub-phase file → sync here.",
        "",
        "| Phase | State | Sub-phases |",
        "|-------|-------|------------|",
        "| **P0** | Mostly done | [0.1](#progress-phase-0) · [0.2](#progress-phase-0) · [0.3](#progress-phase-0) |",
        "| **P1** | Not started | [1.1–1.3](#progress-phase-1) |",
        "| **P2** | Not started | [2.1–2.2](#progress-phase-2) |",
        "| **P3** | Not started | [3.1–3.2](#progress-phase-3) |",
        "| **P3b** | Not started | [3b.1–3b.2](#progress-phase-3b) |",
        "| **P4** | Not started | [4.1](#progress-phase-4) |",
        "| **P5** | Not started | [5.1–5.2](#progress-phase-5) |",
        "| **P6** | Not started | [6.1–6.2](#progress-phase-6) |",
        "| **Cloud** | C.0 partial | [C.0–C.4](#progress-cloud) |",
        "",
        "Full index: [build-plans README §phase-index](./build-plans/README.md#phase-index) · "
        "**Next:** [Build phase 1.1](./build-plans/phases/1-1-cuecode-specs.md)",
        "",
    ]

    lines.append(render_progress_rollup(["0-1-identity", "0-2-cloud-decouple", "0-3-packaging-qa"], "Phase 0 sub-phases", "progress-phase-0"))

    # Phase 0 cross-cutting items not in build phase task IDs
    lines.extend(
        [
            "#### Phase 0 cross-cutting (roadmap)",
            "",
            "- [x] Verify data dir isolation: no shared KV store keys with upstream Zed — `qa-config-isolation.sh` + `qa-p0.sh` Step 5 *(gate)*",
            "- [x] Document BYOK setup in onboarding copy — [09-ui-ux-spec](../design/09-ui-ux-spec#onboarding) · `crates/ai_onboarding/`",
            "- [x] `./script/clippy` passes on touched crates — rebrand crate set *(gate)*",
            "- [x] Agent prompt via Ollama — `qa-agent-ollama.sh` + CI *(gate)*",
            "- [x] Idle app does not phone `zed.dev` — `network-idle-audit.sh` *(gate)*",
            "- [x] Cleanup passes A–E — [03-zed-reference-cleanup-phases](../core/03-zed-reference-cleanup-phases#progress)",
            "- [ ] Optional: [doc screenshots backlog](../build-plans/README.md#doc-screenshots-backlog) (10 CueCode re-captures) · [auto-update E2E](../core/03-pass-c-auto-update-smoke-test.md)",
            "",
            "**Phase 0 not complete until** cross-cutting items above are `[x]` and [#phase-0-exit](#phase-0-exit) passes.",
            "",
        ]
    )

    for stems, heading, anchor in [
        (["1-1-cuecode-specs", "1-2-agent-spec-integration", "1-3-spec-ui-stub"], "Phase 1 sub-phases", "progress-phase-1"),
        (["2-1-intent-core", "2-2-intent-ui"], "Phase 2 sub-phases", "progress-phase-2"),
        (["3-1-checkpoint-store", "3-2-review-panel"], "Phase 3 sub-phases", "progress-phase-3"),
        (["3b-1-background-spawn", "3b-2-notification-verdict"], "Phase 3b sub-phases", "progress-phase-3b"),
        (["4-1-trust-store"], "Phase 4 sub-phases", "progress-phase-4"),
        (["5-1-multi-lane", "5-2-composer-polish"], "Phase 5 sub-phases", "progress-phase-5"),
        (["6-1-release-docs", "6-2-competitive-gate"], "Phase 6 sub-phases", "progress-phase-6"),
    ]:
        lines.append(render_progress_rollup(stems, heading, anchor))

    lines.append(render_progress_rollup(CLOUD_STEMS, "Cloud track sub-phases", "progress-cloud"))

    lines.append("---\n")
    return "\n".join(lines)


def main() -> None:
    content = ROADMAP.read_text()

    for section_id, stems, intro in PHASE_GROUPS:
        exit_id = section_id.replace("-tasks", "-exit")
        new_body = render_tasks_section(section_id, stems, intro)
        content = replace_section(content, section_id, exit_id, new_body)

    content = replace_progress_block(content, build_progress_section())

    ROADMAP.write_text(content)
    print(f"Updated {ROADMAP}")


if __name__ == "__main__":
    main()
