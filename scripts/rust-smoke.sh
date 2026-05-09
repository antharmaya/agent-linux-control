#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
SOCKET="${TMPDIR:-/tmp}/agent-linux-control-rust-smoke.sock"
LOG="${TMPDIR:-/tmp}/agent-linux-control-rust-smoke.log"
PING_OUT="${TMPDIR:-/tmp}/agent-linux-control-rust-ping.json"
MANIFEST_OUT="${TMPDIR:-/tmp}/agent-linux-control-rust-manifest.json"
INPUT_OUT="${TMPDIR:-/tmp}/agent-linux-control-rust-input.json"

rm -f "$SOCKET" "$LOG" "$PING_OUT" "$MANIFEST_OUT" "$INPUT_OUT"

cargo test --manifest-path "$ROOT/Cargo.toml"

AGENT_LINUX_CONTROL_DEVICE_DELAY="${AGENT_LINUX_CONTROL_DEVICE_DELAY:-0.05}"
export AGENT_LINUX_CONTROL_DEVICE_DELAY
cargo run --manifest-path "$ROOT/Cargo.toml" -q -p alc-daemon -- serve --socket "$SOCKET" >"$LOG" 2>&1 &
PID="$!"

cleanup() {
  kill "$PID" 2>/dev/null || true
  wait "$PID" 2>/dev/null || true
  rm -f "$SOCKET"
}
trap cleanup EXIT INT TERM

i=0
while [ "$i" -lt 50 ]; do
  [ -S "$SOCKET" ] && break
  i=$((i + 1))
  sleep 0.1
done

if [ ! -S "$SOCKET" ]; then
  cat "$LOG" >&2
  exit 1
fi

cargo run --manifest-path "$ROOT/Cargo.toml" -q -p alc-daemon -- call --socket "$SOCKET" '{"id":"p1","cmd":"ping"}' >"$PING_OUT"
cargo run --manifest-path "$ROOT/Cargo.toml" -q -p alc-daemon -- call --socket "$SOCKET" '{"id":"m1","cmd":"manifest"}' >"$MANIFEST_OUT"
if [ -w /dev/uinput ]; then
  cargo run --manifest-path "$ROOT/Cargo.toml" -q -p alc-daemon -- call --socket "$SOCKET" '{"id":"i1","cmd":"input","steps":[{"action":"move","dx":1,"dy":1}]}' >"$INPUT_OUT"
fi

python3 -m json.tool "$PING_OUT" >/dev/null
python3 -m json.tool "$MANIFEST_OUT" >/dev/null
python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); assert d["id"] == "p1" and d["ok"] and d["result"]["pong"]' "$PING_OUT"
python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); assert d["id"] == "m1" and d["ok"] and any(c["name"] == "observe" for c in d["result"]["caps"])' "$MANIFEST_OUT"
if [ -s "$INPUT_OUT" ]; then
  python3 -m json.tool "$INPUT_OUT" >/dev/null
  python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); assert d["id"] == "i1" and d["ok"] and d["result"]["device"] == "uinput"' "$INPUT_OUT"
fi

printf '%s\n' "rust smoke passed"
