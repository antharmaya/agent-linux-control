#!/usr/bin/env sh
set -eu

ROOT="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
CLI="$ROOT/bin/agent-linux-control"

"$CLI" doctor --json >/tmp/agent-linux-control-doctor.json
"$CLI" manifest --compact >/tmp/agent-linux-control-manifest.json
"$CLI" manifest --brief >/tmp/agent-linux-control-manifest-brief.json
"$CLI" screenshot --output /tmp/agent-linux-control-smoke.png >/tmp/agent-linux-control-smoke.out
"$CLI" observe --output /tmp/agent-linux-control-observe.png --compact >/tmp/agent-linux-control-observe.json
"$CLI" observe --output /tmp/agent-linux-control-observe-brief.png --brief >/tmp/agent-linux-control-observe-brief.json
"$CLI" watch --count 1 --interval 0.1 --output-dir /tmp/agent-linux-control-watch >/tmp/agent-linux-control-watch.jsonl
"$CLI" watch --brief --count 1 --interval 0.1 --output-dir /tmp/agent-linux-control-watch-brief >/tmp/agent-linux-control-watch-brief.jsonl
"$CLI" journal path >/tmp/agent-linux-control-journal-path.txt
"$CLI" brain export --output-dir /tmp/agent-linux-control-vault --compact >/tmp/agent-linux-control-brain.json
"$CLI" bench observe --count 1 --brief --output-dir /tmp/agent-linux-control-bench >/tmp/agent-linux-control-bench-observe.json
printf '%s\n' '{"steps":[{"action":"observe","output":"/tmp/agent-linux-control-sequence.png"},{"action":"sleep","seconds":0.01}]}' | "$CLI" sequence --file - --compact >/tmp/agent-linux-control-sequence.json
printf '%s\n' '{"steps":[{"action":"observe","output":"/tmp/agent-linux-control-sequence-brief.png"},{"action":"sleep","seconds":0.01}]}' | "$CLI" sequence --file - --brief >/tmp/agent-linux-control-sequence-brief.json
printf '%s\n' '{"steps":[{"action":"browser","url":"https://example.com","prefer":"missing-browser","require_prefer":true}]}' | "$CLI" sequence --file - --compact >/tmp/agent-linux-control-sequence-error.json || true
printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | "$CLI" mcp >/tmp/agent-linux-control-mcp.jsonl
printf '%s\n' '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"browser","arguments":{"url":"https://example.com","prefer":"missing-browser","require_prefer":true}}}' | "$CLI" mcp >/tmp/agent-linux-control-mcp-error.jsonl
"$CLI" move 1 1
"$CLI" key esc

test -s /tmp/agent-linux-control-smoke.png
test -s /tmp/agent-linux-control-observe.png
test -s /tmp/agent-linux-control-sequence.png
python3 -m json.tool /tmp/agent-linux-control-doctor.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-manifest.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-manifest-brief.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-observe.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-observe-brief.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-brain.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-bench-observe.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-sequence.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-sequence-brief.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-sequence-error.json >/dev/null
python3 -m json.tool /tmp/agent-linux-control-mcp-error.jsonl >/dev/null
python3 -m py_compile "$CLI"

printf '%s\n' "smoke passed"
