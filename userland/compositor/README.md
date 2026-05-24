# compositor

## Role

`compositor` is the userland window compositor. It owns the scene graph
(per-window layer table), damage tracking, the frame pacer, the
software blitter, and the IPC client into `driver.virtio_gpu0`. Every
pixel that reaches the GPU goes through this capsule; the kernel never
touches the framebuffer past the surface registry hand-off.

```text
windowing apps (terminal, wm, calculator, ...)
    |
    | scene_submit / damage_commit / focus_set / cursor_update
    v
compositor (this capsule)
    |
    | mk_surface_attach + gfx_client (transfer_to_host / set_scanout / flush)
    v
driver.virtio_gpu0 --> virtio_gpu device --> display
```

## Microkernel contract

- `MkIpcRecv` on port `4310` reads scene/damage/focus/cursor requests
  from windowing capsules.
- `MkSurfaceRegister` / `MkSurfaceAttach` map each window's paint
  buffer into this capsule's address space exactly once.
- `MkSurfacePresent` and the `gfx_client` (which calls into
  `driver.virtio_gpu0` via IPC) drive the actual scanout.
- `MkDisplayVsyncWait` blocks the frame pacer until the GPU is ready
  for the next frame.
- `MkYield` and `MkExit` complete the cooperative loop and the clean
  termination path.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SCENE_SUBMIT` | 0x0002 | windowing capsule registers / updates a layer |
| `OP_SCENE_REMOVE` | 0x0003 | windowing capsule removes its layer |
| `OP_DAMAGE_COMMIT` | 0x0004 | mark a rect dirty on the calling layer |
| `OP_FOCUS_SET` | 0x0005 | request input focus for a window id |
| `OP_CURSOR_UPDATE` | 0x0006 | move + resize the hardware cursor |
| `OP_INPUT_SUBSCRIBE` | 0x0007 | subscribe a windowing capsule to input events |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x7819`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | recv on port 4310 + send to driver.virtio_gpu0 |
| 0x0010 | Memory | allocate scene + damage + attach cache |
| 0x0800 | GraphicsDisplayQuery | learn display dimensions |
| 0x1000 | GraphicsSurfaceCreate | register the primary scanout surface |
| 0x2000 | GraphicsSurfaceMap | map each window's paint buffer into this AS |
| 0x4000 | GraphicsPresent | drive `MkSurfacePresent` and vsync wait |

`Debug` is **deliberately absent** — the NO LOGS / NO TRACES posture
refuses any serial surface and the capsule emits no `MkDebug` markers
during steady-state operation. No `Driver`, `Mmio`, `Irq`, `Dma`,
`Pio`, `Network`, `Crypto`, `FileSystem`, `Hardware`, `Admin`,
`RegisterService` capability is requested. The compositor is policy +
arithmetic, not a hardware actor.

## Privacy posture

| Invariant | How `compositor` honors it |
|---|---|
| NO LOGS | Debug cap dropped; `debug::marker` is no-op when not in capture mode; spawn `debug_tag` is the empty string. |
| NO TRACES | No persistent identifier. Scene + damage + focus + cursor live only in process memory; the moment the compositor exits the entire scene graph vanishes. No frame is captured to disk. |
| EPHEMERAL | Zero files. The attach cache (`state/attach.rs`) only caches paint-buffer VAs for the lifetime of the surface handle. |
| NOT LINUX | NONOS Mk-tag syscall ABI. The wire format is NCMP-style (NONOS Compositor Message Protocol) with magic and version, not POSIX/Wayland-shaped. |
| PRIVACY MICROKERNEL | Cap mask is 7 bits, all graphics-policy scoped. No Network/FileSystem/Crypto path means a compromise of the compositor stays bounded to scene state + the gfx_client IPC channel; it cannot read user files, open sockets, or reach other apps' AS. |

## Runtime lifecycle

1. `_start` initializes the userland heap and constructs `Context`
   (scene, damage, attach cache, focus, cursor, frame counter).
2. The setup phase resolves the gfx endpoint via `MkServiceLookup`
   and registers the primary scanout surface with the virtio_gpu
   capsule.
3. The main loop alternates between draining IPC (every request from
   any windowing capsule) and ticking the frame pacer (composing damaged
   regions, blitting to the primary surface, calling
   `gfx_client::transfer_to_host` + `gfx_client::flush`).
