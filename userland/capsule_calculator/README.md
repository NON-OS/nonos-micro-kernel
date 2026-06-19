# capsule_calculator

## Role

`capsule_calculator` is the userland calculator application. It runs as a
CPL=3 capsule, mediates every pixel through the toolkit + compositor IPC
path, and offers a 5-column / 6-row keypad covering memory, function,
number, operator and equals roles. Arithmetic is `i128` fixed-point with
8 fractional digits, large enough to hold `±1.7e30` with full saturating
overflow detection. All UI policy lives in the capsule; the kernel
mediates only window registration, input delivery and surface
presentation.

```text
calculator app
    |
    | window registration + paint buffer + input subscribe
    v
toolkit (window kind, paint buffer, key + pointer router)
    |
    `-- compositor (scene + scanout) --- driver.virtio_gpu
```

## Microkernel contract

- `MkIpcCall` requests window registration and per-frame paint buffers via
  the toolkit endpoint.
- `MkIpcRecv` waits on the app event inbox at port `4720`.
- `MkSurfaceRegister` / `MkSurfaceAttach` / `MkSurfacePresent` route the
  paint buffer to the compositor (through the toolkit).
- `MkExit` is the only termination path.

## Interface contract

| Call | Purpose |
|---|---|
| `MkIpcCall` toolkit `4610` | register window, request paint buffer |
| `MkIpcRecv` on `4720` | receive input events from the toolkit |
| `MkSurfacePresent` | flush the paint buffer to the compositor |
| `MkExit` | terminate when the user presses Esc |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x1819`, which decodes to
exactly:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | toolkit calls + event recv |
| 0x0010 | Memory | mmap the paint buffer |
| 0x0800 | GraphicsDisplayQuery | learn display dimensions |
| 0x1000 | GraphicsSurfaceCreate | register the paint surface |

`Debug` is **deliberately absent** — the NO LOGS / NO TRACES posture
refuses any serial surface to the capsule and the capsule emits no
`MkDebug` markers anywhere. No `Driver`, `Mmio`, `Irq`, `Dma`, `Pio`,
`Network`, `Crypto`, `FileSystem`, `Hardware`, `Admin` or
`RegisterService` capability is requested.

## Privacy posture

| Invariant | How `capsule_calculator` honors it |
|---|---|
| NO LOGS | Debug cap dropped from the mask; no `MkDebug` call in any file; `debug_tag` field in the kernel spawn spec is the empty string. |
| NO TRACES | No persistent identifier. No history. No recent-calculations log. Every operand vanishes the moment the user presses AC or the capsule exits. |
| EPHEMERAL | Zero files read, zero files written, zero sockets, zero registered IPC service endpoints. State is reconstructed every paint from the live `State` struct that itself lives only in process memory. |
| NOT LINUX | NONOS Mk-tag syscall ABI; no `errno`, no `fd`. Capability names follow the NONOS taxonomy. |
| PRIVACY MICROKERNEL | Capability mask is the minimal viable surface (5 bits). Kernel rejects any syscall outside the granted mask, so a compromise of the capsule cannot reach disks, sockets, devices, or other capsules. |

## Runtime lifecycle

1. `_start` initializes the userland heap via `nonos_app_skeleton::run`.
2. The skeleton calls `manifest()` once to register the window.
3. The skeleton drives a paint pass via `paint(&self.state, fb)` whenever
   the compositor requests a new frame.
4. The skeleton delivers each input event via `on_event(state, event)`,
   which dispatches to `event::on_key` or `event::on_pointer_button`.
5. Pressing Esc returns `EventOutcome::Close` and the capsule exits via
   `MkExit`.

## Failure model

- Heap init failure → exit status `1`.
- Window registration failure → toolkit returns a typed error and the
  skeleton exits cleanly.
- `ErrorKind::DivByZero` → display switches to `Error` (red phosphor) and
  every further input is ignored until `AC` clears the state.
- `ErrorKind::DomainError` (sqrt of negative) → same `Error` display.
- `ErrorKind::Overflow` (i128 product / sum exceeds bounds) → same
  `Error` display.
- Memory operations refuse to corrupt: `M+` / `M-` set the error state
  and do not store on overflow.

## Current implemented surface

