# Agent Adapter Notes

`agent-linux-control` is intentionally agent-neutral. The installer places the same `SKILL.md` in common skill locations so the user's existing agent can discover it.

## Codex

- Skill path: `~/.codex/skills/agent-linux-control/SKILL.md`
- Plugin path in this repo: `plugins/agent-linux-control`
- Primary command: `agent-linux-control doctor`

## Claude Code

- Skill path: `~/.claude/skills/agent-linux-control/SKILL.md`
- If Claude does not auto-load skills in a setup, add a short `CLAUDE.md` instruction: "Use `agent-linux-control` for Linux desktop screenshots, mouse, keyboard, and clipboard control."

## Gemini CLI

- Skill path: `~/.gemini/skills/agent-linux-control/SKILL.md`
- If the CLI uses project memory instead of skills, link or paste the skill in the project instruction file.

## OpenCode

- Skill path: `~/.config/opencode/skills/agent-linux-control/SKILL.md`
- Use the same CLI commands from the portable skill.

## Zen Browser And Browser Targets

- Zen is treated as an app target, not an agent runtime.
- Prefer Playwright, browser devtools, or DOM-level automation for web content.
- Use `agent-linux-control` for browser chrome, downloads, file pickers, extension popups, permission dialogs, and visual checks that DOM tools cannot see.

## Raspberry Pi / ARM Linux

- The installer uses the same `install.sh` path on Raspberry Pi OS, Ubuntu ARM, Fedora ARM, and other ARM Linux systems.
- Package names may differ by distro. If optional tools are unavailable, the Python `/dev/uinput` backend is still the main input path.
- On headless Pi setups, run only CLI checks unless a real desktop session and display server are active.

## Pi / Other Conversational Agents

- If the agent has shell access but no skill system, paste `skills/agent-linux-control/SKILL.md` into its project/system instructions.
- If the agent cannot run shell commands locally, this project cannot grant OS control by itself; it needs a local execution bridge.

## Other Agents

- Generic path: `~/.agents/skills/agent-linux-control/SKILL.md`
- Any agent with shell access can run `agent-linux-control` directly.
- Any agent without a skill loader can be given the skill file as persistent instructions.
