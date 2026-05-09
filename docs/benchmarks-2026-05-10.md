# Benchmarks 2026-05-10

Host: Nobara/Fedora 43 KDE Wayland, 1920x2160 virtual desktop, Blender 5.1.0.

## Blender Task

Script: `examples/blender_cube_material.py`

Creates cube, electric-blue material, area light, camera, saves `/tmp/agent-linux-control-blender-cube.blend`.

Results:

- Background Blender startup + script: ~2.09 s wall.
- Scene operations inside Blender: 50.81 ms.
- Output file: 95 KiB.
- Blender UI process spawn call: ~0.93 ms.
- First visible UI change: ~1.96 s.
- First-run Blender preferences modal appeared and blocked viewport.

Conclusion: use app-native APIs for actual creative work. Use desktop control for launch, visual verification, and dialog handling.

## Agent Loop

- `observe --brief`: ~170 bytes.
- `manifest --brief`: ~617 bytes.
- `bench observe --count 1 --brief`: ~994 ms in one run.
- 5-run observe avg earlier: ~1347 ms.
- Separate uinput setup/action: ~826 ms per action.
- 3 consecutive moves in one `sequence --brief`: ~918 ms total.
- Rust daemon input over one long-lived socket: first call ~51.2 ms with a 50 ms smoke device delay, then warm calls averaged ~0.38 ms.
- `agent-linux-control bench daemon --count 8`: first call ~51.29 ms, warm calls averaged ~0.24 ms.
- Python CLI delegated to Rust daemon: first call ~172.49 ms, warm process-per-command calls averaged ~104.53 ms.
- Python per-action uinput rerun after daemon benchmark: ~827.57 ms average across 3 moves.

Conclusion: Rust not required for Blender logic. Biggest control win is persistent input device or daemon. The Rust daemon result validates this: warm input dispatch is sub-millisecond on the local socket before compositor/UI effects. Python CLI delegation is already a large win, but realtime agents should prefer long-lived MCP/socket paths.
