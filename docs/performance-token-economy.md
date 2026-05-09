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

## Rust Rule

Do not add Rust because it feels fast. Add Rust when benchmark says Python is bottleneck.

Candidate Rust targets:

- Persistent uinput device to avoid setup delay per action.
- Fast JSONL journal writer under high-frequency watch.
- Native screenshot diff/hash pipeline if compositor capture is not bottleneck.
- Optional daemon with Unix socket for sub-50ms command dispatch.

Keep Python stdlib CLI as stable installer path. Rust should be optional acceleration, not required install path.

## Benchmarks To Add

- `bench observe`: capture latency, hash latency, JSON encode time.
- `bench action`: uinput setup + click/key latency.
- `bench sequence`: N action batch latency.
- `bench mcp`: request/response overhead.
- `bench tokens`: byte/token size of full vs brief outputs.
