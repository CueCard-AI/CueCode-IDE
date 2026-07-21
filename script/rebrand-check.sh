#!/usr/bin/env bash
# Phase 0 + L1/L2 rebrand regression checks — see .cursor/specs/core/03-fork-and-rebrand.md
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

failures=0
SEARCH_TOOL="grep"

# Prefer ripgrep when available (CI), fall back to grep (macOS default).
if command -v rg >/dev/null 2>&1; then
  SEARCH_TOOL="rg"
fi

search_quiet() {
  local pattern="$1"
  shift
  if [[ "$SEARCH_TOOL" == "rg" ]]; then
    rg -q -- "$pattern" "$@"
  else
    grep -qE -- "$pattern" "$@"
  fi
}

search_lines() {
  local pattern="$1"
  shift
  if [[ "$SEARCH_TOOL" == "rg" ]]; then
    rg -n -- "$pattern" "$@"
  else
    grep -nE -- "$pattern" "$@"
  fi
}

check() {
  local desc="$1"
  shift
  if "$@"; then
    echo "✓ $desc"
  else
    echo "✗ $desc"
    failures=$((failures + 1))
  fi
}

check_absent() {
  local desc="$1"
  local pattern="$2"
  shift 2
  if search_quiet "$pattern" "$@"; then
    echo "✗ $desc"
    search_lines "$pattern" "$@" | head -5 || true
    failures=$((failures + 1))
  else
    echo "✓ $desc"
  fi
}

echo "CueCode rebrand check (Phase 0.3 + L1/L2 + Tier 2 strings + L3.1–L3.4 + L4 + Pass A–D)"
echo "Search tool: $SEARCH_TOOL"
echo "================================="

check "APP_NAME is CueCode" \
  search_quiet 'APP_NAME.*CueCode' crates/paths/src/paths.rs

check 'Binary name is cuecode' \
  search_quiet 'name = "cuecode"' crates/cuecode/Cargo.toml

check 'Compile-time binary assert present' \
  search_quiet 'CARGO_BIN_NAME' crates/cuecode/src/main.rs

check 'Release channel uses CueCode app IDs' \
  search_quiet 'dev\.cuecode\.CueCode' crates/release_channel/src/lib.rs

check 'Main CLI clap name is cuecode' \
  search_quiet 'name = "cuecode"' crates/cuecode/src/main.rs

check 'CLI helper clap name is cuecode' \
  search_quiet 'name = "cuecode"' crates/cli/src/main.rs

check 'Default agent model uses ollama' \
  bash -c 'grep -A5 "\"default_model\"" assets/settings/default.json | grep -q "\"provider\": \"ollama\""'

check 'Bundle metadata uses dev.cuecode identifiers' \
  search_quiet 'dev\.cuecode\.CueCode' crates/cuecode/Cargo.toml

check 'Linux bundle uses CueCode desktop name' \
  search_quiet 'APP_NAME="CueCode"' script/bundle-linux

check 'macOS bundle DMG volname is CueCode' \
  search_quiet 'volname CueCode' script/bundle-mac

check 'Windows installer product name is CueCode' \
  search_quiet 'appName = "CueCode"' script/bundle-windows.ps1

echo ""
echo "Tier 4 — packaging metadata gates:"

check 'Snap app command is cuecode' \
  search_quiet 'command: usr/bin/cuecode' crates/cuecode/resources/snap/snapcraft.yaml.in

check 'Snap common-id is dev.cuecode.CueCode' \
  search_quiet 'common-id: dev.cuecode.CueCode' crates/cuecode/resources/snap/snapcraft.yaml.in

check_absent 'No dev.zed.Zed in snap manifest' 'dev\.zed\.Zed' crates/cuecode/resources/snap

check 'Flatpak manifest command is cuecode' \
  search_quiet '"command": "cuecode"' crates/cuecode/resources/flatpak/manifest-template.json

check 'Flatpak installs cuecode editor binary' \
  search_quiet 'libexec/cuecode-editor /app/libexec/cuecode-editor' crates/cuecode/resources/flatpak/manifest-template.json

check_absent 'No zed.dev in Flatpak metainfo' 'zed\.dev' crates/cuecode/resources/flatpak

check 'Desktop entry registers cuecode URL scheme' \
  search_quiet 'x-scheme-handler/cuecode' crates/cuecode/resources/zed.desktop.in

