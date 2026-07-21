#!/usr/bin/env bash
# Headless agent smoke — eval-cli + local Ollama (Linux CI + optional qa-p0 Step 4b).
# See .cursor/specs/core/03-zed-reference-cleanup-phases.md#progress
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"

OLLAMA_HOST="${OLLAMA_HOST:-http://127.0.0.1:11434}"
QA_REQUIRE_OLLAMA="${QA_REQUIRE_OLLAMA:-0}"
QA_OLLAMA_PULL="${QA_OLLAMA_PULL:-0}"
QA_AGENT_TIMEOUT="${QA_AGENT_TIMEOUT:-180}"
EVAL_BIN="$ROOT/target/debug/eval-cli"

usage() {
  cat <<'EOF'
Usage: ./script/qa-agent-ollama.sh

Runs eval-cli against a local Ollama model (NativeAgent pipeline, no GUI).

Environment:
  QA_OLLAMA_MODEL     Model tag (default: from assets/settings/default.json)
  QA_OLLAMA_PULL=1    Pull model via `ollama pull` when missing
  QA_REQUIRE_OLLAMA=1 Fail if Ollama is unreachable (CI)
  QA_AGENT_TIMEOUT    eval-cli --timeout seconds (default: 180)
  OLLAMA_HOST         Ollama API base URL (default: http://127.0.0.1:11434)
EOF
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

log() { echo "qa-agent-ollama: $*"; }
die() { log "ERROR: $*"; exit 1; }

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

MODEL="${QA_OLLAMA_MODEL:-$DEFAULT_MODEL}"
if [[ "$DEFAULT_PROVIDER" != "ollama" ]]; then
  die "default.json agent provider must be ollama (got: ${DEFAULT_PROVIDER:-<empty>})"
fi
if [[ -z "$MODEL" ]]; then
  die "could not parse default agent model from assets/settings/default.json"
fi

ollama_tags() {
  curl -sf --max-time 5 "${OLLAMA_HOST}/api/tags"
}

if ! ollama_tags >/dev/null 2>&1; then
  if [[ "$QA_REQUIRE_OLLAMA" == "1" ]]; then
    die "Ollama not reachable at ${OLLAMA_HOST} (QA_REQUIRE_OLLAMA=1)"
  fi
  log "Ollama not reachable — skip (set QA_REQUIRE_OLLAMA=1 to fail)"
  exit 0
fi

model_available() {
  ollama_tags | grep -q "\"name\"[[:space:]]*:[[:space:]]*\"${MODEL}\""
}

if ! model_available; then
  if [[ "$QA_OLLAMA_PULL" == "1" ]]; then
    log "pulling model ${MODEL} …"
    if ! command -v ollama >/dev/null 2>&1; then
      die "QA_OLLAMA_PULL=1 but ollama CLI not found"
    fi
    ollama pull "$MODEL"
  else
    die "model ${MODEL} not in Ollama tags (set QA_OLLAMA_PULL=1 to pull)"
  fi
fi

log "building eval-cli …"
cargo build -p eval_cli --bin eval-cli
[[ -x "$EVAL_BIN" ]] || die "eval-cli not found at $EVAL_BIN"

AGENT_QA_DIR="$(mktemp -d /tmp/cuecode-agent-qa-XXXXXX)"
WORKDIR="$AGENT_QA_DIR/workdir"
OUTPUT="$AGENT_QA_DIR/out"
mkdir -p "$WORKDIR" "$OUTPUT"
export XDG_CONFIG_HOME="$AGENT_QA_DIR/config"
export XDG_DATA_HOME="$AGENT_QA_DIR/data"

log "running eval-cli with ollama/${MODEL} (timeout ${QA_AGENT_TIMEOUT}s) …"
set +e
"$EVAL_BIN" \
  --workdir "$WORKDIR" \
  --model "ollama/${MODEL}" \
  --instruction "Reply with exactly the word: ok" \
  --timeout "$QA_AGENT_TIMEOUT" \
  --output-dir "$OUTPUT" \
  --no-staff \
  --thinking false
eval_exit=$?
set -e

RESULT_JSON="$OUTPUT/result.json"
if [[ ! -f "$RESULT_JSON" ]]; then
  die "eval-cli exited ${eval_exit} without result.json (see stderr above)"
fi

STATUS="$(python3 - <<PY
import json
from pathlib import Path
data = json.loads(Path("$RESULT_JSON").read_text())
print(data.get("status", ""))
PY
)"

if [[ "$eval_exit" -ne 0 || "$STATUS" != "completed" ]]; then
  log "result.json:"
  cat "$RESULT_JSON"
  die "agent smoke failed (exit=${eval_exit}, status=${STATUS})"
fi

log "PASS — eval-cli agent completed via ollama/${MODEL} (${STATUS})"
exit 0
