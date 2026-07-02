#!/usr/bin/env python3
"""Pass D second pass: fix remaining product-facing Zed strings in docs."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"

REPLACEMENTS = [
    ("Clone the [Zed repository]", "Clone the [CueCode repository]"),
    ("https://github.com/zed-industries/zed", "https://github.com/CueCard-AI/CueCode-IDE"),
    ("after Zed creates", "after CueCode creates"),
    ("Zed creates a linked", "CueCode creates a linked"),
    ("Zed creates a new", "CueCode creates a new"),
    ("Zed has built-in", "CueCode has built-in"),
    ("Zed will also load", "CueCode will also load"),
    ("Zed supports Python", "CueCode supports Python"),
    ("how Zed runs", "how CueCode runs"),
    ("Zed consistently fails", "CueCode consistently fails"),
    ("have Zed use", "have CueCode use"),
    ("and Zed is still", "and CueCode is still"),
    ("On Linux, Zed reads", "On Linux, CueCode reads"),
    ("Zed user settings", "CueCode user settings"),
    ("your Zed user settings", "your CueCode user settings"),
    ("local Zed settings", "local CueCode user settings"),
    ("the Zed log", "the CueCode log"),
    ("`provider` field should be set to `Zed AI`", "`provider` field should be set to `cuecode`"),
    ("For example, Zed uses this", "For example, CueCode uses this"),
    ("refer to the Zed [built-in languages]", "refer to the CueCode [built-in languages]"),
    ("[Zed Language-Specific Documentation]", "[CueCode language documentation]"),
    ("Zed doesn't generate", "CueCode doesn't generate"),
    ("You'll recreate them as Zed [tasks]", "You'll recreate them as CueCode [tasks]"),
    ("**Zed is open source**", "**CueCode is open source**"),
    ("building (or running) Zed with", "building (or running) CueCode with"),
    ("Zed automatically installs", "CueCode automatically installs"),
    ("Zed connects to several", "CueCode connects to several online services by default (when enabled)"),
    ("Zed currently does not work", "CueCode currently does not work"),
    ("If telemetry is enabled, Zed uploads", "If telemetry is enabled, CueCode uploads"),
    ("(both are Zed-staff-only)", "(upstream Zed staff only)"),
    ("Until a worktree is trusted, Zed does not run", "Until a worktree is trusted, CueCode does not run"),
    ("By default, Zed does not trust", "By default, CueCode does not trust"),
    ("~/.config/zed/settings.json", "~/.config/cuecode/settings.json"),
    ("~/.zed/settings.json", "~/.config/cuecode/settings.json"),
    ("ZED_UPDATE_EXPLANATION=\"Please use flatpak to update zed.\"", 'CUECODE_UPDATE_EXPLANATION="Please use flatpak to update CueCode."'),
    ("building (or running) CueCode with the environment variable `ZED_UPDATE_EXPLANATION`", "building (or running) CueCode with the environment variable `CUECODE_UPDATE_EXPLANATION`"),
    ("version** of Zed to install using the `ZED_VERSION`", "version** of CueCode to install using the `CUECODE_VERSION`"),
    ("alt=\"Zed Industries\"", 'alt="CueCode"'),
]

KEEP_LINE = [
    "Zed Industries",
    "zed-industries",
    "ZED_WORKTREE_ROOT",
    "ZED_MAIN_GIT_WORKTREE",
    "ZED_DEVICE_ID",
    "ZED_FONTS_",
    "ZED_VERSION",
    "ZED_UPDATE_EXPLANATION",
    ".zed/",
    "zed://",
    "zed.dev/cla",
    "zed-editor",
    "Repology",
    "fork of Zed",
]


def fix_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    out = []
    changed = False
    for line in lines:
        new = line
        if not any(k in line for k in KEEP_LINE):
            for old, repl in REPLACEMENTS:
                if old in new:
                    new = new.replace(old, repl)
            if re.search(r"\bZed\b", new) and not any(k in new for k in KEEP_LINE):
                new = re.sub(r"\bZed\b", "CueCode", new)
        if new != line:
            changed = True
        out.append(new)
    if changed:
        path.write_text("".join(out), encoding="utf-8")
    return changed


def main() -> None:
    n = 0
    for path in sorted(DOCS.rglob("*")):
        if path.suffix in {".md", ".hbs", ".toml"} and path.is_file():
            if fix_file(path):
                n += 1
    print(f"second pass updated {n} files")


if __name__ == "__main__":
    main()
