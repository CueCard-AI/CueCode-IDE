#!/usr/bin/env bash
# Config / KV isolation — CueCode must not read or write upstream Zed user data.
# See .cursor/specs/delivery/07-implementation-roadmap.md (Phase 0 cross-cutting).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

failures=0
SEARCH_TOOL="grep"
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
  local hits
  hits="$(search_lines "$pattern" "$@" 2>/dev/null || true)"
  if [[ -n "$hits" ]]; then
    echo "✗ $desc"
    echo "$hits" | head -8
    failures=$((failures + 1))
  else
    echo "✓ $desc"
  fi
}

echo "QA config isolation — CueCode vs Zed user data"
echo "=============================================="

check "APP_NAME is CueCode" \
  search_quiet 'APP_NAME.*CueCode' crates/paths/src/paths.rs

check "Config dir uses APP_NAME_LOWERCASE" \
  search_quiet 'join\(APP_NAME_LOWERCASE\)' crates/paths/src/paths.rs

check "Data dir uses APP_NAME" \
  search_quiet 'join\(APP_NAME\)' crates/paths/src/paths.rs

check "SQLite KV store lives under data_dir/db" \
  search_quiet 'DATABASE_DIR.*data_dir\(\)\.join\("db"\)' crates/paths/src/paths.rs

check "AppDatabase opens under paths::database_dir()" \
  search_quiet 'database_dir\(\)' crates/db/src/db.rs

check_absent "No .config/zed string literals in crate src" \
  '\.config/zed' \
  crates \
  -g '**/src/**'

check_absent "No Application Support/Zed string literals in crate src" \
  'Application Support/Zed' \
  crates \
  -g '**/src/**'

ZED_SETTINGS="${HOME}/.config/zed/settings.json"

if [[ -f "$ZED_SETTINGS" ]]; then
  ZED_MTIME="$(stat -f "%m" "$ZED_SETTINGS" 2>/dev/null || stat -c "%Y" "$ZED_SETTINGS")"
  echo ""
  echo "Side-by-side note: Zed config present ($ZED_SETTINGS, mtime=$ZED_MTIME)"
  echo "  After normal CueCode use, re-run — Zed mtime should be unchanged."
  echo "  Full isolated launch check: ./script/qa-p0.sh Step 5"
else
  echo ""
  echo "○ No ~/.config/zed/settings.json — side-by-side mtime probe skipped"
  echo "  Fresh-machine isolation: ./script/qa-p0.sh Step 5"
fi

echo ""
if [[ $failures -eq 0 ]]; then
  echo "All static config-isolation checks passed."
  exit 0
else
  echo "$failures static check(s) failed."
  exit 1
fi
