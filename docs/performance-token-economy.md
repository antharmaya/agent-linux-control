# Performance And Token Economy

Goal: make Linux desktop control feel realtime without wasting model context.

## Current Wins

- `manifest --brief`: short capability contract for repeated loops.
- `observe --brief`: screenshot path, dimensions, timing, desktop, uinput state, short hash.
- `watch --brief`: compact JSONL frames.
- `wait-change --brief`: compact transition result.
- `sequence --brief`: compact multi-step results.
- MCP tools accept `brief`/`compact` for manifest, observe, and sequence.
- Skill file compressed from verbose prose to command-first instructions.
- Wayland screenshots prefer `grim` when pointer capture is not requested.
- `sequence` reuses one `/dev/uinput` device across consecutive input steps.
- Rust `alc-daemon` prototype keeps a local Unix-socket process warm and returns compact JSONL responses with request IDs.

## Measured On Nobara/Fedora 43 KDE Wayland

| Task | Result |
|---|---:|
| `manifest --compact` | 3070 bytes |
| `manifest --brief` | 617 bytes |
| `observe --compact` | 719 bytes |
| `observe --brief` | 170 bytes |
| `observe --brief` 5-run avg | ~1347 ms before latest active-window load |
| `bench observe --count 1 --brief` | ~994 ms |
| Separate uinput action avg | ~826 ms/action |
| 3 input actions in one `sequence --brief` | ~918 ms total |
| Rust daemon first input call | ~51.2 ms with 50 ms smoke device delay |
| Rust daemon warm input call over one socket | ~0.38 ms avg |
| Python CLI delegating to Rust daemon | ~172.49 ms first, ~104.53 ms warm avg |
| Python per-action uinput after daemon benchmark | ~827.57 ms avg |
| Blender background startup + script | ~2.09 s wall |
| Blender cube/material scene ops after startup | ~50.81 ms |
| Blender UI first visible change | ~1.96 s |

Interpretation: app-native APIs are fast once app is loaded. Current bottlenecks are screenshot capture, per-process startup, and per-device input setup. Batching input actions is already a large win. Rust daemon-owned input removes the device setup cost almost entirely once warm. Python CLI delegation is much faster than direct Python uinput, but still pays Python process startup; MCP or native Rust CLI should use a long-lived connection for realtime feel.

## Rust Pivot

Rust pivot is now active. Keep Python stdlib CLI as compatibility/install path while Rust becomes realtime engine.

Candidate Rust targets:

- Persistent uinput device to avoid setup delay per action.
- Fast JSONL journal writer under high-frequency watch.
- Native screenshot diff/hash pipeline if compositor capture is not bottleneck.
- Optional daemon with Unix socket for sub-50ms command dispatch.

Keep Python stdlib CLI as stable installer path until Rust reaches command parity.

## Pivot Rule

Short term: Rust core models + CLI prototype, Python compatibility, batching, app-native adapters, isolated workspaces.

Medium term: Rust daemon. The daemon now owns a lazy persistent uinput device and the Python CLI delegates compatible input actions to it when available. Next target is watch/diff state plus MCP on the same socket. Rust is a strong fit for persistent uinput, event loop, socket protocol, and high-frequency watch/diff. Python remains fallback control plane.

## Benchmarks To Add

- `bench observe`: capture latency, hash latency, JSON encode time.
- `bench action`: uinput setup + click/key latency.
- `bench sequence`: N action batch latency.
- `bench mcp`: request/response overhead.
- `bench tokens`: byte/token size of full vs brief outputs.
- `bench daemon`: CLI-to-daemon round trip, long-lived client latency, concurrent client behavior.