check 'install_cli symlinks cuecode' \
  search_quiet '/usr/local/bin/cuecode' crates/install_cli/src/install_cli_binary.rs

check 'Native agent id uses APP_NAME' \
  search_quiet 'format!\("\{\} Agent", paths::APP_NAME\)' crates/agent/src/agent.rs

echo ""
echo "Tier 1 — user-visible product strings:"

TIER1_UI_CRATES=(
  crates/system_specs
  crates/recent_projects/src/remote_servers.rs
  crates/workspace/src/workspace.rs
  crates/workspace/src/pane_group.rs
  crates/sidebar/src/sidebar.rs
  crates/oauth_callback_server
  crates/node_runtime
  crates/ui/src/components/collab/update_button.rs
  crates/feedback
  crates/collab_ui
  crates/auto_update
  crates/util/src/util.rs
  crates/explorer_command_injector/src
)

check_absent 'No bare Zed product strings in Tier 1 UI crates' '"[^"]*\bZed\b[^"]*"' "${TIER1_UI_CRATES[@]}"

check 'System specs version line uses APP_NAME' \
  search_quiet '\{APP_NAME\}: v' crates/system_specs/src/system_specs.rs

check 'Default icon theme is CueCode (Default)' \
  search_quiet 'DEFAULT_ICON_THEME_NAME.*CueCode \(Default\)' crates/theme/src/icon_theme.rs

check 'Default settings icon_theme is CueCode' \
  search_quiet '"icon_theme": "CueCode \(Default\)"' assets/settings/default.json

check 'Feedback links to CueCode-IDE repo' \
  search_quiet 'github.com/CueCard-AI/CueCode-IDE' crates/feedback/src/feedback.rs

check_absent 'No zed-industries URLs in feedback' 'zed-industries' crates/feedback

check 'Badge message is CueCode' \
  search_quiet '"message": "CueCode"' assets/badge/v0.json

echo ""
echo "Tier 2 — user-visible string gates:"

TIER2_CRATES=(
  crates/agent_ui
  crates/onboarding
  crates/workspace/src/welcome.rs
  crates/workspace/src/notifications.rs
  crates/settings_ui
  crates/edit_prediction_ui
  crates/extensions_ui
  crates/language_models/src/provider
)

check_absent 'No "Zed Agent" in Tier 2 surfaces' '"Zed Agent"' "${TIER2_CRATES[@]}"
check_absent 'No "Welcome to Zed" in Tier 2 surfaces' 'Welcome to Zed' "${TIER2_CRATES[@]}"
check_absent 'No "Zed Pro" upsell strings in Tier 2 surfaces' 'Zed Pro' "${TIER2_CRATES[@]}"
check_absent 'No "Zed will retry" in agent UI' 'Zed will retry' crates/agent_ui

check_absent 'No bare Zed in settings_ui page_data' '\bZed\b' crates/settings_ui/src/page_data.rs

check_absent 'No zed CLI examples in settings_ui' '`zed |zed <' crates/settings_ui

check 'Settings CLI copy uses cuecode' \
  search_quiet '`cuecode <path>`' crates/settings_ui/src/page_data.rs

check_absent 'No Zed CLI in install_cli' 'Zed CLI' crates/install_cli

TIER2_STRING_CRATES=(
  crates/settings_ui
  crates/onboarding
  crates/ai_onboarding
  crates/title_bar
  crates/agent_ui
  crates/edit_prediction_ui
  crates/extensions_ui
  crates/language_models/src/provider
  crates/theme_selector
  crates/language_onboarding
  crates/language_tools
  crates/repl
  crates/project/src/project.rs
  crates/recent_projects
  crates/workspace/src/notifications.rs
  crates/collab_ui
  crates/debugger_ui
  crates/copilot_ui
  crates/context_server
  crates/sandbox
  crates/editor/src/editor.rs
)

check_absent 'No zed.dev in Tier 2 user surfaces' 'zed\.dev' "${TIER2_STRING_CRATES[@]}"

check_absent 'No "restart Zed" in Tier 2 surfaces' 'restart Zed' "${TIER2_STRING_CRATES[@]}"

check_absent 'No "in Zed" product copy in Tier 2 surfaces' ' in Zed' "${TIER2_STRING_CRATES[@]}"

ONBOARDING_CRATES=(
  crates/onboarding
  crates/ai_onboarding
)

