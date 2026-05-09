# Workspace Isolation

Goal: agent works like second operator without fighting user's mouse, focus, or clipboard.

## Reality

Normal Linux desktop session has one active pointer/focus path for most apps. A "second mouse" that controls real apps independently is not reliable in the same compositor session.

## Practical Modes

| Mode | User impact | Agent control | Notes |
|---|---|---|---|
| Direct desktop | Shares user pointer/focus | Good | Current `agent-linux-control`; best for dialogs, native apps, visual checks |
| App-native API | No pointer fight | Excellent | Blender Python, browser DevTools/Playwright, DB/API calls |
| Nested compositor | Separate agent display | Excellent | Weston/cage/gamescope nested Wayland; strong candidate |
| Xvfb/xpra/VNC/noVNC | Separate agent desktop | Good | Better for X11/browser workflows |
| VM/container desktop | Strong isolation | Excellent | Heavier but closest to autonomous worker |

## Architecture Direction

Add workspace commands:

- `workspace doctor`
- `workspace launch --mode nested|vnc|xvfb`
- `workspace open APP`
- `workspace observe --brief`
- `workspace stop`

Agents should default to isolated workspace for long-running autonomous work, direct desktop for user-visible dialogs and native OS interactions.
