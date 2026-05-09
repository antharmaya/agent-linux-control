# Agent OS Architecture

`agent-linux-control` should feel less like a bag of shell commands and more like an OS-style substrate for agent work on Linux.

## Layer Model

| Layer | Responsibility | Current Surface |
|---|---|---|
| Capability manifest | Tell agents what exists, what is risky, and how to recover | `manifest` |
| Perception | Capture screen state and change over time | `observe`, `watch`, `wait-change` |
| Actuation | Move pointer, click, type, paste, scroll, launch apps | `click`, `type`, `paste`, `browser`, `sequence` |
| Integration | Let agents call the same controls through structured tools | `mcp`, installed skills |
| Realtime daemon | Keep input devices, watch state, and protocol dispatch warm | Rust `alc-daemon` with daemon-owned uinput |
| Journal | Preserve privacy-safe event traces for recovery and review | `journal` |
| Brain map | Turn capabilities and validation into a graph workspace | `brain export` for Obsidian |
| Token economy | Keep repeated agent loops compact and fast | `--brief`, compact skill text |

## Industry Pattern Without Vendor Lock-In

Public agent systems are converging on the same shape:

- Skills teach workflow.
- MCP exposes tools.
- Hooks automate deterministic checks.
- Subagents isolate focused work.
- Memory and journals make work resumable.
- Visual surfaces make state inspectable.
- Token-efficient contracts keep agent loops snappy.

This project should implement those ideas as portable Linux infrastructure rather than as a single vendor app shell.

## Kernel Analogy

- Drivers: Wayland/X11 screenshot tools, `/dev/uinput`, clipboard tools, browser launchers.
- Syscalls: `observe`, `click`, `paste`, `sequence`, `wait-change`.
- Compact syscalls: `manifest --brief`, `observe --brief`, `sequence --brief`.
- Scheduler: future action queues, retries, timeouts, and budgets.
- Permissions: future policy engine for dangerous actions.
- Audit log: privacy-safe JSONL journal.
- Filesystem/graph: Obsidian vault export for node-based reasoning.

## Next First-Class Upgrades

1. Move realtime core to Rust: `alc-core`, `alc`, `alc-daemon`.
2. Move screenshot/watch/journal paths into the daemon protocol.
3. Add workspace isolation: nested compositor, VNC/xpra, app-native adapters.
4. Add dialog detection/recipes for app modals.
5. Add window/app discovery: `window-list`, `active-window`, `focus`.
6. Add structured error taxonomy so agents can self-recover deterministically.
7. Add adaptive budgets for `sequence` and MCP calls.
8. Add hook templates for Codex/Claude/OpenCode style workflows.
