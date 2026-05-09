# Daemon Protocol

`alc-daemon` is the Rust realtime control process. The prototype establishes the socket protocol now; its job is to keep the fast path alive between agent actions instead of starting a new process and opening devices for every click, key, watch, or journal write.

## Transport

- Local Unix socket.
- Newline-delimited JSON.
- One request line returns one response line.
- `id` is optional but should be sent by agents so responses can be correlated.
- Malformed JSON returns an error response without an `id`, because the envelope could not be trusted.
- The protocol is intentionally small enough for CLI, MCP, and future UI overlays to share.

Default socket:

```sh
/tmp/agent-linux-control.sock
```

## Run

```sh
cargo run -q -p alc-daemon -- serve --socket /tmp/agent-linux-control.sock
```

Call it:

```sh
cargo run -q -p alc-daemon -- call --socket /tmp/agent-linux-control.sock '{"id":"p1","cmd":"ping"}'
cargo run -q -p alc-daemon -- call --socket /tmp/agent-linux-control.sock '{"id":"m1","cmd":"manifest"}'
cargo run -q -p alc-daemon -- call --socket /tmp/agent-linux-control.sock '{"id":"b1","cmd":"bench-summary","samples":[10,20,30]}'
cargo run -q -p alc-daemon -- call --socket /tmp/agent-linux-control.sock '{"id":"i1","cmd":"input","steps":[{"action":"move","dx":1,"dy":1}]}'
```

## Current Commands

| Command | Purpose |
|---|---|
| `ping` | Liveness check with daemon/core version |
| `manifest` | Compact capability contract |
| `bench-summary` | Shared benchmark math over sample milliseconds |
| `input` | Batch OS input actions through daemon-owned `/dev/uinput` |

## Input Steps

`input` accepts a `steps` array. Supported actions:

```json
{"action":"move","dx":1,"dy":1}
{"action":"goto","x":900,"y":600}
{"action":"click","button":"left"}
{"action":"click","button":"left","x":900,"y":600,"delay_ms":50}
{"action":"scroll","vertical":-3,"horizontal":0}
{"action":"key","name":"esc"}
{"action":"hotkey","chord":"ctrl+l"}
{"action":"type","text":"hello"}
```

The daemon lazily opens `/dev/uinput` on first input command and keeps that virtual device alive for later requests. The Python `agent-linux-control` CLI automatically delegates input commands to this socket when it is available; set `AGENT_LINUX_CONTROL_NO_DAEMON=1` to force direct Python `/dev/uinput`.

## Response Shape

Success:

```json
{"id":"p1","ok":true,"result":{"pong":true,"v":"0.1.0"}}
```

Error:

```json
{"ok":false,"error":"..."}
```

## Near-Term Expansion

1. Add `observe`, `paste`, `sequence`, and `watch` commands.
2. Keep screenshot capture, diffing, journal, and input queues warm.
3. Put MCP on top of the same protocol so Codex, Claude Code, Gemini CLI, OpenCode, Goose, Qwen, and other agents see one consistent control surface.
