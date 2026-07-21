#!/usr/bin/env bash
# Idle network audit — ensure CueCode does not connect to zed.dev hosts on a fresh profile.
# See .cursor/specs/core/03-zed-reference-cleanup-phases.md#progress
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"

BIN="$ROOT/target/debug/cuecode"
QA_IDLE_SECONDS="${QA_IDLE_SECONDS:-90}"
QA_POLL_INTERVAL="${QA_POLL_INTERVAL:-5}"
QA_NETWORK_AUDIT_REQUIRED="${QA_NETWORK_AUDIT_REQUIRED:-0}"

# Host substrings that must not appear in established connections during idle.
FORBIDDEN_PATTERN='(\.zed\.dev|^zed\.dev|api-staging\.zed\.dev|llm-staging\.zed\.dev)'

usage() {
  cat <<'EOF'
Usage: ./script/network-idle-audit.sh

Launch CueCode on an isolated profile (no project opened), wait idle, and fail if
any TCP connection targets upstream Zed cloud hosts.

Environment:
  QA_IDLE_SECONDS            Seconds to observe after launch (default: 90)
  QA_POLL_INTERVAL           Poll interval in seconds (default: 5)
  QA_NETWORK_AUDIT_REQUIRED  Exit 1 if cuecode binary missing (CI)
  CARGO_TARGET_DIR           Cargo output dir (default: ./target)
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

log() { echo "network-idle-audit: $*"; }
die() { log "ERROR: $*"; exit 1; }

if ! command -v lsof >/dev/null 2>&1; then
  die "lsof is required"
fi

log "building cuecode …"
cargo build -p cuecode --bin cuecode
[[ -x "$BIN" ]] || die "cuecode binary not found at $BIN"

NET_QA_DIR="$(mktemp -d /tmp/cuecode-net-qa-XXXXXX)"
PID_FILE="$NET_QA_DIR/cuecode.pid"
HITS_FILE="$NET_QA_DIR/forbidden-hits.log"
: >"$HITS_FILE"

cleanup() {
  if [[ -f "$PID_FILE" ]]; then
    local pid
    pid="$(cat "$PID_FILE")"
    kill "$pid" 2>/dev/null || true
    wait "$pid" 2>/dev/null || true
    rm -f "$PID_FILE"
  fi
}
trap cleanup EXIT

scan_forbidden_connections() {
  local pid="$1"
  lsof -nP -p "$pid" -iTCP 2>/dev/null | grep -E "$FORBIDDEN_PATTERN" || true
}

launch_cuecode() {
  local launcher=("$BIN" --user-data-dir "$NET_QA_DIR")
  if [[ -z "${DISPLAY:-}" ]] && command -v xvfb-run >/dev/null 2>&1; then
    log "no DISPLAY — launching under xvfb-run"
    xvfb-run -a "${launcher[@]}" >/dev/null 2>&1 &
  else
    "${launcher[@]}" >/dev/null 2>&1 &
  fi
  echo $! >"$PID_FILE"
}

log "isolated profile: $NET_QA_DIR"
log "idle observation: ${QA_IDLE_SECONDS}s (poll every ${QA_POLL_INTERVAL}s)"

launch_cuecode
PID="$(cat "$PID_FILE")"

LAUNCHED=false
for _ in $(seq 1 30); do
  if kill -0 "$PID" 2>/dev/null; then
    if [[ -d "$NET_QA_DIR/config" ]] || [[ -d "$NET_QA_DIR/data" ]]; then
      LAUNCHED=true
      break
    fi
  else
    die "cuecode exited before idle observation (pid $PID)"
  fi
  sleep 1
done

if [[ "$LAUNCHED" != true ]]; then
  if kill -0 "$PID" 2>/dev/null; then
    LAUNCHED=true
    log "process running; continuing idle watch"
  else
    die "cuecode failed to start"
  fi
fi

elapsed=0
while [[ "$elapsed" -lt "$QA_IDLE_SECONDS" ]]; do
  if ! kill -0 "$PID" 2>/dev/null; then
    log "cuecode exited during idle window (after ${elapsed}s)"
    break
  fi
  hits="$(scan_forbidden_connections "$PID")"
  if [[ -n "$hits" ]]; then
    {
      echo "=== t=${elapsed}s ==="
      echo "$hits"
    } >>"$HITS_FILE"
  fi
  sleep "$QA_POLL_INTERVAL"
  elapsed=$((elapsed + QA_POLL_INTERVAL))
done

if [[ -s "$HITS_FILE" ]]; then
  log "FORBIDDEN zed.dev connections detected:"
  cat "$HITS_FILE"
  die "idle network audit failed — see hits above"
fi

log "PASS — no zed.dev TCP connections during ${QA_IDLE_SECONDS}s idle (pid $PID)"
exit 0
