---
name: agent-linux-control
description: Use when Codex, Claude Code, Gemini CLI, OpenCode, Qwen, Goose, Windsurf, or another coding agent needs Linux desktop control: observe screen, click, type, paste, scroll, clipboard, browser launch, MCP tools, Obsidian brain map, Fedora/Debian/Arch/openSUSE/Alpine/Pi/ARM, KDE/GNOME, Wayland/X11.
---

# Agent Linux Control

One CLI for Linux OS control. Prefer DOM/app APIs for web content; use this for desktop UI, browser chrome, dialogs, downloads, file pickers, screenshots, visual verification.

## Loop

1. `agent-linux-control doctor`
2. `agent-linux-control manifest --brief`
3. `agent-linux-control observe --brief --output /tmp/alc.png`
4. Inspect screenshot.
5. Act once, or batch with `sequence --brief`.
6. Verify via `wait-change --brief` or `observe --brief`.
7. Recover/debug via `journal tail`.

## Token Rules

- Prefer `--brief` for agent loops. Use full JSON only when diagnosing.
- Prefer `paste` over `type` for long text.
- Prefer `sequence --brief` for known multi-step plans; fewer tool calls, fewer tokens.
- If `alc-daemon` is running, CLI and MCP input commands auto-use it; otherwise Python `/dev/uinput` fallback runs.
- Prefer MCP `manifest/observe/sequence` with `{"brief":true}` when MCP available.
- Journal redacts text payloads to length + SHA-256.
- Prefer app-native APIs for real work inside apps; use desktop control for launch, dialogs, visual verification.
- For long autonomous work, prefer isolated workspace when available; direct desktop shares user pointer/focus.

## Commands

- Contract: `agent-linux-control manifest --brief`
- Observe: `agent-linux-control observe --brief --output /tmp/alc.png`
- Watch: `agent-linux-control watch --brief --count 10 --interval 0.5 --output-dir /tmp/alc-watch`
- Wait: `agent-linux-control wait-change --brief --timeout 10 --output /tmp/alc-changed.png`
- Screenshot: `agent-linux-control screenshot --output /tmp/alc.png`
- Move: `agent-linux-control move 200 0`
- Goto: `agent-linux-control goto 900 600`
- Click: `agent-linux-control click left --x 900 --y 600`
- Scroll: `agent-linux-control scroll -5`
- Type: `agent-linux-control type "hello"`
- Paste: `agent-linux-control paste "hello"`
- Key: `agent-linux-control key enter`
- Hotkey: `agent-linux-control hotkey ctrl+l`
- Clipboard: `agent-linux-control clipboard set "text"` / `agent-linux-control clipboard get`
- Browser: `agent-linux-control browser --prefer chrome --require-prefer https://example.com`
- Batch: `agent-linux-control sequence --brief --file plan.json`
- Daemon bench: `agent-linux-control bench daemon --count 8`
- MCP: `agent-linux-control mcp`
- Journal: `agent-linux-control journal tail --lines 10`
- Obsidian: `agent-linux-control brain export --output-dir ./agent-linux-control-vault`
- Daemon: `alc-daemon serve --socket /tmp/agent-linux-control.sock`

## Safety

- Never type secrets, approve elevated prompts, submit payments, send messages, or delete data unless user explicitly asked for that exact action.
- Observe before/after important UI actions.
- If click misses, re-observe and recalc; do not repeat blindly.
- Wayland `goto` is approximate.
- If `/dev/uinput` not writable, ask user to run installer or enable udev rule. Do not bypass permissions.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/antharmaya/agent-linux-control/main/install.sh | sh
```

Local:

```sh
AGENT_LINUX_CONTROL_SOURCE_DIR="$PWD" ./install.sh
```
