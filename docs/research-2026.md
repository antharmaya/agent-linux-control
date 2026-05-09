# 2026 Agent Experience Notes

These notes shape the product direction for `agent-linux-control`.

## What Current Agent Workflows Are Converging On

- OpenAI's April 2026 Codex update moved desktop agents beyond code editing: computer use, an agent cursor, parallel background agents, in-app browser work, richer summaries, memory, and plugins that combine skills, app integrations, and MCP servers.
- GPT-5.3-Codex is positioned for long-running tasks involving research, tool use, and complex execution, with strong Terminal-Bench, OSWorld, SWE-Bench Pro, and web-development performance.
- Claude Code emphasizes skills, MCP, subagents, hooks, custom status lines, and project-level configuration. Its skill system follows an open Agent Skills standard and loads procedural context only when relevant.
- Clicky shows the consumer UX expectation: an assistant near the cursor that sees the screen, accepts natural-language or voice intent, and can spin up background agents.

## Design Implications

- Observation should be one command, not several. `observe` returns screenshot metadata, screen dimensions, hashes, session details, and tool readiness.
- Actions should be batchable. `sequence` lets agents run click/paste/hotkey/wait/observe plans without repeated shell round trips.
- Long text should feel reliable. `paste` uses the clipboard and Ctrl+V instead of slow synthetic typing.
- Browser launch should be first class. Agents often need Chrome, Chromium, Zen, Brave, Firefox, or Edge before they can test web workflows.
- Realtime needs a primitive. `watch` emits JSONL observations at an interval; `wait-change` gives a cheap feedback loop after UI actions.
- Interoperability needs both skills and MCP. Skills teach the workflow; `agent-linux-control mcp` gives structured tools to clients that support MCP.
- First-class agents need a machine-readable contract. `manifest` exposes capability, risk, and recovery metadata so agents do not have to infer tool semantics from help text.
- Recovery needs memory. `journal` records privacy-safe JSONL events, and `brain export` turns the control layer into an Obsidian graph for skill development and validation review.
- Token economy is product quality. Repeated loops should use `--brief`, compact MCP payloads, compressed skill instructions, and stable short keys. Full JSON stays available for diagnosis.
- Linux must stay boring to install. Keep the runtime Python-stdlib-only and keep the curl installer cross-distro.

## Source Links

- OpenAI Codex for almost everything: https://openai.com/index/codex-for-almost-everything/
- OpenAI GPT-5.3-Codex: https://openai.com/index/introducing-gpt-5-3-codex/
- Anthropic Claude Code product page: https://www.anthropic.com/product/claude-code
- Claude Code skills docs: https://code.claude.com/docs/en/slash-commands
- Claude Code hooks docs: https://code.claude.com/docs/en/hooks
- Claude Code MCP docs: https://code.claude.com/docs/en/mcp
- Clicky: https://www.clicky.so/