check_absent 'No zed.dev in onboarding surfaces' 'zed\.dev' "${ONBOARDING_CRATES[@]}"

echo ""
echo "Tier 2b — settings schema docs:"

check_absent 'No Zed product phrases in settings_content' \
  ' using Zed|opening Zed|When opening Zed|languages you'\''re using Zed|The URL of the Zed' \
  crates/settings_content/src

echo ""
echo "Pass A — settings and keymap comments:"

check 'No bare Zed in default.json except compat path comments' \
  bash -c '
    if command -v rg >/dev/null 2>&1; then
      hits=$(rg -n "\bZed\b" assets/settings/default.json 2>/dev/null || true)
    else
      hits=$(grep -nE "\bZed\b" assets/settings/default.json 2>/dev/null || true)
    fi
    bad=$(printf "%s\n" "$hits" | grep -v "legacy Zed project settings" | grep -v "legacy Zed user settings" | grep -v "{cuecode,CueCode,zed,Zed}" || true)
    [[ -z "$bad" ]]
  '

check_absent 'No bare Zed in keymap comments' '\bZed\b' assets/keymaps/

echo ""
echo "Pass B — remaining user-visible strings:"

PASS_B_SURFACES=(
  crates/explorer_command_injector
  crates/extension_cli
  crates/sandbox/src/linux_bubblewrap.rs
  crates/editor/src/editor.rs
  crates/debugger_ui
  crates/dap/src/transport.rs
  crates/copilot_ui
  crates/context_server/src
  crates/extension_host/src/wasm_host
  crates/inspector_ui
  crates/git/src/repository.rs
  crates/recent_projects/src/recent_projects.rs
  crates/ui/src/components/list/list_bullet_item.rs
  crates/ui/src/components/collab/collab_notification.rs
  crates/ui/src/components/image.rs
  crates/ui/src/components/button/icon_button.rs
  crates/agent/src/templates
  crates/cuecode/src/reliability.rs
  crates/repl/src/repl_sessions_ui.rs
)

check_absent 'No bare Zed product strings in Pass B surfaces' '"[^"]*\bZed\b[^"]*"' "${PASS_B_SURFACES[@]}"

check 'Extension CLI User-Agent uses CueCode' \
  search_quiet 'CueCode Extension CLI/' crates/extension_cli/src/main.rs

check_absent 'No Zed Extension CLI User-Agent' 'Zed Extension CLI/' crates/extension_cli crates/extension_host/benches

check_absent 'No Open with Zed explorer fallback' 'Open with Zed' crates/explorer_command_injector

check_absent 'No Zed.exe in auto_update error contexts' 'Zed\.exe' crates/auto_update

echo ""
echo "Pass C — filename and artifact renames:"

check 'Linux auto-update uses cuecode.app layout' \
  search_quiet 'cuecode\{\}\.app' crates/auto_update/src/auto_update.rs

check 'Linux auto-update uses libexec/cuecode-editor' \
  search_quiet 'libexec/cuecode-editor' crates/auto_update/src/auto_update.rs

check_absent 'No zed{}.app as primary Linux install folder' \
  'let app_folder_name = format!("zed\{\}\.app"' crates/auto_update/src/auto_update.rs

check 'Auto-update requests cuecode release asset' \
  search_quiet '"cuecode"' crates/auto_update/src/auto_update.rs

check 'Auto-update requests cuecode-remote-server asset' \
  search_quiet 'cuecode-remote-server' crates/auto_update/src/auto_update.rs

check 'bundle-linux ships cuecode.app and cuecode-editor' \
  bash -c 'grep -q "cuecode\$suffix.app" script/bundle-linux && grep -q libexec/cuecode-editor script/bundle-linux'

check 'bundle-linux produces cuecode-remote-server artifact' \
  search_quiet 'cuecode-remote-server-linux' script/bundle-linux

check 'Windows updater moves bin/cuecode CLI' \
  search_quiet 'install\\\\bin\\\\cuecode' crates/auto_update_helper/src/updater.rs

echo ""
echo "Pass D — docs and legal:"

check 'configuring-cuecode.md exists (configuring-zed.md removed)' \
  bash -c '[[ -f docs/src/configuring-cuecode.md ]] && [[ ! -f docs/src/configuring-zed.md ]]'

check 'SUMMARY links configuring-cuecode.md' \
  search_quiet 'configuring-cuecode\.md' docs/src/SUMMARY.md

