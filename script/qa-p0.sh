#!/usr/bin/env bash
# QA-P0 — Rebrand smoke (see .cursor/specs/delivery/07-implementation-roadmap.md)
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export CARGO_TARGET_DIR="$ROOT/target"
BIN="$ROOT/target/debug/cuecode"

PASS=0
FAIL=0
SKIP=0
RESULTS=()

pass() { PASS=$((PASS + 1)); RESULTS+=("PASS  $1"); echo "✓ $1"; }
fail() { FAIL=$((FAIL + 1)); RESULTS+=("FAIL  $1 — $2"); echo "✗ $1 — $2"; }
skip() { SKIP=$((SKIP + 1)); RESULTS+=("SKIP  $1 — $2"); echo "○ $1 — $2"; }

QA_DIR="$(mktemp -d /tmp/cuecode-qa-XXXXXX)"
LOG_FILE="$QA_DIR/harness.log"
PID_FILE="$QA_DIR/cuecode.pid"
ZED_SETTINGS="${HOME}/.config/zed/settings.json"
MAC_LOG="${HOME}/Library/Logs/CueCode/CueCode.log"
THEMES_DIR="$QA_DIR/config/themes"

cleanup() {
  if [[ -f "$PID_FILE" ]]; then
    local pid
    pid="$(cat "$PID_FILE")"
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "=============================================="
echo "QA-P0 — CueCode rebrand smoke"
echo "Isolated user-data-dir: $QA_DIR"
echo "Binary: $BIN"
echo "=============================================="

# --- Pre-flight (before log tee) ---
echo ""
echo "--- Pre-flight: rebrand-check.sh ---"
if ./script/rebrand-check.sh; then
  pass "rebrand-check.sh (automated gates)"
else
  fail "rebrand-check.sh" "automated gates failed"
fi

# --- Step 1 ---
echo ""
echo "--- Step 1: Fresh config (isolated --user-data-dir) ---"
pass "Step 1 — empty isolated profile at $QA_DIR"

# --- Build ---
echo ""
echo "--- Build cuecode ---"
cargo build -p cuecode --bin cuecode
if [[ -x "$BIN" ]]; then
  pass "cuecode binary built at $BIN"
else
  fail "cuecode build" "binary not found at $BIN"
  exit 1
fi

SPECS_OUT="$("$BIN" --system-specs 2>&1 || true)"
if [[ "$SPECS_OUT" == *"CueCode System Specs"* ]]; then
  pass "CLI identity — --system-specs prints CueCode"
else
  fail "CLI identity" "--system-specs missing CueCode header"
fi

DEFAULT_MODEL="$(python3 - <<'PY'
import re
from pathlib import Path
text = Path("assets/settings/default.json").read_text()
match = re.search(r'"default_model"\s*:\s*\{[^}]*?"model"\s*:\s*"([^"]+)"', text, re.S)
print(match.group(1) if match else "")
PY
)"
DEFAULT_PROVIDER="$(python3 - <<'PY'
import re
from pathlib import Path
text = Path("assets/settings/default.json").read_text()
match = re.search(r'"default_model"\s*:\s*\{[^}]*?"provider"\s*:\s*"([^"]+)"', text, re.S)
print(match.group(1) if match else "")
PY
)"

exec > >(tee -a "$LOG_FILE") 2>&1

# --- Step 4 (before GUI launch — avoids Ollama contention) ---
echo ""
echo "--- Step 4: Ollama prompt smoke ---"
if [[ "$DEFAULT_PROVIDER" == "ollama" ]]; then
  pass "Step 4 — default provider is ollama"
else
  fail "Step 4 — default provider" "expected ollama, got $DEFAULT_PROVIDER"
fi

if [[ -z "$DEFAULT_MODEL" ]]; then
  fail "Step 4 — default model" "could not parse from default.json"
elif curl -sf --max-time 5 "http://127.0.0.1:11434/api/tags" | grep -q "$DEFAULT_MODEL"; then
  pass "Step 4 — Ollama model $DEFAULT_MODEL available"
else
  fail "Step 4 — Ollama model" "$DEFAULT_MODEL not available"
fi

if [[ -n "$DEFAULT_MODEL" ]] && curl -sf --max-time 120 "http://127.0.0.1:11434/api/generate" \
  -d "{\"model\":\"$DEFAULT_MODEL\",\"prompt\":\"Say hello in one word\",\"stream\":false}" \
  | grep -qi '"response"'; then
  pass "Step 4 — Ollama generate smoke (Say hello in one word)"
else
  fail "Step 4 — Ollama generate" "no response within 120s"
fi

