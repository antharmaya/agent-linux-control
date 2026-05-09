# Dialog Handling

Testing Blender exposed a first-run preferences modal. This is a core agent-control problem: real apps interrupt workflows with onboarding, permissions, updates, unsaved changes, auth prompts, and file pickers.

## Required Pattern

1. Observe.
2. Classify dialog type.
3. Pick known recipe.
4. Act minimally.
5. Verify dialog gone or expected state reached.

## Recipes To Build

- First-run onboarding: dismiss/accept defaults only when safe.
- File picker: use path field or keyboard navigation.
- Permission prompt: ask user unless task explicitly authorized.
- Unsaved changes: save-as path or ask user.
- Update modal: dismiss unless update requested.
- Auth/payment/delete prompts: always require explicit user approval.

## CLI Direction

Add future commands:

- `dialog detect --brief`
- `dialog recipe list`
- `dialog handle --recipe blender-first-run --verify`

Initial implementation can be recipe docs + screenshot classifier hints. Later: CV/OCR-assisted detection.