check 'No bare Zed in docs except intentional upstream citations' \
  bash -c '
    if command -v rg >/dev/null 2>&1; then
      hits=$(rg -n "\bZed\b" docs/ 2>/dev/null || true)
    else
      hits=$(grep -rnE "\bZed\b" docs/ 2>/dev/null || true)
    fi
    bad=$(printf "%s\n" "$hits" | grep -v "upstream Zed staff only" || true)
    [[ -z "$bad" ]]
  '

check 'Legal docs include fork notice' \
  search_quiet 'Fork notice' legal/

check 'CONTRIBUTING identifies CueCode fork' \
  search_quiet 'fork of Zed' CONTRIBUTING.md

check 'book.toml redirects legacy configuring-zed.html' \
  search_quiet '"/configuring-zed\.html"' docs/book.toml

echo ""
echo "L3.1 — cuecode_actions crate rename:"

check 'cuecode_actions in workspace members' \
  search_quiet 'crates/cuecode_actions' Cargo.toml

check 'workspace dependency points at cuecode_actions' \
  search_quiet 'cuecode_actions = \{ path = "crates/cuecode_actions" \}' Cargo.toml

check_absent 'zed_actions removed from workspace members' 'crates/zed_actions' Cargo.toml

check_absent 'No zed_actions dependency keys in crate manifests' 'zed_actions\.workspace' crates

echo ""
echo "L3.2 — cuecode app crate rename:"

check 'cuecode crate in workspace members' \
  search_quiet 'crates/cuecode' Cargo.toml

check 'default workspace member is cuecode' \
  search_quiet 'default-members = \["crates/cuecode"\]' Cargo.toml

check 'workspace dependency points at cuecode crate' \
  search_quiet 'cuecode = \{ path = "crates/cuecode" \}' Cargo.toml

check 'cuecode package name in manifest' \
  search_quiet '^name = "cuecode"' crates/cuecode/Cargo.toml

check 'main binary uses app module not zed' \
  search_quiet '^mod app;' crates/cuecode/src/main.rs

check_absent 'crates/zed removed from workspace members' 'crates/zed"' Cargo.toml

check_absent 'No zed path dependency in workspace Cargo.toml' 'path = "crates/zed"' Cargo.toml

echo ""
echo "L3.3 — action namespace and internal identifiers:"

check_absent 'No namespace = zed in cuecode_actions' 'namespace = zed' crates/cuecode_actions/src/lib.rs

check_absent 'No actions!(zed in crates' 'actions!\(\s*zed\s*,' crates

check_absent 'No RegisterZedScheme in cuecode app' 'RegisterZedScheme' crates/cuecode/src/app.rs

check_absent 'No OpenZedPredictOnboarding outside deprecated aliases' 'OpenZedPredictOnboarding' \
  crates/agent_ui crates/edit_prediction crates/edit_prediction_ui

check_absent 'No EmailZed outside deprecated aliases' 'EmailZed' \
  crates/feedback crates/cuecode/src/app

check 'Default keymaps use cuecode:: action namespace' \
  search_quiet 'cuecode::Quit' assets/keymaps/default-macos.json

check_absent 'No zed:: in default macOS keymap' 'zed::' assets/keymaps/default-macos.json

check_absent 'No zed:: in default Linux keymap' 'zed::' assets/keymaps/default-linux.json

check_absent 'No zed:: in default Windows keymap' 'zed::' assets/keymaps/default-windows.json

check 'Linux F10 opens CueCode app menu' \
  search_quiet 'OpenApplicationMenu", "CueCode"' assets/keymaps/default-linux.json

check 'release_channel reads CUECODE_RELEASE_CHANNEL' \
  search_quiet 'CUECODE_RELEASE_CHANNEL' crates/release_channel/src/lib.rs

check 'stateless mode reads CUECODE_STATELESS' \
  search_quiet 'CUECODE_STATELESS' crates/cuecode_env_vars/src/cuecode_env_vars.rs

echo ""
echo "L3.4 — satellite crates and HTTP identity:"

check 'cuecode_env_vars in workspace members' \
  search_quiet 'crates/cuecode_env_vars' Cargo.toml

check 'cuecode_credentials_provider in workspace members' \
  search_quiet 'crates/cuecode_credentials_provider' Cargo.toml

check_absent 'zed_env_vars removed from workspace members' 'crates/zed_env_vars"' Cargo.toml

