# Rust Pivot Plan

Goal: Rust becomes realtime agent-control engine. Python CLI remains compatibility shim until Rust reaches parity.

## Judgment From Current Work

- `agent-linux-control` skill helps by forcing observe -> act -> verify and providing compact commands.
- Browser launch helped open research pages, but browser UI is not ideal for fast research; shell/web APIs are faster. Desktop browser control is still critical for real browser chrome, dialogs, permissions, and visual verification.
- Blender benchmark showed app-native APIs beat desktop clicking for creative work. Desktop control should orchestrate, verify, and handle modals.
- `sequence` input reuse proved persistent state matters. Rust daemon now owns persistent uinput; journal, workspace sessions, and event loop follow.

## Rust Target Architecture

| Crate | Role |
|---|---|
| `alc-core` | contracts, capability manifest, bench math, shared models |
| `alc` | Rust CLI, eventually replaces Python `bin/agent-linux-control` |
| `alc-daemon` | persistent Unix-socket daemon: protocol and uinput now, watch/workspace sessions next |
| `alc-mcp` | future MCP server using official Rust SDK |

## Why Rust

- Persistent `/dev/uinput` device without Python process/device setup per action.
- Tokio Unix socket daemon for low-latency command dispatch.
- Strong typed protocol between CLI, daemon, MCP, and future UI overlay.
- Optional native screenshot hash/diff pipeline.
- Single static-ish binary possible later.

## Research Anchors

- Official Rust MCP SDK exists but is Tier 2 in MCP docs; good target after core daemon.
- Linux kernel docs recommend libevdev over raw uinput for new software.
- Tokio Unix sockets fit local daemon IPC.
- Existing Python stdlib CLI remains fallback and installer-safe path during migration.

## Agent Browser Judgment

`agent-linux-control browser --prefer chrome --require-prefer https://www.rust-lang.org/tools/install` worked and proved browser launch path is useful for opening target docs in the user's real desktop.

For research itself, browser UI is slower than direct web/search tooling: more visual latency, more modal risk, more focus contention. Browser control remains valuable for:

- Real browser chrome.
- Auth/permission/download/file picker flows.
- Visual verification.
- Testing how agents experience the same UI as user.

## Migration Steps

1. Rust scaffold compiles: `alc-core`, `alc`. Done.
2. Port manifest, bench math, compact models. Done.
3. Add Rust daemon prototype with Unix socket. Done: `alc-daemon` accepts JSONL requests and preserves response IDs.
4. Port uinput to Rust daemon, benchmark against Python per-action setup. Done: warm daemon input averaged ~0.38 ms vs Python per-action ~827.57 ms.
5. Add Python shim: if Rust daemon socket exists, delegate compatible commands. Done for input commands, MCP input tools, and input steps inside `sequence`.
6. Add Rust-native MCP server after more daemon protocol commands stabilize.
7. Add workspace isolation adapters: nested compositor / VNC / app-native runners.

## Current Daemon Smoke

```sh
./scripts/rust-smoke.sh
cargo run -q -p alc-daemon -- serve --socket /tmp/agent-linux-control.sock
cargo run -q -p alc-daemon -- call --socket /tmp/agent-linux-control.sock '{"id":"p1","cmd":"ping"}'
cargo run -q -p alc-daemon -- call --socket /tmp/agent-linux-control.sock '{"id":"i1","cmd":"input","steps":[{"action":"move","dx":1,"dy":1}]}'
AGENT_LINUX_CONTROL_DAEMON_SOCKET=/tmp/agent-linux-control.sock agent-linux-control move 1 1
AGENT_LINUX_CONTROL_DAEMON_SOCKET=/tmp/agent-linux-control.sock agent-linux-control bench daemon --count 8
```

Protocol details live in `docs/daemon-protocol.md`.

## Non-Goals For First Rust Pass

- Do not remove Python installer.
- Do not rewrite screenshots until benchmark says it matters.
- Do not add security-heavy policy layer before realtime core exists.
