# NØNOS

NONOS is a privacy-first microkernel operating system in Rust where
nothing runs unless it can prove itself. Every application is a signed,
sandboxed capsule carrying a transparent enrolled-secret proof of exactly
what it is and what it is allowed to touch, and the kernel re-verifies
that proof at every spawn. There is no override, no debug flag,
no unsigned default path. The boot chain, the kernel, the drivers and the
desktop all hold to the same rule: verify, then run.

This is not a design document. The system boots to a real desktop with
a damage-tracked compositor, window management, a terminal, a file
manager and some sixty capsules, on a driver stack that covers NVMe,
AHCI, e1000, RTL8139/8169, xHCI, USB HID and mass storage, HDA audio,
i2c and PS/2. One command builds it, proves it and boots it.

---

## Contents

- [A microkernel, strictly](#a-microkernel-strictly)
- [The chain of trust](#the-chain-of-trust)
- [Proven, not asserted](#proven-not-asserted)
- [Privacy by architecture](#privacy-by-architecture)
- [Requirements](#requirements)
- [Build and run](#build-and-run)
- [Verify it yourself](#verify-it-yourself)
- [Verification](#verification)
- [Build your own capsule](#build-your-own-capsule)
- [Earning NOX for securing NONOS](#earning-nox-for-securing-nonos)
- [Command reference](#command-reference)
- [Repository map](#repository-map)
- [License](#license)

---

## A microkernel, strictly

The kernel keeps primitives; capsules own policy. That line is
enforced, not aspirational: the network stack itself is not kernel
code. IP, TCP, DNS and the Nym mixnet client are each their own
sandboxed capsule, talking over kernel IPC like any other application.

```
  KERNEL (src/)                      CAPSULES (userland/)
  ------------------------------    --------------------------------
  memory, paging, scheduler          compositor          wm
  IPC ports and message rings        desktop shell       terminal
  capability grants                  file manager        editor
  capsule spawn gate (ZK)            settings -> live policy
  syscall surface                    net_ip   net_tcp    net_dns
  crypto primitives, CSPRNG          net_nym (sphinx mixnet client)
  cryptofs, ramfs hosts              drivers: gpu, usb, blk, input
  hardware broker                    wallet   market     about ...
            ^                                  |
            |          MkIpc* syscalls,        |
            +------ capability-checked --------+
                    on every message
```

A capsule sees exactly the capabilities its signed manifest requested
and its proof binds: the calculator cannot read your wallet, the text
editor cannot open a socket, and no capsule reaches hardware except
through the broker. Compromising one capsule gets an attacker that
capsule's mask and nothing else.

## The chain of trust

Trust is established once at the root and then never assumed again;
every arrow below is a cryptographic check that fails closed.

```
  UEFI firmware
      |
      |  signature check (Secure Boot, when enabled)
      v
  NONOS bootloader ............ verifies its own state, draws the
      |                         verified-boot panel, measures into
      |  Ed25519 + BLAKE3       the TPM when one is present
      v
  kernel image ................ signature and hash verified before
      |                         a single instruction executes
      |  transparent enrolled-secret ZK proof
      v
  capsule spawn gate .......... every capsule's proof re-verified
      |                         at runtime, every boot; no proof,
      |  capability grant       no process
      v
  running capsule ............. sandboxed behind capability-based
                                syscalls; owns its policy, never
                                the kernel's
```

The proofs bind each capsule's exact hash, its capability mask, its
epoch and the policy root it was enrolled under. There is no trusted
setup, proving key or ceremony trapdoor in the capsule attestation path.

## Proven, not asserted

The security-critical paths are not only tested, they are machine-checked,
and the checks run against the code that actually ships rather than a
separate model. The layers are documented in full under `verification/`:

- **Runnable proofs.** Host crates include the real capsule and kernel
  source (`#[path]`, only the syscall clock shimmed) and execute it: the
  VFS store and path canonicalization, the protocol codec, caller
  attestation, the Ethernet, IPv4 and UDP parsers, the driver ring walks,
  the bootloader anti-rollback and the in-kernel STARK. Millions of
  structured and random inputs assert the parsers never panic and never
  break their invariants. Writing them found and fixed real bugs.
- **Kani.** Over every bounded input, the untrusted-input surfaces are
  proven panic-free and UB-free, together with the no-impersonation and
  path-canonicalization theorems.
- **Verus.** Theorems about the kernel's own capability bit operations,
  page-table permission encoding and IPC length guards.
- **Lean.** Specification-level theorems for the capability algebra, W^X
  isolation, anti-rollback and path safety, refined onto the code proofs
  above, with the capability theorems bound to the extracted Rust through
  Charon and Aeneas. The Lean layer runs in CI on every push.

Because the specs are the code the kernel runs, there is no
model-implementation gap. `verification/README.md` and
`verification/STATUS.md` carry the current status, the exact reproduction
commands, and an honest account of what runs in CI versus what is
reproduced locally.

## Privacy by architecture

Privacy here is not a settings page; it is what falls out of the
design when state, identity and traffic are each handled by the most
paranoid mechanism available.

```
  your data ........ zero-state by default: the system is RAM-resident
                     and capsules hold no implicit persistent state.
                     What you choose to keep goes through cryptofs,
                     encrypted at rest with kernel crypto primitives.

  your apps ........ compartmentalized by capability: each capsule is
                     its own address space with its own grant mask.
                     There is no global filesystem view, no ambient
                     authority, nothing to scrape.

  your traffic ..... the network stack is capsules, and the top of it
                     is a Nym mixnet client speaking sphinx packets
                     through gateway and directory infrastructure, for
                     traffic that should not map to you.

  your identity .... verification is zero-knowledge end to end: the
                     system proves WHAT code is running without anyone
                     learning anything else. Contributor receipts bind
                     to an address you choose.

  our reach ........ limited by design. Default QEMU boots attach no
                     NIC, and network boots are explicit. Packet-capture
                     evidence is still required before claiming no
                     telemetry for a release.
```

## Requirements

| What | Why | Notes |
|---|---|---|
| rustup | toolchain manager | the repo pins its own nightly; rustup fetches it on first build |
| QEMU 8+ | running the OS | ships the UEFI firmware (OVMF / edk2) the run targets auto-detect |
| GNU make | the build driver | every workflow is a make target |
| git | submodules | clone with `--recursive`; the trust keystore and build includes are submodules |
| ~10 GB disk, 4 GB RAM | first build | a 2-core 2 GB box verifies the fleet fine; building the full desktop wants more |

macOS and Linux are both supported as build hosts. The GUI run targets
use hardware acceleration (HVF on macOS, KVM on Linux via QEMU's
default probing); the serial and CI boot targets run anywhere QEMU
does.

## Build and run

Clone with submodules; the trust keystore and build includes arrive
with the tree:

```sh
git clone --recursive https://github.com/NON-OS/nonos-micro-kernel.git
cd nonos-micro-kernel
```

Build everything and boot the desktop in QEMU:

```sh
make nonos-mk-run
```

The first build compiles the pinned toolchain targets, the bootloader,
the kernel, all capsules, then signs and attests each one before
booting. The default QEMU boot attaches no NIC; use a `*-net` target
when exercising network-capable capsules. Expect a long first run;
everything after is incremental.
Useful variants:

```sh
make nonos-mk-run-serial      # headless, serial console only
make nonos-mk-run-nat         # QEMU outbound NAT for wallet/RPC demos
make nonos-mk-run-net         # explicit QEMU user-network host forwarding
make nonos-mk-run-serial-nat  # serial console plus outbound NAT
make nonos-mk-run-serial-net  # serial console plus explicit host forwarding
make nonos-mk-run-wizard      # first-boot setup wizard flow
make nonos-mk-debug           # QEMU paused with GDB on :1234
make nonos-mk                 # build the runtime baseline, no QEMU
make nonos-mk-check           # fast cargo check of the kernel
make nonos-mk-bootloader      # UEFI bootloader only
```

## Verify it yourself

Do not take this README on faith. The current source path builds the
transparent attestation tools, regenerates capsule trailers from an
enrollment seed and compiles the capsule kernel profile with those
trailers embedded:

More verification entry points:

```sh
make nonos-mk-capsules          # capsule kernel profile, no QEMU
make nonos-mk-all-capsules-attested      # rebuild signed and attested capsules
make nonos-mk-host-trust-verify # host trust chain against the baked policy
make nonos-mk-check-trust-manifest      # keystore ledger integrity
```

## Verification

```sh
make nonos-mk-verify          # static gates + capsule build + symbol scan
make nonos-mk-verify-trust    # signed capsule manifest and trust ledger checks
make nonos-mk-capsules        # compile capsule profile with embedded trailers
```

## Build your own capsule

The complete path from an empty folder to your own signed, attested
window on the desktop is [QUICKSTART.md](QUICKSTART.md). It was
executed end to end to create `userland/capsule_hello`, which stays in
the tree as the guide's living reference; the run took an afternoon,
including reading time. The short version:

```
  write a small Rust crate  ->  13-line Capsule.mk  ->  one Makefile
  include  ->  generate your publisher keys  ->  make nonos-mk-hello
  nonos-mk-hello-sign  ->  your capsule is signed, certified and
  carrying a proof the kernel will re-verify at every boot
```

The syscall ABI and IPC wire format are specified in
`abi/syscalls.toml` and `abi/wire.toml`, not in Rust headers, so the
contract is language-neutral even though Rust is the paved road.

## Earning NOX for securing NONOS

The old Groth16-based NOX receipt lane has been removed from the active
build. [CONTRIBUTING-ZK.md](CONTRIBUTING-ZK.md) now tracks the
transparent-attestation receipt work that must land before reward
claims are advertised again.

## Command reference

`make help` prints the live catalog. The targets you will reach for:

```
  build        nonos-mk, nonos-mk-core, nonos-mk-capsules,
               nonos-mk-check, nonos-mk-bootloader, nonos-mk-esp
  run          nonos-mk-run, nonos-mk-run-serial, nonos-mk-run-wizard,
               nonos-mk-debug
  verify       nonos-mk-static, nonos-mk-scan, nonos-mk-verify-fast,
               nonos-mk-verify, nonos-mk-verify-trust,
               nonos-mk-host-trust-verify, nonos-mk-check-trust-manifest
  zk           nonos-mk-zk-tools, nonos-mk-zk-report,
               nonos-mk-zk-verify-live
  capsules     nonos-mk-<slug>, nonos-mk-<slug>-sign per capsule;
               nonos-mk-userland-clean
  hygiene      nonos-mk-fmt, nonos-mk-clean
```

## Repository map

```
   src/                  the microkernel: mem, sched, ipc, syscall,
                         security, cryptofs, capsule spawn, broker
   nonos-bootloader/     UEFI bootloader: verified boot, firmware
                         terminal, ZK attestation tooling under tools/
   userland/             capsules (apps, drivers, the whole network
                         stack), the SDK family, app skeleton,
                         toolkit, libc
   nonos-data/           trust keystore (submodule): publisher keys,
                         signed manifests and proof trailers
   verification/         machine-checked proofs and their status: Lean
                         specifications, Charon/Aeneas extraction and Verus;
                         the runnable and Kani proof crates live beside the
                         code they prove, under userland/ and the bootloader
   abi/                  machine-readable contracts: syscalls, wire
                         format, schemas, the mainnet deployment
   nonos-mk/             capsule build include (submodule)
   nonos-sign/           host signing tool (submodule)
   QUICKSTART.md         empty folder to attested capsule
   CONTRIBUTING-ZK.md    verifiable work, receipts and claims
```

## License

GNU AGPL-3.0. See [LICENSE](LICENSE).

<p align="center">
  <strong>NØNOS — Sovereignty from ∅</strong>
</p>
