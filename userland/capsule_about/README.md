# capsule_about

## Role

`capsule_about` is the userland "About this system" application. It owns five
sections of product, authority, display and license metadata; the user cycles
sections with Tab/Shift-Tab, clicks a tab to jump directly, and scrolls long
sections (like the full AGPL-3 license) with Up/Down or Page Up/Page Down.
Esc closes the window. All UI policy lives in the capsule; the kernel
mediates only window registration, input delivery and surface presentation
through the toolkit and the compositor.

```text
about app
    |
    | window registration + paint buffer + input subscribe
    v
toolkit (window kind, paint buffer, key router)
    |
    `-- compositor (scene + scanout) --- driver.virtio_gpu
```

## Microkernel contract

- `MkIpcCall` requests window registration and per-frame paint buffers via
  the toolkit endpoint.
- `MkIpcRecv` waits on the app event inbox at port `4710`.
- `MkSurfaceRegister` / `MkSurfaceAttach` / `MkSurfacePresent` route the
  paint buffer to the compositor (through the toolkit).
- `MkTimeMillis` reads the monotonic wall clock for the Uptime section.
- `MkExit` is the only termination path.

## Interface contract

| Call | Purpose |
|---|---|
| `MkIpcCall` toolkit `4610` | register window, request paint buffer |
| `MkIpcRecv` on `4710` | receive input events from the toolkit |
| `MkSurfacePresent` | flush the paint buffer to the compositor |
| `MkTimeMillis` | read the wall clock for the Uptime section |
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

`MkTimeMillis` requires no extra capability bit (it is part of the CoreExec
microkernel surface). `Debug` is **deliberately absent** — the NO LOGS /
NO TRACES posture refuses any serial surface to the capsule and the capsule
emits no `MkDebug` markers anywhere in its source. No `Driver`, `Mmio`,
`Irq`, `Dma`, `Pio`, `Network`, `Crypto`, `FileSystem`, `Hardware`, `Admin`
or `RegisterService` capability is requested.

## Privacy posture

This capsule is designed under the NONOS first-principles invariants:

| Invariant | How `capsule_about` honors it |
|---|---|
| NO LOGS | `Debug` cap dropped from the mask; no `MkDebug` call in any file; `debug_tag` field in the kernel spawn spec is the empty string so even spawn-time errors do not leak a label. |
| NO TRACES | Build timestamp removed from the embedded info; only the SHA + version remain (integrity, not forensic). The Uptime section displays wall-clock since-boot only — there is no persistent identifier of any kind in the binary or in runtime state. |
| EPHEMERAL | The capsule reads zero files, writes zero files, opens zero sockets, and registers zero IPC service endpoints. State is reconstructed from compile-time tables on every paint. |
| NOT LINUX | No POSIX shapes anywhere. Capability names follow the NONOS taxonomy (`CoreExec`, `GraphicsDisplayQuery`). Syscalls are the 4-byte ASCII tag form (`MMAP`, `MTMS`). No `errno`, no `fd`. |
| PRIVACY MICROKERNEL | Capability mask is the minimal viable surface (5 bits). Refuses `Network`, `FileSystem`, `Crypto`, `Hardware`, `Admin`, `RegisterService`. The kernel rejects any syscall outside the granted mask, so a compromise of the capsule cannot reach disks, sockets, devices, or other capsules. |

## Privacy and persistence

The capsule reads no user data and writes no user data. No configuration
file, no telemetry, no persistent UI state. The Display section reads
display dimensions live; the Uptime section reads the monotonic wall clock
live; everything else is compile-time-static from `about/data/*`.

## Runtime lifecycle

1. `_start` initializes the userland heap via `nonos_app_skeleton::run` and
   constructs the `About` value.
2. The skeleton calls `manifest()` once to register the window.
3. The skeleton drives a paint pass via `paint(state, fb)` whenever the
   compositor requests a new frame; the body dispatches to the current
   `Section`.
4. The skeleton delivers each input event via `on_event(state, event)` which
   dispatches into the per-key handlers under `about/event/`.
5. Pressing Esc returns `EventOutcome::Close` and the capsule exits cleanly
   through `MkExit`.

## Failure model

- Heap init failure → exit status `1` (caught inside
  `nonos_app_skeleton::run`).
- Window registration failure → toolkit returns a typed error, the skeleton
  exits with status `2`; no partial state is left in the compositor.
- Surface attach failure during paint → the skeleton drops the frame, marks
  the surface dead, and re-registers on the next paint tick.
- Display dimensions query failure → the Display section renders the literal
  string `unavailable` instead of a fake value.
- Wall clock unavailable (TSC not yet calibrated) → the Uptime section
  renders `unavailable` instead of a zero.

## Current implemented surface

| Section | Source | Lines |
|---|---|---|
| Identity | `data/{product,build,abi}` (compile-time) | 9 |
| Authority | `data/{caps,trust}` (compile-time + decoded mask) | 6 + 6 + 1 + 15 |
| Display | `data/display::primary_dimensions()` (live) | 4 |
| Uptime | `data/uptime::read_millis() + split_dhms()` (live) | 5 |
| License | `data/license::TEXT` (full AGPL-3 via `include_str!`) | 4 + 661 |

| Concern | File |
|---|---|
| App harness | `about/app.rs` |
| Window manifest | `about/manifest.rs` |
| Section enum | `about/section.rs` |
| State + scroll/section cursor | `about/state.rs` |
| Theme colors + metrics | `about/theme.rs` |
| Event router | `about/event/router.rs` |
| Per-key handlers | `about/event/on_*.rs` (9 key handlers: esc, tab, shift-tab, arrow up/down, page up/down, home, end) |
| Pointer handler | `about/event/on_pointer_button.rs` (tab strip hit-test) |
| Frame composition | `about/paint/frame.rs` |
| Header band | `about/paint/header.rs` |
| Tab strip | `about/paint/tabs.rs` |
| Body dispatcher | `about/paint/body.rs` |
| Scrollbar | `about/paint/scrollbar.rs` |
| Status bar | `about/paint/status_bar.rs` |
| Section renderers | `about/section_render/*.rs` (6 files) |
| Data sources | `about/data/*.rs` (8 files) |
| Number formatting | `about/format.rs` |

## Wire format

The capsule speaks the standard toolkit NCMP wire surface as defined in
`abi/wire.toml`. It does not introduce any private wire types.

## State ownership

`State` (`about/state.rs`) owns: the current `Section`, the `scroll` line
index within that section, and a `painted: bool` first-frame flag. There is
no shared static state, no cross-thread state, and no IPC-visible state.

## Operating rules

- No inline comments anywhere outside the 15-line license header.
- No `unsafe` blocks.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!` anywhere in the
  capsule.
- Every file ≤ 75 LOC.
- One function per file where the function is non-trivial; `mod.rs` carries
  re-exports only.
- No hardcoded version: `data/build.rs` baked from `build.rs` env vars
  (commit SHA, build timestamp, package version).

## Release target

x86_64-nonos-user. Cross-compiled with the kernel-pinned nightly toolchain
under `userland/x86_64-nonos-user.json`. `aarch64-nonos-user` and
`riscv64-nonos-user` are architecture-ready but not yet validated for this
capsule.

## Release evidence

The kernel `cargo check --features microkernel-core,nonos-production,nonos-capsule-about`
must compile clean. The capsule's own
`cd userland/capsule_about && cargo build --release --target ../x86_64-nonos-user.json`
must produce a signed ELF whose SHA matches the embedded manifest
`nonos-data/trust/capsules/about.manifest.bin`.

## Release checklist

- [x] One function per file or ≤ 75 LOC per file
- [x] 15-line license header on every file
- [x] No inline comments past the license header
- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS`, slug, handle, endpoints
- [x] Capability mask audited (`0x1919` decodes correctly)
- [x] Kernel mirror at `src/userspace/capsule_about/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/apps.rs`
- [x] README documents all 16 contract sections
- [x] Build info embedded at compile time (git SHA + unix timestamp, fallback to `NONOS_BUILD_SHA` / `GITHUB_SHA` env vars in CI)
- [x] Live data sources for Display and Uptime sections
- [x] Full AGPL-3 text embedded via `include_str!`, scrollable in the License section
- [x] Cap mask auto-decoded from single `ALL_CAPS` table (no hand-maintained grant/deny lists)
- [x] Mouse support — click any tab to jump to its section
- [x] Saturating arithmetic in scrollbar and section iteration (no over/underflow class)
- [x] Trust section wording grounded in fact ("reached _start" implies `spawn_verified` passed)
- [x] Home/End keys jump to top/bottom of section
- [x] Window-size-aware visible-line count (no hardcoded `14`; recomputed every paint from `fb.height`)
- [x] Section breadcrumb (e.g. `2 / 5`) rendered in the header
- [x] Architecture string in Identity section (cfg-gated per target)
- [x] Privacy posture (NO LOGS / NO TRACES / EPHEMERAL) explicitly enforced: Debug cap dropped, build timestamp removed
- [ ] QEMU spawn-verify with `OP_HEALTHCHECK` reply on serial
  (blocked by the OVMF ExitBootServices `#PF` boot escalation)

## Explicit non-goals today

- No HTTP/network access. The capsule never opens a socket.
- No file IO. The capsule never reads from VFS.
- No CPU/RAM/process telemetry. Those require kernel syscalls not yet in
  the libc surface (cpuid, mem stats, process list); will be added before
  the Identity section grows a hardware/process panel.
- No clipboard yet. Ctrl-C to copy the commit SHA would need a libc
  binding for `capsule_clipboard`; deferred until that binding lands.
- No toolkit theme pull. `nonos_toolkit::theme::snapshot()` is a static
  palette today and changes nothing; will be wired once the theme grows a
  live observer.
- No internationalization. Strings are ASCII byte literals.

## Verification

- `nonos-ci/run-static-checks.sh` clean (per-capsule one-function-per-file
  enforcement, capability mask, README contract sections).
- `cd nonos-sign && cargo test --release --test artifacts` round-trips the
  baked `about.manifest.bin` against the trust anchor.
- Kernel cargo check matrix passes with `nonos-capsule-about` on top of
  `microkernel-core,nonos-production`.
