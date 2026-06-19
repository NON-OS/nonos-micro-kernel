# capsule_settings

## Role

`capsule_settings` is a real production application capsule built on
`nonos_app_skeleton`. It owns a toggle list with cursor navigation and a
boolean state per row. The capsule keeps its own state, has no globals, and
reads keyboard input through the toolkit. The capsule routes UI rendering
through the toolkit IPC path rather than through any kernel UI code.

```text
settings app
    |
    | UI frame request
    v
toolkit endpoint
    |
    `-- app event loop on app endpoint
```

## Microkernel contract

The capsule uses the basic Mk surface that every app skeleton uses:

- `MkIpcCall` sends a UI frame request to the toolkit endpoint.
- `MkIpcRecv` receives application messages on the app endpoint.
- `MkYield` backs off when no message is available.
- `MkDebug` emits ownership and proof markers.
- `MkExit` exits when the IPC surface is parked.

The active spawn path is the standard `nonos_app_skeleton::run` entry point.

## Interface contract

| Call | Purpose |
|---|---|
| `MkIpcCall` to toolkit | request a UI frame through userland toolkit policy |
| `MkIpcRecv` on app endpoint | receive app input messages |
| `MkYield`, `MkDebug`, `MkExit` | cooperative loop and proof markers |

The keyboard surface accepts `j` and `k` for cursor movement, Space or Enter
to toggle the highlighted row, and Esc to leave the panel.

## Authority

The capsule keeps the narrow capability set defined by the app skeleton. It
does not request system settings authority, graphics, network, filesystem,
device drivers, or direct framebuffer authority. The toggles displayed are
local UI state today; no system settings are actually mutated.

## Privacy and persistence

The capsule keeps no profile, no telemetry, and no persistent state. The
toggles and the cursor position disappear with the process. Persistence is
explicitly out of scope until a settings storage contract exists.

## Runtime lifecycle

The capsule emits ownership markers, enters the app skeleton run loop, and
processes input events through `MkIpcRecv` until the IPC surface is parked. On
clean shutdown the runtime state is gone with the process.

## Failure model

Toolkit failure is observable through proof markers and never grants the
capsule a fallback framebuffer path. Invalid input is dropped by the settings
state machine and never escalates to a kernel call.

## Current implemented surface

- Source for the toggle list, cursor state, theme, painter, and event loop.
- Toggle navigation with `j`, `k`, Space, Enter, and Esc.
- Local boolean state per row.
- Runs above `nonos_app_skeleton` with toolkit-only UI.

## Wire format

Input messages arrive on the app endpoint as toolkit input events. UI requests
go out on the toolkit endpoint.

## State ownership

The capsule owns the toggle list and the cursor. The toolkit owns rendering.
The compositor owns scene and focus. The kernel owns no settings state.

## Operating rules

- Route UI through toolkit IPC only.
- Do not request direct framebuffer authority.
- Keep all app state volatile until a storage contract exists.
- Do not introduce kernel UI exports for this app.

## Release target

The release version is a signed application capsule with `Capsule.mk`, signed
manifest, feature gated spawn, toolkit only UI rendering, an explicit storage
contract for any persistent toggles, and validation evidence that settings policy
stays out of the kernel.

## Release checklist

- `Capsule.mk` and signed manifest exist.
- Toolkit IPC validation passes.
- Feature gated spawn is present.
- Static gate confirms no kernel app UI exports.

## Explicit non-goals today

No production manifest, signed spawn path, persistent settings storage,
network access, filesystem access, graphics driver access, or direct
framebuffer authority belongs in this directory.

## Verification

- Build: `cargo build --manifest-path userland/capsule_settings/Cargo.toml`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Promotion check: add `Capsule.mk`, manifest signing, feature gated spawn,
  and validation evidence before claiming production app status.