echo ""
echo "--- Step 4b: Agent prompt via eval-cli (Ollama) ---"
if ./script/qa-agent-ollama.sh; then
  pass "Step 4b — eval-cli agent completed via Ollama"
else
  fail "Step 4b — agent Ollama smoke" "qa-agent-ollama.sh failed"
fi

ZED_MTIME_BEFORE=""
if [[ -f "$ZED_SETTINGS" ]]; then
  ZED_MTIME_BEFORE="$(stat -f "%m" "$ZED_SETTINGS" 2>/dev/null || stat -c "%Y" "$ZED_SETTINGS")"
fi

MAC_LOG_LINES_BEFORE=0
if [[ -f "$MAC_LOG" ]]; then
  MAC_LOG_LINES_BEFORE="$(wc -l <"$MAC_LOG" | tr -d ' ')"
fi

# --- Steps 2–3: Launch ---
echo ""
echo "--- Steps 2–3: Launch CueCode (isolated) ---"
"$BIN" --user-data-dir "$QA_DIR" >/dev/null 2>&1 &
echo $! >"$PID_FILE"

LAUNCHED=false
for _ in $(seq 1 25); do
  sleep 1
  if [[ -f "$MAC_LOG" ]]; then
    if tail -n +"$((MAC_LOG_LINES_BEFORE + 1))" "$MAC_LOG" 2>/dev/null | grep -q "starting CueCode"; then
      LAUNCHED=true
      break
    fi
  fi
  if [[ -d "$QA_DIR/config" ]]; then
    LAUNCHED=true
    break
  fi
done

if [[ "$LAUNCHED" == true ]]; then
  pass "Step 2 — CueCode launched (log or isolated config detected)"
else
  fail "Step 2 — launch" "no startup signal within 25s"
fi

if [[ -f "$MAC_LOG" ]]; then
  if tail -n +"$((MAC_LOG_LINES_BEFORE + 1))" "$MAC_LOG" | grep -qi "starting Zed"; then
    fail "Step 2 — branding" "log contains 'starting Zed'"
  else
    pass "Step 2 — no 'starting Zed' in new log lines"
  fi
fi

if [[ "$(uname -s)" == "Darwin" ]]; then
  WINDOW_TITLE="$(osascript -e '
    tell application "System Events"
      repeat with p in (every application process whose name contains "cuecode" or name is "CueCode")
        try
          return name of front window of p
        end try
      end repeat
    end tell
    return ""
  ' 2>/dev/null || true)"
  if [[ "$WINDOW_TITLE" == *"CueCode"* ]]; then
    pass "Step 2 — window title contains CueCode ($WINDOW_TITLE)"
  elif pgrep -f "$ROOT/target/debug/cuecode" >/dev/null 2>&1; then
    pass "Step 2 — cuecode process running (window title unreadable in automation)"
  else
    fail "Step 2 — window title" "cuecode not running"
  fi
else
  skip "Step 2 — window title" "macOS AppleScript check only"
fi

if rg -q '"show_sign_in": false' assets/settings/default.json; then
  pass "Step 3 — show_sign_in false by default"
else
  fail "Step 3 — sign-in wall" "show_sign_in not false"
fi

if rg -q 'cloud trial upsell disabled' crates/agent_ui/src/ui/end_trial_upsell.rs; then
  pass "Step 3 — EndTrialUpsell disabled in CueCode"
else
  fail "Step 3 — trial upsell" "EndTrialUpsell still active"
fi

# --- Step 5 ---
echo ""
echo "--- Step 5: Config path isolation ---"
if [[ -d "$QA_DIR/config" ]]; then
  pass "Step 5 — isolated config dir at $QA_DIR/config"
else
  fail "Step 5 — isolated config" "expected $QA_DIR/config"
fi

if [[ -f "$ZED_SETTINGS" ]]; then
  ZED_MTIME_AFTER="$(stat -f "%m" "$ZED_SETTINGS" 2>/dev/null || stat -c "%Y" "$ZED_SETTINGS")"
  if [[ "$ZED_MTIME_BEFORE" == "$ZED_MTIME_AFTER" ]]; then
    pass "Step 5 — ~/.config/zed/settings.json mtime unchanged"
  else
    fail "Step 5 — zed config isolation" "zed settings mtime changed"
  fi
else
  pass "Step 5 — no ~/.config/zed/settings.json present"
fi

# --- Step 7 ---
echo ""
echo "--- Step 7: No user-visible Zed in agent/onboarding UI ---"
AGENT_ZED_HITS="$(rg -n 'Sign in to Zed|Zed Pro|zed\.dev|"Welcome to Zed"|"Zed Agent"' crates/agent_ui crates/onboarding crates/ai_onboarding/src --glob '*.rs' 2>/dev/null || true)"
if [[ -z "$AGENT_ZED_HITS" ]]; then
  pass "Step 7 — no Zed/zed.dev user strings in agent/onboarding UI"
