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
2. Capture before acting: `agent-linux-control screenshot --output /tmp/alc-screen.png`.
3. Inspect the screenshot with the image viewer available to the agent.
4. Move/click/type in small steps, then capture again.

## Commands

- Screenshot: `agent-linux-control screenshot --output /tmp/alc.png`
- Screenshot with pointer when supported: `agent-linux-control screenshot --pointer --output /tmp/alc.png`
- Move relative: `agent-linux-control move 200 0`
- Approximate absolute move: `agent-linux-control goto 900 600`
- Click current pointer: `agent-linux-control click left`
- Click coordinates: `agent-linux-control click left --x 900 --y 600`
- Scroll: `agent-linux-control scroll -5`
- Type text: `agent-linux-control type "hello"`
- Press key: `agent-linux-control key enter`
- Hotkey: `agent-linux-control hotkey ctrl+l`
- Clipboard set: `agent-linux-control clipboard set "text"`
- Clipboard get: `agent-linux-control clipboard get`
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
curl -fsSL https://raw.githubusercontent.com/antharmaya-labs/agent-linux-control/main/install.sh | sh
```

For a local checkout:

```sh
AGENT_LINUX_CONTROL_SOURCE_DIR="$PWD" ./install.sh
```
