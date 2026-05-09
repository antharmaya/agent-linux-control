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
```

## Current Commands

| Command | Purpose |
|---|---|
| `ping` | Liveness check with daemon/core version |
| `manifest` | Compact capability contract |
| `bench-summary` | Shared benchmark math over sample milliseconds |

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

1. Move `/dev/uinput` ownership into the daemon.
2. Add `observe`, `click`, `key`, `paste`, `sequence`, and `watch` commands.
3. Keep screenshot capture, diffing, journal, and input queues warm.
4. Let the Python CLI delegate to the daemon when available and fall back to stdlib behavior otherwise.
5. Put MCP on top of the same protocol so Codex, Claude Code, Gemini CLI, OpenCode, Goose, Qwen, and other agents see one consistent control surface.
