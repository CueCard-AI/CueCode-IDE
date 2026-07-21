#!/usr/bin/env python3
"""Pass D: rebrand product copy in docs, legal headers, and contributing guides."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# If any of these appear on a line, skip generic \bZed\b replacement on that line.
LINE_KEEP_SUBSTRINGS = [
    "Zed Industries",
    "zed-industries",
    "fork of Zed",
    "Zed fork",
    "upstream Zed",
    "Based on Zed",
    "GPL",
    ".zed/",
    "zed://",
    "ZED_",
    "zed:extension",
    "zed.dev/cla",
    "zed.dev/code-of-conduct",
    "github.com/zed-industries",
    "Repology",
    "zed-editor/zed/zed",  # upstream homebrew tap
    "package zed-editor",  # nix third-party
    "CueCode fork notice",
]

# Ordered replacements applied to every eligible line (after keep check for Zed word).
ORDERED_REPLACEMENTS = [
    ("https://zed.dev/docs", "https://cuecode.dev/docs"),
    ("https://www.zed.dev", "https://cuecode.dev"),
    ("https://zed.dev", "https://cuecode.dev"),
    ("http://zed.dev", "https://cuecode.dev"),
    ("{#kb zed::", "{#kb cuecode::"),
    ("{#action zed::", "{#action cuecode::"),
    ("zed::OpenSettings", "cuecode::OpenSettings"),
    ("zed::OpenSettingsFile", "cuecode::OpenSettingsFile"),
    ("Zed Agent", "CueCode Agent"),
    ("Zed Pro", "CueCode Pro"),
    ("Zed-hosted", "CueCode-hosted"),
    ("Zed-Hosted", "CueCode-Hosted"),
    ("Zed Business", "CueCode Business"),
    ("Zed Code Editor", "CueCode Editor"),
    ("Zed Code", "CueCode"),
    ("Zed Team", "CueCode Team"),
    ("Configuring Zed", "Configuring CueCode"),
    ("configuring-zed.md", "configuring-cuecode.md"),
    ("configuring-zed.html", "configuring-cuecode.html"),
    ("zed-agent.md", "cuecode-agent.md"),
    ("zed-agent.html", "cuecode-agent.html"),
    ("Zed Agent", "CueCode Agent"),
    ("the Zed editor", "CueCode"),
    ("Zed editor", "CueCode"),
    ("Zed's", "CueCode's"),
    ("Zed software", "CueCode software"),
    ("Open Zed", "Open CueCode"),
    ("in Zed", "in CueCode"),
    ("from Zed", "from CueCode"),
    ("to Zed", "to CueCode"),
    ("with Zed", "with CueCode"),
    ("using Zed", "using CueCode"),
    ("run Zed", "run CueCode"),
    ("launch Zed", "launch CueCode"),
    ("install Zed", "install CueCode"),
    ("Install Zed", "Install CueCode"),
    ("update Zed", "update CueCode"),
    ("quit Zed", "quit CueCode"),
    ("restart Zed", "restart CueCode"),
    ("about Zed", "about CueCode"),
    ("for Zed", "for CueCode"),
    ("on Zed", "on CueCode"),
    ("into Zed", "into CueCode"),
    ("through Zed", "through CueCode"),
    ("within Zed", "within CueCode"),
    ("inside Zed", "inside CueCode"),
    ("`zed .`", "`cuecode .`"),
    ("`zed` CLI", "`cuecode` CLI"),
    ("the `zed` CLI", "the `cuecode` CLI"),
    ("Zed CLI", "CueCode CLI"),
    ("Zed binary", "CueCode binary"),
    ("Zed binary", "CueCode binary"),
    ("~/.local/zed.app", "~/.local/cuecode.app"),
    ("zed.app", "cuecode.app"),
    ("libexec/zed-editor", "libexec/cuecode-editor"),
    ("dev.zed.Zed", "dev.cuecode.CueCode"),
    ("Zed.dmg", "CueCode.dmg"),
    ("Zed.exe", "CueCode.exe"),
    ("Zed-aarch64", "CueCode-aarch64"),
    ("Zed-x86_64", "CueCode-x86_64"),
    ("Zed Preview", "CueCode Preview"),
    ("Zed Nightly", "CueCode Nightly"),
    ("Zed Dev", "CueCode Dev"),
    ("Zed Stable", "CueCode Stable"),
    ("developing-zed", "developing-cuecode"),
    ("contribute-to-zed", "contribute-to-cuecode"),
    ("Learn how to use and customize Zed,", "Learn how to use and customize CueCode,"),
    ("title = \"Zed\"", 'title = "CueCode"'),
    ('authors = ["The Zed Team"]', 'authors = ["The CueCode Team"]'),
    ("# Zed Documentation Guidelines", "# CueCode Documentation Guidelines"),
    ("# Contributing to Zed (upstream)", "# Contributing to CueCode"),
    ("make Zed better", "make CueCode better"),
    ("Contributing to Zed", "Contributing to CueCode"),
    ("Packaging Zed", "Packaging CueCode"),
    ("view of Zed", "view of CueCode"),
    ("Bird's-eye view of Zed", "Bird's-eye view of CueCode"),
    ("Building Zed", "Building CueCode"),
    ("Zed glossary", "CueCode glossary"),
    ("Zed Extension API", "CueCode extension API"),
    ("Zed Extension", "CueCode extension"),
    ("Zed culture", "CueCode culture"),
    ("Zed Feature Process", "CueCode feature process"),
    ("Zed forums", "CueCode forums"),
    ("Zed staff", "CueCode maintainers"),
    ("Zed's design", "CueCode's design"),
    ("Zed’s design", "CueCode's design"),
    ("Zed’s voice", "CueCode's voice"),
    ("Zed's voice", "CueCode's voice"),
    ("Zed jargon", "CueCode jargon"),
    ("Zed site", "CueCode site"),
    ("Zed documentation", "CueCode documentation"),
    ("Zed docs", "CueCode docs"),
    ("Zed packages", "CueCode packages"),
    ("install Zed using", "install CueCode using"),
    ("downloadable Zed software", "downloadable CueCode software"),
]

ZED_WORD = re.compile(r"\bZed\b")


def line_keeps_zed(line: str) -> bool:
    return any(keep in line for keep in LINE_KEEP_SUBSTRINGS)


def rebrand_line(line: str) -> str:
    new = line
    for old, repl in ORDERED_REPLACEMENTS:
        new = new.replace(old, repl)
    if not line_keeps_zed(new):
        new = ZED_WORD.sub("CueCode", new)
    return new


def rebrand_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    lines = text.splitlines(keepends=True)
    out = [rebrand_line(line) for line in lines]
    new_text = "".join(out)
    if new_text != text:
        path.write_text(new_text, encoding="utf-8")
        return True
    return False


def main() -> int:
    targets: list[Path] = []

    docs = ROOT / "docs"
    for pattern in ("**/*.md", "**/*.hbs", "**/.rules", "book.toml"):
        targets.extend(docs.glob(pattern))

    legal = ROOT / "legal"
    if legal.is_dir():
        targets.extend(legal.glob("**/*.md"))

    for rel in ("CONTRIBUTING.md",):
        p = ROOT / rel
        if p.exists():
            targets.append(p)

    repo_readme = ROOT.parent / "README.md"
    if repo_readme.exists():
        targets.append(repo_readme)

    changed = 0
    for path in sorted(set(targets)):
        if path.is_file() and rebrand_file(path):
            changed += 1
            print(path.relative_to(ROOT))

    print(f"Updated {changed} files", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
