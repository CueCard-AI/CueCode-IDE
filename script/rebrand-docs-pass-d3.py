#!/usr/bin/env python3
"""Pass D third pass: paths, CDN URLs, and zed.dev links in docs/."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOCS = ROOT / "docs"

# Order matters — longer / more specific first.
REPLACEMENTS: list[tuple[str, str]] = [
    ("https://images.zed.dev/docs/troubleshooting/instruments-template-picker.webp", "../assets/images/troubleshooting/instruments-template-picker.webp"),
    ("https://images.zed.dev/docs/troubleshooting/instruments-target-and-record.webp", "../assets/images/troubleshooting/instruments-target-and-record.webp"),
    ("https://images.zed.dev/docs/troubleshooting/instruments-recording.webp", "../assets/images/troubleshooting/instruments-recording.webp"),
    ("https://images.zed.dev/docs/project-panel/panel.png", "../assets/images/project-panel/panel.png"),
    ("https://images.zed.dev/docs/project-panel/sticky-scroll-false.png", "../assets/images/project-panel/sticky-scroll-false.png"),
    ("https://cdn.zed.dev/fonts/Lilex-Regular.woff2", "../fonts/Lilex-Regular.woff2"),
    ("billing-support@zed.dev", "support@cuecode.dev"),
    ("dashboard.zed.dev", "cuecode.dev"),
    ("collab.zed.dev", "cuecode.dev"),
    ("connections to `zed.dev`", "connections to your model provider"),
    ("signing in to zed.dev", "configuring a model in Settings"),
    ("zed.dev/releases", "cuecode.dev/docs"),
    ("Download it at zed.dev", "Download it from cuecode.dev"),
    ("at zed.dev", "at cuecode.dev"),
    ("on zed.dev", "on cuecode.dev"),
    ("from zed.dev", "from cuecode.dev"),
    ("content on zed.dev", "CueCode docs"),
    ("Existing copy on zed.dev", "CueCode docs"),
    ("upload them to another location (e.g., zed.dev", "upload them to cuecode.dev/docs/assets"),
    ("requests to `zed.dev/docs`", "requests to `cuecode.dev/docs`"),
    ("https://images.zed.dev/docs/project-panel/auto-fold-dirs-true.png", "../assets/images/project-panel/auto-fold-dirs-true.png"),
    ("https://images.zed.dev/docs/project-panel/auto-fold-dirs-false.png", "../assets/images/project-panel/auto-fold-dirs-false.png"),
    ("https://images.zed.dev/docs/project-panel/compare-marked-files.png", "../assets/images/project-panel/compare-marked-files.png"),
    ("https://images.zed.dev/docs/project-panel/git-status.png", "../assets/images/project-panel/git-status.png"),
    ("https://docs.zed.dev/languages/go", "https://cuecode.dev/docs/languages/go"),
    ("https://docs.zed.dev/languages/rust", "https://cuecode.dev/docs/languages/rust"),
    ("https://docs.zed.dev/languages/svelte", "https://cuecode.dev/docs/languages/svelte"),
    ("https://docs.zed.dev/languages/typescript", "https://cuecode.dev/docs/languages/typescript"),
    ("https://docs.zed.dev/", "https://cuecode.dev/docs/"),
    ("https://docs.zed.dev", "https://cuecode.dev/docs"),
    ("https://cdn.zed.dev/fonts/iAWriterQuattroV.woff2", "../fonts/iAWriterQuattroV.woff2"),
    ("https://cdn.zed.dev/fonts/iAWriterQuattroV-Italic.woff2", "../fonts/iAWriterQuattroV-Italic.woff2"),
    ("https://cdn.zed.dev/fonts/IBMPlexSerif-Var.woff2", "../fonts/IBMPlexSerif-Var.woff2"),
    ("https://cdn.zed.dev/fonts/IBMPlexSerif-Var-Italic.woff2", "../fonts/IBMPlexSerif-Var.woff2"),
    ("https://cdn.zed.dev/images/noise.png", "{{ path_to_root }}assets/theme/noise.png"),
    ("$XDG_CONFIG_HOME/zed/debug.json", "$XDG_CONFIG_HOME/cuecode/debug.json"),
    ("$XDG_CONFIG_HOME/zed/", "$XDG_CONFIG_HOME/cuecode/"),
    ("~/.config/zed/tasks.json", "~/.config/cuecode/tasks.json"),
    ("~/.config/zed/keymap.json", "~/.config/cuecode/keymap.json"),
    ("~/.config/zed/settings.json", "~/.config/cuecode/settings.json"),
    ("~/.config/zed/themes", "~/.config/cuecode/themes"),
    ("~/.config/zed/snippets", "~/.config/cuecode/snippets"),
    ("~/.config/zed/debug.json", "~/.config/cuecode/debug.json"),
    ("~/.config/zed", "~/.config/cuecode"),
    ("%USERPROFILE%\\AppData\\Roaming\\Zed\\", "%USERPROFILE%\\AppData\\Roaming\\CueCode\\"),
    ("%APPDATA%\\Zed\\", "%APPDATA%\\CueCode\\"),
    ('"provider": "zed.dev"', '"provider": "ollama"'),
    ("hi@zed.dev", "support@cuecode.dev"),
    ("sales@zed.dev", "support@cuecode.dev"),
    ("Link to `zed.dev` pages when appropriate", "Link to `cuecode.dev/docs` pages when appropriate"),
    ("`zed.dev` marketing link", "`cuecode.dev` docs link"),
    ("relevant `zed.dev` marketing link", "relevant `cuecode.dev/docs` link"),
]

KEEP_SUBSTRINGS = [
    "zed-industries",
    "github.com/zed-industries",
    "fork of Zed",
    "forked from Zed",
    "Based on Zed",
    "upstream Zed",
    ".zed/",
    "zed://",
    "ZED_WORKSPACE",
    "ZED_BUNDLE_TYPE",
    "ZED_COMMIT_SHA",
    "ZED_RELEASE_CHANNEL",
    "zed_explorer_command_injector",
    "zed.metainfo",
    "zed.desktop.in",
    "Repology",
]


def should_skip_line(line: str) -> bool:
    return any(k in line for k in KEEP_SUBSTRINGS)


def fix_file(path: Path) -> bool:
    text = path.read_text(encoding="utf-8")
    new = text
    for old, repl in REPLACEMENTS:
        new = new.replace(old, repl)
    if path.suffix == ".md" and not should_skip_line(new):
        new = re.sub(r"https://zed\.dev(?!/)", "https://cuecode.dev", new)
        new = re.sub(r"https://zed\.dev/", "https://cuecode.dev/docs/", new)
    if new != text:
        path.write_text(new, encoding="utf-8")
        return True
    return False


def main() -> None:
    exts = {".md", ".hbs", ".css", ".js", ".toml", ".rules"}
    n = 0
    for path in sorted(DOCS.rglob("*")):
        if path.is_file() and path.suffix in exts:
            if fix_file(path):
                n += 1
                print(path.relative_to(ROOT))
    print(f"third pass updated {n} files")


if __name__ == "__main__":
    main()
