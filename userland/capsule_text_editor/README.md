# capsule_text_editor

## Role

`capsule_text_editor` is a real production application capsule built on
`nonos_app_skeleton`. It owns a fixed capacity edit buffer with column wrap.
The capsule keeps its own state, has no globals, and reads keyboard input
through the toolkit. The capsule routes UI rendering through the toolkit IPC
path rather than through any kernel UI code.

```text
text editor app
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

The keyboard surface accepts printable characters, Backspace to delete the
character before the cursor, Enter to insert a newline, and Esc to leave the
editor.

## Authority

The capsule keeps the narrow capability set defined by the app skeleton. It
does not request filesystem authority, graphics, network, device drivers, or
direct framebuffer authority. The editor does not read or write files today;
the buffer is local owned state.

## Privacy and persistence

The capsule keeps no profile, no telemetry, and no persistent state. The
buffer contents and the cursor disappear with the process. Persistence is
explicitly out of scope until a filesystem contract exists.

## Runtime lifecycle

The capsule emits ownership markers, enters the app skeleton run loop, and
processes input events through `MkIpcRecv` until the IPC surface is parked. On
clean shutdown the runtime state is gone with the process.

## Failure model

Toolkit failure is observable through proof markers and never grants the
capsule a fallback framebuffer path. Invalid input is dropped by the editor
state machine and never escalates to a kernel call.

## Current implemented surface

- Source for the edit buffer, cursor position, theme, painter, and event loop.
- Keyboard surface for printable characters, Backspace, Enter, and Esc.
- Fixed capacity buffer with column wrap and deterministic behaviour at the
  buffer boundary.
- Runs above `nonos_app_skeleton` with toolkit-only UI.

## Wire format

Input messages arrive on the app endpoint as toolkit input events. UI requests
go out on the toolkit endpoint.

## State ownership

The capsule owns the buffer and the cursor. The toolkit owns rendering. The
compositor owns scene and focus. The kernel owns no editor state.

## Operating rules

- Route UI through toolkit IPC only.
- Do not request direct framebuffer authority.
- Keep all app state volatile until a filesystem contract exists.
- Do not introduce kernel UI exports for this app.

## Release target

The release version is a signed application capsule with `Capsule.mk`, signed
manifest, feature gated spawn, toolkit only UI rendering, an explicit
capability for filesystem access where it is eventually granted, and validation
proof that editor policy stays out of the kernel.

## Release checklist

- `Capsule.mk` and signed manifest exist.
- Toolkit IPC validation passes.
- Feature gated spawn is present.
- Static gate confirms no kernel app UI exports.

## Explicit non-goals today

No production manifest, signed spawn path, real file open or save, network
access, graphics driver access, or direct framebuffer authority belongs in
this directory.

## Verification

- Build: `cargo build --manifest-path userland/capsule_text_editor/Cargo.toml`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Promotion check: add `Capsule.mk`, manifest signing, feature gated spawn,
  and validation evidence before claiming production app status.
