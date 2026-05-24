# capsule_power

## Role

`capsule_power` is the userland power-management service. It exposes
reboot and shutdown to other capsules via IPC. Reboot lands real on
every x86 box (ACPI reset register if present, 8042 reset, triple
fault as last resort). Shutdown is honest about the AML evaluator
gap: it returns `E_NOTSUP` until a real AML interpreter can evaluate
the DSDT `_S5` method to obtain SLP_TYPa.

```text
any capsule
    |
    | OP_REBOOT / OP_SHUTDOWN
    v
capsule_power -> mk_admin_reboot / mk_admin_shutdown
    |
    v
kernel admin_ops dispatcher
    |
    v
arch::x86_64::acpi::power_reboot::reboot()  (real, three-stage fallback)
arch::x86_64::acpi::power_sleep::shutdown() (returns ENOTSUP; AML required)
```

## Microkernel contract

- `MkIpcRecv` on port `4448` reads power requests.
- `MkIpcSend` returns each status.
- `AdminReboot` syscall invokes the kernel reboot path.
- `AdminShutdown` syscall invokes the kernel shutdown path (returns
  `-95` until AML evaluator lands).
- `MkTimeMillis` reads the wall clock to track last-request timestamps.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_REBOOT` | 0x0002 | invoke `AdminReboot`; the system reboots |
| `OP_SHUTDOWN` | 0x0003 | invoke `AdminShutdown`; returns `E_NOTSUP` until AML lands |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x219`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x01 | CoreExec | run user code |
| 0x08 | IPC | recv + reply on port 4448 |
| 0x10 | Memory | bounded reply buffer |
| 0x200 | Admin | invoke `AdminReboot` / `AdminShutdown` |

`Debug` is **deliberately absent** — power transitions must never
leak to the serial surface.

## Privacy posture

| Invariant | How `capsule_power` honors it |
|---|---|
| NO LOGS | Debug cap dropped; no `MkDebug` calls. |
| NO TRACES | The capsule keeps a single `last_reboot_request_unix` and `last_shutdown_request_unix` for the current process lifetime only. Nothing persists across reboot. |
| EPHEMERAL | Zero files. |
| NOT LINUX | NCMP-style wire, Mk-tag syscall ABI. No `init(8)` semantics, no SysV runlevels. |
| PRIVACY MICROKERNEL | 4-bit cap mask. Admin is gated by `caps.can_admin()` at the kernel dispatch; no other capsule has Admin in its mask. A compromise of every other capsule cannot reboot or shutdown the box without going through this capsule's IPC. |

## Runtime lifecycle

1. `_start` initializes the heap.
2. `server::run()` enters the IPC loop on port `4448`.
3. Each request:
   - Healthcheck → status 0.
   - Reboot → record timestamp, reply with status 0, then issue
     `mk_admin_reboot()`. The reply lands first so the caller can
     audit the response before the box dies.
   - Shutdown → call `mk_admin_shutdown()`. Returns `-95` until AML
     evaluator wired.

## Failure model

- Heap init failure → exit `1`.
- Reboot: the kernel handler is real and fires regardless of which
  fallback stage triggers (ACPI reset reg / 8042 / triple fault).
  Worst case the box hard-resets; nothing is silently swallowed.
- Shutdown: returns the kernel errno verbatim. The caller can
  distinguish "AML not available" (`-95` ENOTSUP) from other failure
  modes.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + heap init | `main.rs` |
| Wire protocol | `protocol/*.rs` (decode, encode, header, ops, errno, mod) |
| Server loop | `server/runner.rs` |
| Reply builder | `server/respond.rs` |
| Per-op handlers | `server/handlers/{health,reboot,shutdown,router}.rs` |
| State (last request timestamps) | `state/mod.rs` |

## Wire format

20-byte NCMP-style header (magic `0x504F5752` = `'POWR'` LE,
version 1) followed by typed payload.

## State ownership

`PowerState` carries two `u64` last-request timestamps. No other
mutable state.

## Operating rules

- No inline comments past the 15-line license header.
- No `unsafe` past `_start`.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where non-trivial; `mod.rs` re-exports only.

## Release target

x86_64-nonos-user.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-power`
must compile clean.

## Release checklist

- [x] Every file ≤ 75 LOC
- [x] 15-line license header on every file
- [x] `Capsule.mk` mask `0x219` includes Admin
- [x] `Admin` cap is gated on `caps.can_admin()` in
      `src/syscall/contract/cap_table/admin.rs`
- [x] Kernel handler `admin_ops::handle` calls `power_reboot::reboot()`
- [ ] AML evaluator (multi-week project): when shipped, replace
      `power_sleep::shutdown` body to evaluate `_S5` from DSDT
- [ ] Kernel mirror at `src/userspace/capsule_power/`
- [ ] Spawn wired through `src/userspace/init/spawn_plan/`

## Explicit non-goals today

- ACPI suspend (S3) requires AML for `_S3` package + GPE wakeup
  configuration. Out of scope until AML lands.
- CPU frequency scaling, thermal management, battery monitoring — all
  require ACPI table parsing beyond what is shipped.

## Verification

- Reboot in QEMU: `system_powerdown` from the QEMU monitor; the
  kernel reboot path takes effect within milliseconds of any
  capsule calling `OP_REBOOT`.
- The kernel `power_reboot::reboot` has three independent fallback
  stages (ACPI reset reg, 8042, triple fault) so the reboot is
  unconditional regardless of board quirks.
