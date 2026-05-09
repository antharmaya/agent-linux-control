---
name: agent-linux-control
description: Use when Codex, Claude Code, Gemini CLI, OpenCode, Qwen, Goose, Windsurf, or another coding agent needs to inspect, click, type, scroll, control clipboard, test Zen/browser workflows, or otherwise operate a Linux desktop through the agent-linux-control CLI on Fedora, Debian/Ubuntu, Arch, openSUSE, Alpine, Raspberry Pi/ARM Linux, KDE, GNOME, Wayland, or X11 systems.
---

# Agent Linux Control

Use the `agent-linux-control` command as the single desktop-control surface. Prefer app-native APIs, browser automation, or file edits when available; use OS control for desktop UI, native apps, browser chrome, permission dialogs, and visual verification.

This skill is portable. If the host agent does not have a first-class skill loader, paste or link this file into that agent's persistent instructions.

For browser work in Zen, Chrome, Firefox, or Electron apps, prefer DOM/browser automation when available. Use OS control for browser chrome, extension popups, permission prompts, downloads, file pickers, and visual verification.

## Startup

1. Run `agent-linux-control doctor`.
2. Run `agent-linux-control manifest` when you need the tool contract, risk classes, or recovery loop.
3. Capture before acting: `agent-linux-control observe --output /tmp/alc-screen.png`.
4. Inspect the screenshot with the image viewer available to the agent.
5. Move/click/type in small steps, or use `sequence` for an atomic batch.
6. Verify with `agent-linux-control observe` or `agent-linux-control wait-change`.

## Smooth Agent Loop

- Use `observe` instead of raw `screenshot` when possible; it returns image path, dimensions, hash, desktop/session data, and tool readiness in one JSON payload.
- Use `paste` for long text. It sets the clipboard and presses Ctrl+V, which is more reliable than keying long strings.
- Use `sequence` when you know the next few steps. It batches actions like click/paste/hotkey/sleep/observe and returns structured results.
- Use `watch` for realtime-ish monitoring while a UI is loading.
- Use `wait-change` after clicks, browser launches, reloads, and other UI transitions.
- Use `browser --prefer chrome|chromium|zen|firefox URL` to launch a supported local browser directly. Add `--require-prefer` when fallback to another browser would be wrong.
- Use `journal tail` to understand what happened across previous actions. Raw text payloads are redacted into length and SHA-256.
- Use `brain export` to create an Obsidian vault for capability maps, validation nodes, and skill-development planning.
- If the agent supports MCP, configure `agent-linux-control mcp` as a stdio MCP server and prefer the MCP tools for structured calls.

## Commands

- Observe: `agent-linux-control observe --output /tmp/alc.png`
- Manifest: `agent-linux-control manifest`
- Screenshot only: `agent-linux-control screenshot --output /tmp/alc.png`
- Screenshot with pointer when supported: `agent-linux-control screenshot --pointer --output /tmp/alc.png`
- Watch screen changes: `agent-linux-control watch --count 10 --interval 0.5 --output-dir /tmp/alc-watch`
- Wait for visible change: `agent-linux-control wait-change --timeout 10 --output /tmp/alc-changed.png`
- Move relative: `agent-linux-control move 200 0`
- Approximate absolute move: `agent-linux-control goto 900 600`
- Click current pointer: `agent-linux-control click left`
- Click coordinates: `agent-linux-control click left --x 900 --y 600`
- Scroll: `agent-linux-control scroll -5`
- Type text: `agent-linux-control type "hello"`
- Paste text: `agent-linux-control paste "hello"`
- Press key: `agent-linux-control key enter`
- Hotkey: `agent-linux-control hotkey ctrl+l`
- Clipboard set: `agent-linux-control clipboard set "text"`
- Clipboard get: `agent-linux-control clipboard get`
- Browser launch: `agent-linux-control browser --prefer chrome https://example.com`
- Strict browser launch: `agent-linux-control browser --prefer chrome --require-prefer https://example.com`
- Batched actions: `agent-linux-control sequence --file plan.json`
- MCP server: `agent-linux-control mcp`
- Journal path: `agent-linux-control journal path`
- Journal tail: `agent-linux-control journal tail --lines 10`
- Obsidian brain export: `agent-linux-control brain export --output-dir ./agent-linux-control-vault`
- Open URL/file: `agent-linux-control open https://example.com`

## Operating Rules

- Never type secrets, submit payments, send messages, delete data, or approve elevated prompts unless the user explicitly asked for that exact action.
- For important UI actions, take a screenshot before and after the action.
- If a click misses, do not repeat blindly. Capture the screen, recalculate coordinates, and try once.
- Use `clipboard set` plus paste hotkeys for long text instead of slow keystroke typing.
- On Wayland, `goto` is approximate: it clamps to the compositor's virtual desktop origin, then moves to the requested offset.
- If `doctor` says `/dev/uinput` is not writable, ask the user to run the installer or enable the udev rule; do not try to bypass OS permissions.

## Install

For a published repo:

```sh
curl -fsSL https://raw.githubusercontent.com/antharmaya/agent-linux-control/main/install.sh | sh
```

For a local checkout:

```sh
AGENT_LINUX_CONTROL_SOURCE_DIR="$PWD" ./install.sh
```
