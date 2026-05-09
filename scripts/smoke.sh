#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
CLI="$ROOT/bin/agent-linux-control"

"$CLI" doctor --json >/tmp/agent-linux-control-doctor.json
"$CLI" screenshot --output /tmp/agent-linux-control-smoke.png >/tmp/agent-linux-control-smoke.out
"$CLI" move 1 1
"$CLI" key esc

test -s /tmp/agent-linux-control-smoke.png
python3 -m py_compile "$CLI"

printf '%s\n' "smoke passed"
