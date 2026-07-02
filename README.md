# CueCode

CueCode is a fork of [Zed](https://github.com/zed-industries/zed) — a spec-driven agent IDE with local/BYOK models by default, no zed.dev sign-in wall, and separate user data from Zed.

**Upstream:** Zed (GPL-3.0-or-later). See About in the app for full attribution.

---

## Specs & build plan

Product and engineering specs live in the workspace `.cursor/specs/` tree:

- [Spec index](../.cursor/specs/00-README.md)
- [Fork & rebrand](../.cursor/specs/core/03-fork-and-rebrand.md)
- [Master build plan](../.cursor/specs/delivery/build-plans/00-master-build-plan.md)
- [Implementation roadmap](../.cursor/specs/delivery/07-implementation-roadmap.md)

Phase 0 exit criteria and QA scripts are in [03-fork-and-rebrand §manual QA](../.cursor/specs/core/03-fork-and-rebrand.md#manual-qa).

---

## Developing CueCode

From this directory (`CueCode-IDE/`):

```bash
cargo run -p cuecode --bin cuecode
```

Or use the local helper:

```bash
./script/cuecode-local
```

Fresh config paths (no collision with Zed):

| Platform | Config / data |
|----------|----------------|
| macOS | `~/Library/Application Support/CueCode/` |
| Linux | `~/.config/cuecode/` |
| Windows | `%LOCALAPPDATA%\CueCode\` |

---

## CLI

```bash
cargo build -p cuecode --bin cuecode
./target/debug/cuecode --help    # product name should say CueCode
```

Install CLI to PATH (macOS, from the app): **Command Palette → install cli**.

---

## Packaging

| Platform | Script |
|----------|--------|
| macOS | `script/bundle-mac` |
| Linux | `script/bundle-linux` |
| Windows | `script/bundle-windows.ps1` |
| Flatpak | `script/flatpak/bundle-flatpak` |

Rebrand regression gate:

```bash
./script/rebrand-check.sh
```

Phase 0 smoke (automated QA-P0):

```bash
./script/qa-p0.sh
```

Runs with `--user-data-dir /tmp/cuecode-qa-*` so your real CueCode config is not modified.

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Upstream Zed contribution docs remain below for reference where the fork has not diverged.

---

## Licensing

CueCode is based on Zed and inherits GPL-3.0-or-later obligations. Third-party license compliance uses `script/generate-licenses` and `script/licenses/zed-licenses.toml` (upstream naming).
