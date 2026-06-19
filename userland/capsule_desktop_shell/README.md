# capsule_desktop_shell

## Role

`capsule_desktop_shell` is the userland desktop chrome capsule. It
owns the overlay surface (taskbar / tray / notifications / spotlight)
that lives above every window in z-order. It consumes the compositor
and wm services and coordinates with the wallpaper and market
capsules. No graphics hardware is touched directly.

```text
desktop_shell (this capsule)
    |
    | overlay surface (z = 1, full-screen)
    v
compositor --> driver.virtio_gpu0 --> display
    ^
    |
wm / wallpaper / market (peer IPC)
```

## Microkernel contract

- `MkIpcRecv` on port `4410` reads tray/spotlight/notify requests
  from app capsules.
- `MkIpcCall` against the compositor, wm, wallpaper, market.
- `MkMmap` allocates the overlay backing buffer.
- `MkSurfaceRegister` / `MkSurfaceShare` publish the overlay.
- `MkDisplayDimensions` learns the primary display size.
- `MkExit` is the only termination path.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_TRAY_ADD` | 0x0002 | app registers a tray entry |
| `OP_TRAY_REMOVE` | 0x0003 | drop a tray entry |
| `OP_TRAY_UPDATE` | 0x0004 | mutate a tray entry label |
| `OP_NOTIFY` | 0x0005 | post a transient notification |
| `OP_SPOTLIGHT_OPEN` | 0x0006 | toggle the spotlight overlay |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x1819`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | recv on 4410 + send to peers |
| 0x0010 | Memory | overlay backing + tray + notify tables |
| 0x0800 | GraphicsDisplayQuery | learn display dimensions |
| 0x1000 | GraphicsSurfaceCreate | register the overlay surface |

`Debug` is **deliberately absent**. No driver/network/fs/crypto cap.

## Privacy posture

| Invariant | How `capsule_desktop_shell` honors it |
|---|---|
| NO LOGS | Debug cap dropped; spawn `debug_tag` empty; no `MkDebug` calls. |
| NO TRACES | No notification history past visible TTL. No tray persistence. No spotlight query log. |
| EPHEMERAL | Zero files. Overlay backing in private anonymous mmap that vanishes on exit. |
| NOT LINUX | NONOS Mk-tag syscall ABI; wire is NCMP-style, not D-Bus or systemd-shaped. |
| PRIVACY MICROKERNEL | 5-bit cap mask. A compromise stays bounded to the overlay surface + peer IPC channels. |

## Runtime lifecycle

1. `_start` initializes heap.
2. `setup::prime::run()`:
   - `peers::resolve()` finds compositor/wm/wallpaper/market.
   - `overlay::allocate()` learns display dims, mmaps the backing.
   - `register::register_overlay()` publishes surface + scene_submit at z=1.
   - Healthchecks each peer; sets wallpaper policy.
3. Server loop drains IPC and re-paints overlay on damage.

## Failure model

- Peer lookup fails at startup → exit with typed error; init must
  bring peers up first (compositor → wm → wallpaper → desktop_shell).
- `mk_surface_register` / `mk_surface_share` rejected → exit cleanly.
- Tray table full → `E_NOMEM`; app retries.
- Notification flood → bounded queue; oldest dropped.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + IPC loop | `server/runner.rs` |
| Per-op handlers | `server/handlers/*.rs` |
| Tray table | `state/tray/{mod,entry,table}.rs` |
| Spotlight state | `state/spotlight.rs` |
| Context | `state/context.rs` |
| Setup peers resolution | `setup/prime/peers.rs` |
| Overlay mmap | `setup/prime/overlay.rs` |
| Surface register + scene_submit | `setup/prime/register.rs` |
| Setup driver | `setup/prime/run.rs` |
| Compositor client | `compositor_client/*.rs` |
| wm / wallpaper / market clients | `wm_client/mod.rs`, `wallpaper_client/mod.rs`, `market_client/mod.rs` |
| Render (chrome paint) | `render/*.rs` |
| Wire protocol | `protocol/*.rs` |

## Wire format

20-byte NCMP-style header followed by typed payload. Magic identifies
NONOS Desktop Shell Protocol.

## State ownership

`Context` owns: peer ports, display dims, overlay backing, tray
table, spotlight state, last notify level, next request id.

## Operating rules

- No inline comments past the 15-line license header.
- No `unsafe` past `_start` and `mk_mmap`.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where non-trivial; `mod.rs` re-exports only.

## Release target

x86_64-nonos-user.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-desktop-shell`
must compile clean.

## Release checklist

- [x] Every file ≤ 75 LOC
- [x] 15-line license header on every file
- [x] `Capsule.mk` mask `0x1819`
- [x] Spawn matches mask (no Debug)
- [x] Kernel mirror at `src/userspace/capsule_desktop_shell/`
- [x] Cert + manifest baked
- [x] Spawn wired
- [x] README documents 16 sections + Privacy Posture
- [ ] QEMU spawn-verify (blocked by OVMF #PF)

## Explicit non-goals today

- No spotlight search across capsules (deferred).
- No tray icons beyond text label (deferred until toolkit images).
- No notification persistence across restart.
- No global hotkey table.

## Verification

- `nonos-ci/run-static-checks.sh` clean (desktop shell policy
  ownership markers live in userland; render path routes through
  compositor IPC; kernel source free of desktop-shell state markers).
- `make nonos-mk-host-trust-verify` verifies
  the baked `desktop_shell.manifest.bin`.
- Kernel cargo check matrix passes with `nonos-capsule-desktop-shell`.
