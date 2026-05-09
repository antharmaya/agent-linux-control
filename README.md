# Agent Linux Control

Small Linux desktop-control toolkit for coding agents. It gives an agent one CLI for screenshots, mouse, keyboard, clipboard, and readiness checks on Fedora/Nobara, Debian/Ubuntu, Arch, openSUSE, Alpine, KDE, GNOME, Wayland, and X11-like setups.

The goal is plug-and-play computer control for Linux agents in the same spirit as modern desktop-agent products: keep the user's existing agent, install one tool, then let that agent inspect and operate the local desktop safely. It is designed for Codex, Claude Code, Gemini CLI, OpenCode, Qwen, Goose, Windsurf, generic shell agents, and browser-heavy workflows such as Zen Browser testing.

## Install

Published repo:

```sh
curl -fsSL https://raw.githubusercontent.com/antharmaya-labs/agent-linux-control/main/install.sh | sh
```

Local checkout:

```sh
AGENT_LINUX_CONTROL_SOURCE_DIR="$PWD" ./install.sh
```

The installer detects `dnf`, `apt-get`, `pacman`, `zypper`, or `apk`, installs useful tools when possible, adds a `/dev/uinput` uaccess rule when sudo is available, installs `agent-linux-control` into `~/.local/bin`, and installs the agent skill into common locations:

- `~/.agents/skills/agent-linux-control`
- `~/.codex/skills/agent-linux-control`
- `~/.claude/skills/agent-linux-control`
- `~/.gemini/skills/agent-linux-control`
- `~/.config/opencode/skills/agent-linux-control`
- `~/.qwen/skills/agent-linux-control`
- `~/.config/goose/skills/agent-linux-control`
- `~/.config/crush/skills/agent-linux-control`
- `~/.codeium/windsurf/skills/agent-linux-control`

On Raspberry Pi and other ARM Linux systems, the same installer path is used. Package availability depends on the distro repo; the Python `/dev/uinput` backend remains the fallback when higher-level tools are missing.

## Use

```sh
agent-linux-control doctor
agent-linux-control screenshot --output /tmp/screen.png
agent-linux-control click left --x 900 --y 600
agent-linux-control type "hello"
agent-linux-control hotkey ctrl+l
agent-linux-control clipboard set "long text"
```

## Project Layout

- `bin/agent-linux-control` - Python stdlib CLI; no daemon required.
- `install.sh` - curl-friendly installer.
- `skills/agent-linux-control/SKILL.md` - portable skill for agents.
- `plugins/agent-linux-control/` - Codex plugin wrapper around the skill.
- `adapters/` - notes for Codex, Claude Code, Gemini CLI, OpenCode, Zen/browser targets, Pi/ARM Linux, and other agents.

## Safety

The CLI can control the desktop. Agents should capture screenshots before and after important actions, avoid secret entry without explicit user approval, and use app-native APIs before OS-level UI control when possible.
