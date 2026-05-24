# capsule_clipboard

## Role

`capsule_clipboard` is the userland clipboard service. It runs as a CPL=3
capsule holding a bounded FIFO of recently-copied entries, addressable by
content type. Any other capsule that needs cut/copy/paste talks to it
over IPC; the kernel never carries clipboard bytes itself. Idle entries
self-clear after a configurable timeout so a left-open window does not
leak its last selection forever.

```text
copying capsule (terminal, text_editor, ...)
    |
    | OP_COPY  (content_type + bytes)
    v
capsule_clipboard (this capsule)
    ^   |
    |   |  OP_PASTE / OP_HISTORY_LIST / OP_HISTORY_GET
    |   v
pasting capsule (terminal, text_editor, ...)
```

## Microkernel contract

- `MkIpcRecv` on port `4414` waits for incoming requests.
- `MkIpcSend` on port `4414` (reply channel) returns responses.
- `MkTimeMillis` reads the monotonic wall clock to drive the idle-timeout
  privacy invariant.
- `MkYield` backs off when no request is pending.
- `MkExit` is the only termination path.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping, returns status only |
| `OP_COPY` | 0x0002 | push a `content_type:u32 + bytes` entry to the head |
| `OP_PASTE` | 0x0003 | recall the most-recent entry of a given content type |
| `OP_HISTORY_LIST` | 0x0004 | list `(content_type, len)` pairs of all entries |
| `OP_HISTORY_GET` | 0x0005 | recall the entry at a given index |
| `OP_CLEAR` | 0x0006 | wipe every entry immediately |
| `OP_SET_IDLE_TIMEOUT` | 0x0007 | set idle auto-clear timeout (0 disables) |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x19`, which decodes to
exactly:

| Bit | Capability | Purpose |
|---|---|---|
| 0x01 | CoreExec | run user code |
| 0x08 | IPC | recv/send on port `4414` |
| 0x10 | Memory | heap allocation for clipboard entries |

`MkTimeMillis` requires no extra capability bit (CoreExec covers it).
`Debug` is **deliberately absent** — the NO LOGS / NO TRACES posture
refuses any serial surface and the capsule emits no `MkDebug` markers.
No `Driver`, `Mmio`, `Irq`, `Dma`, `Pio`, `Network`, `Crypto`,
`FileSystem`, `Hardware`, `Admin`, `RegisterService` or any Graphics
capability is requested.

## Privacy posture

| Invariant | How `capsule_clipboard` honors it |
|---|---|
| NO LOGS | Debug cap dropped from the mask; no `MkDebug` call in any file; `debug_tag` in the kernel spawn spec is the empty string. |
| NO TRACES | Idle entries self-clear after the configurable timeout (default 10 min). No persistent identifier, no on-disk record, no IPC service publishes the entry list outside this capsule. |
| EPHEMERAL | Zero files. All state lives in a `VecDeque<Entry>` that vanishes on capsule exit. Total bytes bounded by `MAX_TOTAL_BYTES = 256 KiB`. |
| NOT LINUX | NONOS Mk-tag syscall ABI. The wire is the NCMP-style `MAGIC=0x43424930` header (NONOS clipboard); no POSIX shapes. |
| PRIVACY MICROKERNEL | Capability mask is the minimal 3-bit viable surface. Refuses Network, FileSystem, every Graphics/Driver cap. Any compromise stays bounded by IPC + heap with no path to disk, devices, or other capsules' memory. |

## Runtime lifecycle

1. `_start` initializes the userland heap via `nonos_libc::heap_init`.
2. The server enters `run()` which creates the bounded `Clipboard`
   structure with `MAX_DEPTH=16` entries / `MAX_TOTAL_BYTES=256 KiB` /
   `DEFAULT_IDLE_TIMEOUT_MS=600_000` (10 minutes).
3. Each loop iteration:
   - Reads current wall clock via `mk_time_millis`.
   - Calls `clipboard.expire_if_idle(now)`; if the idle timeout has
     elapsed, every entry is dropped immediately.
   - Blocks on `mk_ipc_recv` for the next request.
   - Routes the request through `handlers::route` and sends the reply.

## Failure model

- Heap init failure → exit status `1`.
- Malformed request (bad magic / version / length) → typed errno reply
  (E_BAD_MAGIC / E_BAD_VERSION / E_BAD_LEN); no state change.
- Unknown op → E_BAD_OP reply; no state change.
- Payload too large (`> MAX_ENTRY_BYTES`) → E_RANGE; entry refused.
- Response buffer too small for a paste reply → E_RANGE; entry kept,
  not delivered.
- Idle-timeout out of range on `OP_SET_IDLE_TIMEOUT` → E_RANGE; current
  timeout unchanged.

## Current implemented surface

| Concern | File |
|---|---|
| `_start` + heap init | `main.rs` |
| Wire protocol (magic/version/header) | `protocol/header.rs` |
| Op codes | `protocol/ops.rs` |
| Errno table | `protocol/errno.rs` |
| Limits + timeout bounds | `protocol/limits.rs` |
| Request decode | `protocol/decode.rs` |
| Response encode | `protocol/encode.rs` |
| Entry record | `state/entry.rs` |
| Bounded FIFO + storage ops | `state/clipboard/storage.rs` |
| Idle-clear timer | `state/clipboard/timer.rs` |
| Server loop | `server/runner.rs` |
| Reply builder | `server/respond.rs` |
| Handler router | `server/handlers/router.rs` |
| Per-op handlers | `server/handlers/{health,copy,paste,history_list,history_get,clear,set_idle_timeout}.rs` |

## Wire format

20-byte NCMP-style header followed by a typed payload. Magic
`0x4342_4930` (`'CBI0'` little-endian — NONOS ClipBoard Interface v0).
All multi-byte fields are little-endian.

Header layout (fixed at `HDR_LEN = 20`):

```text
0    4 magic        u32
4    2 version      u16
6    2 op           u16
8    2 flags        u16
10   2 reserved     u16
12   4 request_id   u32
16   4 payload_len  u32
```

Response always starts with a 4-byte `status: i32` (0 on success,
negative errno on failure). Op-specific data follows the status when
applicable (see per-handler files for layout).

## State ownership

`Clipboard` (`state/clipboard/types.rs`) owns: bounded `VecDeque<Entry>`,
the total bytes counter, depth + byte caps, last-activity timestamp, and
idle-timeout config. There is no shared static state, no cross-thread
state, and no IPC-visible state outside the documented ops.

## Operating rules

- No inline comments anywhere outside the 15-line license header.
- No `unsafe` blocks (the `_start` extern is unavoidable; nothing else).
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where the function is non-trivial; `mod.rs`
  carries re-exports only.
- All wire parsing is bounds-checked; no `try_into().unwrap()` —
  little-endian decode uses explicit indexing.

## Release target

x86_64-nonos-user. Cross-compiled with the kernel-pinned nightly
toolchain under `userland/x86_64-nonos-user.json`. `aarch64-nonos-user`
and `riscv64-nonos-user` are architecture-ready but not yet validated
for this capsule.

## Release evidence

The kernel `cargo check --features microkernel-core,nonos-production,nonos-capsule-clipboard`
must compile clean. The capsule's own
`cd userland/capsule_clipboard && cargo build --release --target ../x86_64-nonos-user.json`
must produce a signed ELF whose SHA matches the embedded manifest
`nonos-data/trust/capsules/clipboard.manifest.bin`.

## Release checklist

- [x] Every file ≤ 75 LOC (max is 60)
- [x] 15-line license header on every file
- [x] No inline comments past the license header
- [x] `Capsule.mk` with `CAPSULE_REQUIRED_CAPS`, slug, handle, endpoints
- [x] Capability mask audited (`0x19` = CoreExec + IPC + Memory, no
  Debug, no Network, no FileSystem)
- [x] Kernel mirror at `src/userspace/capsule_clipboard/`
- [x] Cert + manifest baked into `nonos-data/trust/capsules/`
- [x] Spawn wired through `src/userspace/init/spawn_plan/`
- [x] README documents all 16 contract sections
- [x] Idle-timeout privacy auto-clear (default 10 min, configurable
  via `OP_SET_IDLE_TIMEOUT`, bounded by `MIN_IDLE_TIMEOUT_MS=5s` and
  `MAX_IDLE_TIMEOUT_MS=24h`)
- [x] Bounded depth + bounded total bytes (no unbounded growth)
- [x] Typed errno surface (no silent failures)
- [x] No `try_into().unwrap()` in decode path (replaced with explicit
  bounds-checked LE decoders)
- [ ] QEMU spawn-verify with `OP_HEALTHCHECK` reply on serial
  (blocked by the OVMF ExitBootServices `#PF` boot escalation)

## Explicit non-goals today

- No persistence across capsule restart. Intentional — matches the
  EPHEMERAL posture. A "remember after reboot" pin would require an
  explicit user authorization flow and an encrypted-at-rest backing
  store; deferred.
- No multi-user separation. The capsule serves whatever caller can
  reach its port. Per-user clipboards belong in a higher-layer policy
  capsule above this one.
- No content sanitization. Bytes go in and come back unchanged; the
  caller is responsible for character-set handling.
- No drag-and-drop. That belongs in the compositor + toolkit layer,
  not in this service.

## Verification

- `nonos-ci/run-static-checks.sh` clean (per-capsule one-function-per-file
  enforcement, capability mask, README contract sections).
- `cd nonos-sign && cargo test --release --test artifacts` round-trips
  the baked `clipboard.manifest.bin` against the trust anchor.
- Kernel cargo check matrix passes with `nonos-capsule-clipboard` on
  top of `microkernel-core,nonos-production`.
