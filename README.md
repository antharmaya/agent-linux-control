# Agent Linux Control

Premium Linux desktop-control toolkit for coding agents. It gives an agent one CLI for realtime observation, screenshots, mouse, keyboard, clipboard, browser launch, batched action sequences, MCP access, and readiness checks on Fedora/Nobara, Debian/Ubuntu, Arch, openSUSE, Alpine, KDE, GNOME, Wayland, and X11-like setups.

The goal is plug-and-play computer control for Linux agents in the same spirit as modern desktop-agent products: keep the user's existing agent, install one tool, then let that agent inspect and operate the local desktop safely. It is designed for Codex, Claude Code, Gemini CLI, OpenCode, Qwen, Goose, Windsurf, generic shell agents, and browser-heavy workflows such as Zen Browser testing.

## Install

Published repo:

```sh
curl -fsSL https://raw.githubusercontent.com/antharmaya/agent-linux-control/main/install.sh | sh
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
agent-linux-control observe --output /tmp/screen.png
agent-linux-control click left --x 900 --y 600
agent-linux-control paste "long text that should land cleanly"
agent-linux-control hotkey ctrl+l
agent-linux-control clipboard set "long text"
agent-linux-control browser --prefer chrome https://reddit.com
agent-linux-control browser --prefer chrome --require-prefer https://reddit.com
agent-linux-control watch --count 10 --interval 0.5 --output-dir /tmp/alc-watch
```

## Agent Loop

Use the higher-level loop for smoother work:

1. `agent-linux-control observe --output /tmp/alc-screen.png`
2. Inspect the screenshot in the host agent.
3. Run one precise action or a batch:

```sh
agent-linux-control sequence --file examples/sequence.browser-observe.json
```

4. Use `agent-linux-control wait-change` or `agent-linux-control observe` to verify the UI changed.

`sequence` accepts JSON steps such as `observe`, `browser`, `click`, `type`, `paste`, `hotkey`, `scroll`, `wait-change`, and `sleep`. Batching removes tool-call latency and makes the desktop feel closer to a realtime control surface. Browser steps accept `require_prefer: true` when the agent must fail instead of falling back to another browser.

## MCP

Any MCP-capable agent can expose the same controls by running:

```sh
agent-linux-control mcp
```

Example `.mcp.json`:

```json
{
  "mcpServers": {
    "agent-linux-control": {
      "command": "agent-linux-control",
      "args": ["mcp"]
    }
  }
}
```

The MCP server exposes `observe`, `click`, `type`, `paste`, `hotkey`, `browser`, and `sequence` tools.

## Verified Hosts

- 2026-05-09: Nobara/Fedora 43 KDE Plasma Wayland.
  - Public curl installer succeeded from `raw.githubusercontent.com/antharmaya/agent-linux-control`.
  - `dnf` installed missing optional packages: `grim`, `slurp`, `scrot`, `imlib2`.
  - `agent-linux-control doctor` reported writable `/dev/uinput`.
  - Installed CLI captured a 1920x2160 screenshot and sent a small uinput move/key smoke successfully.

## Project Layout

- `bin/agent-linux-control` - Python stdlib CLI; no daemon required.
- `install.sh` - curl-friendly installer.
- `skills/agent-linux-control/SKILL.md` - portable skill for agents.
- `plugins/agent-linux-control/` - Codex plugin wrapper around the skill.
- `examples/` - ready-to-copy MCP and sequence configs.
- `docs/research-2026.md` - product/research notes behind the agent experience.
- `adapters/` - notes for Codex, Claude Code, Gemini CLI, OpenCode, Zen/browser targets, Pi/ARM Linux, and other agents.

## Safety

The CLI can control the desktop. Agents should capture screenshots before and after important actions, avoid secret entry without explicit user approval, and use app-native APIs before OS-level UI control when possible.