check_absent 'zed_credentials_provider removed from workspace members' 'crates/zed_credentials_provider"' Cargo.toml

check 'client exposes cuecode_urls module' \
  search_quiet 'pub mod cuecode_urls' crates/client/src/client.rs

check 'User-Agent uses APP_NAME' \
  search_quiet 'paths::APP_NAME' crates/cuecode/src/main.rs

check 'Bundled builds read CUECODE_BUNDLE' \
  search_quiet 'CUECODE_BUNDLE' crates/language/build.rs

check 'compile_time bundle helper in paths crate' \
  search_quiet 'compile_time_bundle_is_set' crates/paths/src/paths.rs

echo ""
echo "L4 — cuecode:// URL scheme:"

check 'app_urls module defines CUECODE_URL_SCHEME' \
  search_quiet 'CUECODE_URL_SCHEME' crates/client/src/app_urls.rs

check 'Default settings schema uses cuecode://' \
  search_quiet '"cuecode://schemas/settings"' assets/settings/default.json

check 'Shared agent links use cuecode://' \
  search_quiet 'cuecode://agent/shared/' crates/client/src/cuecode_urls.rs

check 'OS registers cuecode URL scheme' \
  search_quiet 'register_app_url_schemes' crates/install_cli/src/register_zed_scheme.rs

check 'Open listener normalizes legacy zed:// links' \
  search_quiet 'normalize_incoming_url' crates/cuecode/src/app/open_listener.rs

check 'Skill share links emit cuecode://' \
  search_quiet 'SKILL_SHARE_LINK_SCHEME: &str = "cuecode"' crates/agent_skills/agent_skills.rs

echo ""
echo "L4.5 — legacy compat + project paths:"

check 'Project settings folder is .cuecode' \
  search_quiet '".cuecode"' crates/paths/src/paths.rs

check 'Legacy .zed project paths remain readable' \
  search_quiet 'legacy_local_settings_file_relative_path' crates/paths/src/paths.rs

check 'Edit prediction provider defaults to open_ai_compatible_api (BYOK)' \
  search_quiet '"provider": "open_ai_compatible_api"' assets/settings/default.json

check 'EditPredictionProvider uses CueCode variant' \
  search_quiet 'EditPredictionProvider::CueCode' crates/settings_content/src/language.rs

check 'OS registers cuecode scheme only' \
  search_quiet 'CUECODE_URL_SCHEME];' crates/client/src/app_urls.rs

check 'Legacy zed:// links still normalize in-app' \
  search_quiet 'normalize_incoming_url' crates/client/src/app_urls.rs

check 'Language model settings accept cuecode.dev key' \
  search_quiet 'rename = "cuecode.dev"' crates/settings_content/src/language_model.rs

check 'Extension WIT package remains zed:extension for compat' \
  search_quiet 'package zed:extension' crates/extension_api/wit

check 'Docs base URL uses cuecode.dev' \
  search_quiet 'cuecode.dev/docs' crates/release_channel/src/lib.rs

check 'Windows bundle uses cuecode resources' \
  search_quiet 'cuecode\\resources\\windows' script/bundle-windows.ps1

check 'Windows bundle builds cuecode package' \
  search_quiet '--package cuecode' script/bundle-windows.ps1

check 'Windows installer template is cuecode.iss' \
  search_quiet 'cuecode\.iss' script/bundle-windows.ps1

check_absent 'No zed.dev URLs in docs' 'zed\.dev' docs

check_absent 'No images.zed.dev URLs in docs' 'images\.zed\.dev' docs

check_absent 'No cdn.zed.dev URLs in docs' 'cdn\.zed\.dev' docs

check_absent 'No ~/.config/zed paths in docs src' '~/.config/zed' docs/src

echo ""
echo "Config isolation (CueCode vs Zed KV/data dir):"

if ./script/qa-config-isolation.sh; then
  echo "✓ qa-config-isolation.sh (static gates)"
else
  echo "✗ qa-config-isolation.sh"
  failures=$((failures + 1))
fi

echo ""
echo "Informational: zed.dev lines in default.json (comments OK):"
search_lines 'zed\.dev' assets/settings/default.json | head -5 || true

echo ""
if [[ $failures -eq 0 ]]; then
  echo "All rebrand checks passed."
  exit 0
else
  echo "$failures check(s) failed."
  exit 1
fi
