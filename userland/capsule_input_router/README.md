# capsule_input_router

## Role

`capsule_input_router` is the userland input dispatcher. It drains the
kernel input ring via `MkInputEventDrain`, normalizes each event into
the wire `InputEvent` shape, and fans it out to whichever capsule
holds focus for that input class. No driver claims live here — this
capsule is pure IPC + the `MkInputEvent*` surface.

```text
driver.ps2_input / driver.usb_hid / driver.virtio_*_input
    |
    | kernel input ring (MkInputEventPost from the driver side)
    v
capsule_input_router (this capsule)
    |
    | normalized InputEvent fanned out per focus
    v
focused window (terminal / calculator / desktop_shell / ...)
```

## Microkernel contract

- `MkInputEventDrain` reads the kernel input ring (no DMA, no IRQ
  claims — the kernel buffers events from the driver capsules and we
  drain).
- `MkIpcRecv` on port `4320` reads subscription requests from
  windowing capsules.
- `MkIpcSend` delivers each event to the focused subscriber's reply
  port.
- `MkYield` and `MkExit` complete the cooperative loop.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_SUBSCRIBE` | 0x0002 | a windowing capsule registers as the focus target |
| `OP_UNSUBSCRIBE` | 0x0003 | remove subscription |
| `OP_FOCUS_HINT` | 0x0004 | compositor pushes focus identity (pid + window id) |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x19`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x01 | CoreExec | run user code |
| 0x08 | IPC | recv on 4320 + send to focused subscriber |
| 0x10 | Memory | subscription table allocations |

`Debug` is **deliberately absent**. No Driver/Mmio/Irq/Dma/Pio means
this capsule cannot interpose on raw hardware; it only consumes the
already-brokered event ring.

## Privacy posture

| Invariant | How `capsule_input_router` honors it |
|---|---|
| NO LOGS | Debug cap dropped; spawn `debug_tag` empty; no `MkDebug` calls anywhere — keystrokes never leak to serial. |
| NO TRACES | No event history kept. Each event is drained, dispatched, dropped. No keylog file, no persistent buffer. |
| EPHEMERAL | Zero files. Bounded subscription table (`MAX_SUBSCRIBERS`) prevents unbounded growth. |
| NOT LINUX | NONOS Mk-tag syscall ABI. The wire is a NONOS Input Routing wire format, not evdev or X11-shaped. |
| PRIVACY MICROKERNEL | 3-bit cap mask. Refuses every Network/FS/Crypto/Graphics/Driver/Mmio cap. A compromise of input_router stays bounded to the in-flight event ring — it cannot persist keystrokes, exfiltrate to disk, or reach hardware. |

## Runtime lifecycle

1. `_start` initializes the heap and the subscription + focus tables.
2. Setup looks up the compositor port so focus hints are accepted.
3. Main loop:
   - Drain N events from `MkInputEventDrain`.
   - For each event, look up the current focus subscriber.
   - Send the normalized `InputEvent` to that subscriber.
   - `MkYield` between drain passes.

## Failure model

- No subscribers → events drop silently.
- Subscriber's reply port unreachable → drop the subscription, continue.
- `MkInputEventDrain` returns `<0` → backoff via `MkYield`.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + drain loop | `server/runner.rs` |
| Per-op handlers | `server/handlers/{health,subscribe,unsubscribe,focus_hint}.rs` |
| Reply builder | `server/respond.rs` |
| Event sources (drain) | `sources/{drain,normalize}.rs` |
| Routing table | `route/{focus,subscribers,dispatch}.rs` |
| State (subscriptions + focus) | `state/{mod,subscriptions,focus}.rs` |
| Wire protocol | `protocol/*.rs` |

## Wire format

20-byte NCMP-style header followed by an `InputEvent` (kind, flags,
code, x, y, delta_x, delta_y, timestamp_ns) — matches the app_skeleton
`InputEvent` shape exactly so subscribers can deserialize with no
translation.

## State ownership

`Context` owns subscriptions, focus identity, and the compositor port.
No shared static state; all routing keyed by pid.

## Operating rules

- No inline comments past the 15-line license header.
- No `unsafe` past the `_start` extern.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where non-trivial; `mod.rs` re-exports only.

## Release target

x86_64-nonos-user.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-input-router`
must compile clean.

## Release checklist

- [x] Every file ≤ 75 LOC
- [x] 15-line license header on every file
- [x] No inline comments past the license header
- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS = 0x19`
- [x] Capability mask audited (3 bits, no Debug)
- [x] Kernel mirror at `src/userspace/capsule_input_router/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/`
- [x] README documents all 16 contract sections
- [x] No keystroke ever reaches a log surface
- [ ] QEMU spawn-verify (blocked by OVMF #PF)

## Explicit non-goals today

- No IME / dead-key composition (deferred until UTF-8 in toolkit).
- No keyboard macro expansion. Belongs in a higher-layer policy capsule.
- No global hotkey table. `capsule_desktop_shell` owns global shortcuts.
- No input recording for replay. Intentional — matches NO TRACES.

## Verification

- `nonos-ci/run-static-checks.sh` clean (kernel input modules remain
  ingest-only; routing/focus/compositor policy lives in compositor
  protocol + handlers).
- `make nonos-mk-host-trust-verify` verifies
  the baked `input_router.manifest.bin` against the trust anchor.
- Kernel cargo check matrix passes with `nonos-capsule-input-router`
  on top of `microkernel-core,nonos-production`.
