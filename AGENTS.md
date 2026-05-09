# Agent Linux Control Contract

Goal: provide plug-and-play Linux desktop control for coding agents without binding to one vendor.

## Rules

1. Keep `bin/agent-linux-control` as the canonical runtime.
2. Keep `install.sh` working with `curl ... | sh` after the repo is published.
3. Keep `skills/agent-linux-control/SKILL.md` portable across Codex, Claude Code, Gemini CLI, OpenCode, Qwen, Goose, Windsurf, and generic `.agents` skill loaders.
4. Prefer distro/package-manager detection over distro-specific branches when possible.
5. Verify on at least one Wayland KDE/Fedora-like system before claiming readiness.

## Checks

```sh
python3 -m py_compile bin/agent-linux-control
./scripts/smoke.sh
AGENT_LINUX_CONTROL_SOURCE_DIR="$PWD" AGENT_LINUX_CONTROL_NO_DEPS=1 ./install.sh
```

## Safety

The tool can click, type, and control the desktop. Agent instructions must require screenshots before and after important UI actions, and must not approve elevated prompts, enter secrets, send messages, make purchases, or delete user data without explicit user instruction.
