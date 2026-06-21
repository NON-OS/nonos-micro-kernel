# capsule_process_manager

## Role

`capsule_process_manager` is a real production application capsule built on
`nonos_app_skeleton`. It is an observability surface that honestly reports the
kernel debug syscall as `E_NOSYS` and exposes an input-driven refresh counter.
The capsule keeps its own state, has no globals, and reads keyboard input
through the toolkit. The capsule routes UI rendering through the toolkit IPC
path rather than through any kernel UI code.

```text
process manager app
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

The keyboard surface accepts a refresh key and Esc.

## Authority

The capsule keeps the narrow capability set defined by the app skeleton. It
does not request kernel statistics authority, graphics, network, filesystem,
device drivers, or direct framebuffer authority. The view shown to the user
is the honest result of the kernel debug syscall, which is `E_NOSYS` today,
plus a local counter of refresh events.

## Privacy and persistence

The capsule keeps no profile, no settings, no telemetry, and no persistent
state. The refresh counter and the last response disappear with the process.

## Runtime lifecycle

The capsule emits ownership markers, enters the app skeleton run loop, and
processes input events through `MkIpcRecv` until the IPC surface is parked. On
clean shutdown the runtime state is gone with the process.

## Failure model

Toolkit failure is observable through proof markers and never grants the
capsule a fallback framebuffer path. The `E_NOSYS` response from the kernel
debug syscall is shown verbatim; the capsule does not paper it over.

## Current implemented surface

- Source for the observability state, painter, theme, and event loop.
- Honest reporting of the kernel debug syscall response.
- Refresh counter driven by input events.
- Runs above `nonos_app_skeleton` with toolkit-only UI.

## Wire format

Input messages arrive on the app endpoint as toolkit input events. UI requests
go out on the toolkit endpoint.

## State ownership

The capsule owns the local response cache and the refresh counter. The toolkit
owns rendering. The compositor owns scene and focus. The kernel owns no
process manager state.

## Operating rules

- Route UI through toolkit IPC only.
- Do not request direct framebuffer authority.
- Keep all app state volatile.
- Do not introduce kernel UI exports for this app.

## Release target

The release version is a signed application capsule with `Capsule.mk`, signed
manifest, feature gated spawn, toolkit only UI rendering, an explicit
capability for whatever kernel statistics surface eventually exists, and
validation evidence that process manager policy stays out of the kernel.

## Release checklist

- `Capsule.mk` and signed manifest exist.
- Toolkit IPC validation passes.
- Feature gated spawn is present.
- Static gate confirms no kernel app UI exports.

## Explicit non-goals today

No production manifest, signed spawn path, persistent settings, real kernel
statistics surface, network access, filesystem access, graphics driver access,
or direct framebuffer authority belongs in this directory.

## Verification

- Build: `cargo build --manifest-path userland/capsule_process_manager/Cargo.toml`
- Static gate: `bash nonos-ci/run-static-checks.sh`
- Promotion check: add `Capsule.mk`, manifest signing, feature gated spawn,
  and validation evidence before claiming production app status.