4. `MkDisplayVsyncWait` (or `MkYield` fallback) gates the next tick.

## Failure model

- gfx endpoint lookup fails at startup → exit with a typed error.
- `mk_surface_attach` on an incoming surface_handle fails → drop the
  scene_submit; the offending window is invisible until it re-submits.
- `gfx_client::transfer_to_host` or `flush` fails → mark
  `scanout_error_reported`, emit one proof marker (only in
  capture-mode debug builds), and continue. The compositor never
  panics — a stuck GPU degrades the scene but does not crash the
  display server.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + IPC drain loop | `server/runner/{entry,drain,dispatch}.rs` |
| Per-op handlers | `server/handlers/{health,scene_submit,scene_remove,damage_commit,focus_set,cursor_update,input_subscribe}.rs` |
| Reply builder | `server/respond.rs` |
| Scene layer table + z-sort snapshot | `state/scene/{layer,table,snapshot}.rs` |
| Damage accumulator | `state/damage.rs` |
| Focus table | `state/focus.rs` |
| Cursor tracker | `state/cursor.rs` |
| Attach cache (paint buffer VA per surface_handle) | `state/attach.rs` |
| Context (owned by runner) | `state/context.rs` |
| Frame pacer (compose + flush + vsync) | `frame_pacer/{tick,compose,vsync}.rs` |
| gfx_client (driver.virtio_gpu0 IPC) | `gfx_client/{get_primary_surface,transfer_to_host,set_scanout,flush}.rs` |
| Software blitter | `sw_blitter/*.rs` |
| Wire protocol (header/ops/errno/limits) | `protocol/*.rs` |
| Setup (lookup gfx + register primary) | `setup/{discover,prime}.rs` |
| Debug proof markers (capture-only) | `debug.rs` |

## Wire format

20-byte NCMP-style header followed by typed payload. All multi-byte
fields little-endian. Magic `0x434F_4D50` (`'COMP'` LE — NONOS
Compositor Message Protocol).

## State ownership

`Context` (`state/context.rs`) owns: gfx_port, resource_id, width,
height, stride, backing_va, first_scanout_done, scanout_error_reported,
next_request_id, scene, damage, focus, cursor, attach. There is no
shared static state. Per-window state is keyed by the caller's pid.

## Operating rules

- No inline comments past the 15-line license header.
- No `unsafe` past the unavoidable surface mapping path.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where the function is non-trivial; `mod.rs`
  carries re-exports only.

## Release target

x86_64-nonos-user. Cross-compiled with the kernel-pinned nightly
toolchain.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-compositor`
must compile clean. The capsule's own
`cd userland/compositor && cargo build --release --target ../x86_64-nonos-user.json`
must produce a signed ELF whose SHA matches the embedded manifest
`nonos-data/trust/capsules/compositor.manifest.bin`.

## Release checklist

- [x] Every file ≤ 75 LOC (max is 73)
- [x] 15-line license header on every file
- [x] No inline comments past the license header
- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS = 0x7819`
- [x] Capability mask audited (7 bits, no Debug)
- [x] Kernel mirror at `src/userspace/capsule_compositor/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/`
- [x] README documents all 16 contract sections
- [x] Boot-framebuffer fallback removed; virtio-only present path
- [x] Saturating arithmetic in scrollbar/scene math (no underflow class)
- [ ] QEMU spawn-verify with `OP_HEALTHCHECK` reply on serial
  (blocked by the OVMF ExitBootServices `#PF` boot escalation)

## Explicit non-goals today

- No GPU 3D acceleration. The compositor is a 2D blitter; 3D will land
  when virtio-gpu virgl exposure is wired and signed.
- No tear-free triple-buffering. Single-buffered with vsync wait.
- No transparency/compositing effects beyond alpha-blended copy.
- No multi-monitor scanout beyond `VG_MAX_SCANOUTS=1` in the driver.

## Verification

- `nonos-ci/run-static-checks.sh` clean (compositor scene/damage/cursor
  through Mk* + gfx_client path, no legacy `nonos_surface_*` calls).
- `cd nonos-sign && cargo test --release --test artifacts` round-trips
  the baked `compositor.manifest.bin` against the trust anchor.
- Kernel cargo check matrix passes with `nonos-capsule-compositor` on
  top of `microkernel-core,nonos-production`.