| Concern | File |
|---|---|
| App harness | `calc/app.rs` |
| Window manifest | `calc/manifest.rs` |
| Theme (NONOS phosphor green palette) | `calc/theme.rs` |
| Grid + display geometry | `calc/layout.rs` (with `hit_test`) |
| Fixed-point primitives | `calc/fixed.rs` |
| Binary operations | `calc/op.rs` |
| Unary operations (sqrt, square, reciprocal) | `calc/unary.rs` |
| Number → display formatting | `calc/format/*.rs` (5 files) |
| Button grid + roles + actions | `calc/buttons/*.rs` (8 files) |
| State (display, operand, memory, error) | `calc/state.rs` |
| Event router | `calc/event/router.rs` |
| Key classifier (digits + ops + memory + Esc) | `calc/event/key_classifier.rs` |
| Per-event handlers | `calc/event/on_key.rs`, `on_pointer_button.rs` |
| Action dispatcher | `calc/actions/dispatch.rs` |
| Per-action handlers | `calc/actions/*.rs` (16 files) |
| Frame composition | `calc/paint/frame.rs` |
| Background, wordmark, display, memory badge, grid, button | `calc/paint/*.rs` (7 files) |

## Wire format

The capsule speaks the standard toolkit NCMP wire surface as defined in
`abi/wire.toml`. It does not introduce any private wire types.

## State ownership

`State` (`calc/state.rs`) owns: `display: Fixed`, `operand: Fixed`,
`operator: Op`, `memory: Fixed`, `new_input: bool`,
`decimal_digits_typed: u8`, `error: ErrorKind`. There is no shared
static state, no cross-thread state, and no IPC-visible state.

## Operating rules

- No inline comments anywhere outside the 15-line license header.
- No `unsafe` blocks (the `_start` extern is unavoidable; nothing else).
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where the function is non-trivial; `mod.rs`
  carries re-exports only.
- All arithmetic uses `checked_*` / `saturating_*` — overflow turns into
  a typed `ErrorKind`, never wraps or panics.

## Release target

x86_64-nonos-user. Cross-compiled with the kernel-pinned nightly
toolchain under `userland/x86_64-nonos-user.json`. `aarch64-nonos-user`
and `riscv64-nonos-user` are architecture-ready but not yet validated
for this capsule.

## Release evidence

The kernel `cargo check --features microkernel-core,nonos-production,nonos-capsule-calculator`
must compile clean. The capsule's own
`cd userland/capsule_calculator && cargo build --release --target ../x86_64-nonos-user.json`
must produce a signed ELF whose SHA matches the embedded manifest
`nonos-data/trust/capsules/calculator.manifest.bin`.

## Release checklist

- [x] One function per file or ≤ 75 LOC per file (max file 60 LOC)
- [x] 15-line license header on every file
- [x] No inline comments past the license header
- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS`, slug, handle, endpoints
- [x] Capability mask audited (`0x1819` = 5 bits, no Debug)
- [x] Kernel mirror at `src/userspace/capsule_calculator/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/apps.rs`
- [x] README documents all 16 contract sections
- [x] NONOS phosphor-green palette + `NONOS calc` wordmark
- [x] Mouse hit-test on every button cell (5×6 grid)
- [x] Keyboard: digits, `.`, +, -, *, /, =, Enter, C, Backspace, Esc, %, n, r/q/i, m/M, a/s/l for memory ops
- [x] i128 fixed-point with 8 fractional digits (max ±1.7e30)
- [x] Memory register (M+, M-, MR, MC, MS) with overflow detection
- [x] sqrt, square, reciprocal unary ops with domain + overflow errors
- [x] Memory badge (amber `M`) shown when the register is non-zero
- [x] Error display (red phosphor `Error`) on div-by-zero, sqrt of
  negative, or overflow
- [ ] QEMU spawn-verify with `OP_HEALTHCHECK` reply on serial
  (blocked by the OVMF ExitBootServices `#PF` boot escalation)

## Explicit non-goals today

- No scientific functions beyond sqrt / square / reciprocal. No trig, no
  log/ln/exp, no x^y. A scientific mode tab is deferred until the toolkit
  exposes a tabbed-window widget.
- No copy/paste of the display value. Needs a libc binding for
  `capsule_clipboard`; deferred until that binding lands.
- No expression history. Intentional — keeping the calculator stateless
  across operations matches the EPHEMERAL posture.
- No theme pull from `nonos_toolkit::theme::snapshot()`. The local
  phosphor palette matches the NONOS brand and the toolkit theme is
  static today.

## Verification

- `nonos-ci/run-static-checks.sh` clean (per-capsule one-function-per-file
  enforcement, capability mask, README contract sections).
- `make nonos-mk-host-trust-verify` verifies
  the baked `calculator.manifest.bin` against the trust anchor.
- Kernel cargo check matrix passes with `nonos-capsule-calculator` on
  top of `microkernel-core,nonos-production`.
