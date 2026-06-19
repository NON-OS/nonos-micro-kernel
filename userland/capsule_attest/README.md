# capsule_attest

## Role

`capsule_attest` is the userland privacy-attestation service. It
returns a structural proof that the running system honors the NONOS
invariants. The capsule exists because Linux fundamentally cannot
make this claim: too many of its components run with full kernel
privilege.

```text
inquisitive capsule / external auditor
    |
    | OP_PROOF_SUMMARY / OP_PROOF_INVARIANTS / OP_PROOF_BOOT / OP_PROOF_CAPSULE_LIST
    v
capsule_attest (this capsule)  ----> returns the recorded mask + claim + mechanism
```

## Microkernel contract

- `MkIpcRecv` on port `4444` reads attestation queries.
- `MkIpcSend` returns each typed proof reply.
- `MkTimeMillis` reads the monotonic boot clock for OP_PROOF_BOOT.
- `MkYield` and `MkExit` complete the cooperative loop.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_PROOF_SUMMARY` | 0x0002 | product name + tagline + version |
| `OP_PROOF_INVARIANTS` | 0x0003 | every invariant + claim + mechanism tuple |
| `OP_PROOF_BOOT` | 0x0004 | monotonic boot ms + bootloader identity |
| `OP_PROOF_CAPSULE_LIST` | 0x0005 | every known capsule + its capability mask |

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x19`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x01 | CoreExec | run user code |
| 0x08 | IPC | recv + reply on port 4444 |
| 0x10 | Memory | bounded reply buffer |

`Debug` is **deliberately absent**. The attestation capsule loses all
credibility the moment it has any path to a log surface; that's the
entire point of the NO LOGS invariant it asserts.

## Privacy posture

This capsule is the *meta-statement* of the posture every other
capsule honors:

| Invariant | How `capsule_attest` honors it AND proves it |
|---|---|
| NO LOGS | Debug cap dropped; no `MkDebug` in any file. And the capsule itself asserts the NO LOGS invariant in `state/invariants.rs` so external auditors can verify the claim by querying. |
| NO TRACES | No persistent identifier kept. Every query reply is reconstructed from compile-time tables. |
| EPHEMERAL | Zero files. State lives only in the static tables in `state/`. |
| NOT LINUX | Mk syscall ABI; NCMP wire; NONOS cap taxonomy. The capsule asserts each of those characteristics explicitly. |
| PRIVACY MICROKERNEL | 3-bit cap mask. The capsule asserts that this is the policy across the system, and the cap mask itself proves it for `capsule_attest`. |

## Runtime lifecycle

1. `_start` initializes heap.
2. Server enters `run()` on port `4444`.
3. Each loop iteration:
   - Block on `mk_ipc_recv`.
   - Route the request via `handlers::route`.
   - Send the typed proof reply.

## Failure model

- Unknown op → `E_BAD_OP`.
- Reply would exceed buffer → `E_INVAL`, no partial state.
- Heap init failure at boot → exit `1`.

## Current implemented surface

| Concern | File |
|---|---|
| Entry + heap init | `main.rs` |
| Wire protocol | `protocol/*.rs` |
| Static invariant table | `state/invariants.rs` |
| Product identity | `state/product.rs` |
| Server loop | `server/runner.rs` |
| Reply builder | `server/respond.rs` |
| Handler router | `server/handlers/router.rs` |
| Per-op handlers | `server/handlers/{health,proof_summary,proof_invariants,proof_boot,proof_capsule_list}.rs` |

## Wire format

20-byte NCMP-style header (magic `0x41545354` = `'ATST'`, version 1)
followed by typed payload. Per-op payload format documented in the
corresponding handler file.

## State ownership

All state is compile-time-static (`state/invariants.rs`,
`state/product.rs`, `KNOWN_CAPSULES` table). The capsule has no
mutable state and no persistent identifier.

## Operating rules

- No inline comments past the 15-line license header.
- No `unsafe` past the unavoidable `_start` extern.
- No `panic!`, `unwrap`, `expect`, `todo!`, `unimplemented!`.
- Every file ≤ 75 LOC.
- One function per file where non-trivial; `mod.rs` re-exports only.

## Release target

x86_64-nonos-user.

## Release evidence

`cargo check --features microkernel-core,nonos-production,nonos-capsule-attest`
must compile clean.

## Release checklist

- [x] Every file ≤ 75 LOC
- [x] 15-line license header on every file
- [x] No inline comments past the license header
- [x] `Capsule.mk` mask `0x19` (no Debug)
- [x] Kernel mirror at `src/userspace/capsule_attest/`
- [ ] Cert + manifest baked (needs `nonos-mk-attest-sign` Makefile rule)
- [ ] Spawn wired through `src/userspace/init/spawn_plan/`
- [x] README documents all 16 contract sections + Privacy Posture
- [x] Static invariant table that external auditors can query
- [ ] QEMU spawn-verify (blocked by OVMF #PF)

## Explicit non-goals today

- No cryptographic signature on the reply (deferred): needs the
  attest capsule to hold a private key, which contradicts the
  CPL=3 capability bound).
- No live measurement of running capsules' actual cap masks (would
  need a kernel-side syscall returning the live caps_bits table; not
  yet exposed in libc).
- No TPM/TEE binding (the trust anchor is firmware-readable; binding
  to a hardware root of trust is a separate, large piece of work).
- No remote attestation transport (capsule_attest exposes the local
  service; an external auditor would need a network capsule to relay
  the proof bytes off-box).

## Verification

- `nonos-ci/run-static-checks.sh` clean.
- `make nonos-mk-host-trust-verify` verifies
  the baked `attest.manifest.bin` (once the signing rule is added).
- Kernel cargo check matrix passes with `nonos-capsule-attest`.

## Why this beats Linux

Linux cannot answer "is there a log surface anywhere in the running
system" with a definitive *no*. Too many components run as root, too
many libraries open files, too many daemons write `/var/log`. The
best Linux can do is "we shipped a hardened distro and you can grep
for journald."

NONOS answers the question structurally. Every loaded capsule has a
static, kernel-enforced capability mask signed in its manifest. The
mask either has the `Debug` bit or it doesn't. `OP_PROOF_CAPSULE_LIST`
returns every shipped capsule and its mask. Any reply that contains
the `Debug` bit (0x100) anywhere in the mask is a failure of the NO
LOGS invariant, and an external auditor can detect it programmatically
without trusting any narrative.