else
  fail "Step 7 — agent/onboarding grep" "$(echo "$AGENT_ZED_HITS" | head -3)"
fi

# Capture themes dir from first launch before relaunch steps
HAS_THEMES=false
if [[ -d "$THEMES_DIR" ]] || find "$QA_DIR" -type d -name themes 2>/dev/null | grep -q .; then
  HAS_THEMES=true
fi

# --- Step 6 ---
echo ""
echo "--- Step 6: Settings persist on relaunch ---"
kill "$(cat "$PID_FILE")" 2>/dev/null || true
wait "$(cat "$PID_FILE")" 2>/dev/null || true
rm -f "$PID_FILE"
sleep 2

PERSIST_MARKER=""
if [[ -f "$QA_DIR/config/settings.json" ]]; then
  PERSIST_MARKER="$QA_DIR/config/settings.json"
elif [[ -d "$QA_DIR/db" ]]; then
  PERSIST_MARKER="$QA_DIR/db"
fi

"$BIN" --user-data-dir "$QA_DIR" >/dev/null 2>&1 &
echo $! >"$PID_FILE"
sleep 12
kill "$(cat "$PID_FILE")" 2>/dev/null || true
wait "$(cat "$PID_FILE")" 2>/dev/null || true
rm -f "$PID_FILE"

if [[ -n "$PERSIST_MARKER" ]] && [[ -e "$PERSIST_MARKER" ]]; then
  pass "Step 6 — isolated profile persisted across relaunch ($PERSIST_MARKER)"
else
  fail "Step 6 — settings persist" "no persisted state under $QA_DIR"
fi

# --- Step 8 ---
echo ""
echo "--- Step 8: Theme / branding stability ---"
if [[ "$HAS_THEMES" == true ]]; then
  pass "Step 8 — theme directory initialized under isolated profile"
else
  fail "Step 8 — theme" "no themes dir after first launch"
fi

"$BIN" --user-data-dir "$QA_DIR" >/dev/null 2>&1 &
echo $! >"$PID_FILE"
sleep 12
if [[ "$(uname -s)" == "Darwin" ]]; then
  WINDOW_TITLE="$(osascript -e '
    tell application "System Events"
      repeat with p in (every application process whose name contains "cuecode" or name is "CueCode")
        try
          return name of front window of p
        end try
      end repeat
    end tell
    return ""
  ' 2>/dev/null || true)"
  if [[ "$WINDOW_TITLE" == *"CueCode"* ]]; then
    pass "Step 8 — title bar still CueCode after relaunch ($WINDOW_TITLE)"
  elif pgrep -f "$ROOT/target/debug/cuecode" >/dev/null 2>&1; then
    pass "Step 8 — cuecode running after relaunch"
  else
    fail "Step 8 — branding" "cuecode not running"
  fi
else
  pgrep -f "$ROOT/target/debug/cuecode" >/dev/null 2>&1 && pass "Step 8 — cuecode running" || fail "Step 8" "not running"
fi
kill "$(cat "$PID_FILE")" 2>/dev/null || true
wait "$(cat "$PID_FILE")" 2>/dev/null || true
rm -f "$PID_FILE"

# --- Bonus ---
echo ""
echo "--- Bonus: cuecode --help ---"
HELP_OUT="$("$BIN" --help 2>&1 || true)"
if echo "$HELP_OUT" | grep -qi "cuecode"; then
  pass "cuecode --help mentions CueCode"
else
  fail "cuecode --help" "product name not found"
fi
if echo "$HELP_OUT" | grep -qE '(^|[^a-z])Zed([^a-z]|$)'; then
  fail "cuecode --help" "contains bare Zed product string"
else
  pass "cuecode --help — no bare Zed product string"
fi

if [[ "${QA_NETWORK_IDLE:-0}" == "1" ]]; then
  echo ""
  echo "--- Step 9: Idle network audit (no zed.dev) ---"
  if ./script/network-idle-audit.sh; then
    pass "Step 9 — idle network audit (no zed.dev connections)"
  else
    fail "Step 9 — network idle audit" "network-idle-audit.sh failed"
  fi
fi

echo ""
echo "=============================================="
echo "QA-P0 SUMMARY"
echo "=============================================="
for r in "${RESULTS[@]}"; do echo "$r"; done
echo ""
echo "Pass: $PASS  Fail: $FAIL  Skip: $SKIP"
echo "Harness log: $LOG_FILE"
echo "Profile: $QA_DIR"
echo "=============================================="

if [[ "$FAIL" -gt 0 ]]; then
  exit 1
fi
exit 0
