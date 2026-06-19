# NØNOS

NONOS is a privacy-first microkernel operating system in Rust where
nothing runs unless it can prove itself. Every application is a signed,
sandboxed capsule carrying a Groth16 zero-knowledge proof of exactly
what it is and what it is allowed to touch, and the kernel re-verifies
that proof at every single spawn. There is no override, no debug flag,
no unsigned path. The boot chain, the kernel, the drivers and the
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
      |  Groth16 / BLS12-381
      v
  capsule spawn gate .......... every capsule's proof re-verified
      |                         at runtime, every boot; no proof,
      |  capability grant       no process
      v
  running capsule ............. sandboxed behind capability-based
                                syscalls; owns its policy, never
                                the kernel's
```

The proofs bind each capsule's exact hash, its capability mask and the
policy root it was built under, over BLS12-381 with 192-byte proofs
and seven public inputs. The verifying key comes from a
five-contributor external trusted-setup ceremony whose transcript
ships in this repo; nobody, including us, holds the trapdoor.

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

Clone with submodules; the canonical ceremony keys arrive with the
trust keystore and re-verify on your machine during the first build:

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

Do not take this README on faith. From a fresh clone, with no signing
keys and nothing rebuilt, re-prove every shipped capsule:

```sh
make nonos-mk-verify-attestation
```

That runs a real BLS12-381 pairing check on every committed capsule
proof trailer against the public verifying key (fingerprint
6cd2015037ea6181). As of this writing the fleet stands at 59 capsules
verified, 0 failed, and the runtime gate enforces all of them at every
boot. It needs only the toolchain and the trust keystore that comes
with the clone, no keys of your own.

(`make nonos-mk-attestation-receipt` is the heavier developer path: it
rebuilds and re-attests every capsule from source with your own
scratch keys, then verifies. Use it when changing capsules, not just
to check the release.)

More verification entry points:

```sh
make nonos-mk-zk-verify-live    # watch each pairing check, one per capsule
make nonos-mk-attestation CAP=terminal   # deep-dive one capsule's proof
make nonos-mk-host-trust-verify # host trust chain against the baked policy
make nonos-mk-check-trust-manifest      # keystore ledger integrity
```

## Verification

```sh
make nonos-mk-verify          # static gates + capsule build + symbol scan
make nonos-mk-verify-trust    # signed capsule manifest and trust ledger checks
make nonos-mk-verify-attestation # committed capsule attestation receipt checks
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

Verifiable security work earns receipts, and accepted receipts settle
on Ethereum mainnet from reward pools funded only by real demand
revenue. No minting, no yield; usage finances security.

```
        work                    receipt                  settlement
   ----------------       ------------------       -------------------
   run the fleet           nox-work-receipt          epoch Merkle root
   verification    ---->   validates the    ---->    published and
   boot a witness          artifact, binds           finalized on-chain;
   audit a capsule         it to your addr           claim pays from
   join a ceremony                                   NoxRewardPool
```

Start with one command and your reward address:

```sh
make nonos-mk-nox CONTRIB=0x<your address>
```

It rebuilds the system, runs all 59 pairing checks on your machine,
issues a receipt bound to your address and re-verifies it the way an
authorizer would. The loop, the receipt kinds and the claim flow are
documented in [CONTRIBUTING-ZK.md](CONTRIBUTING-ZK.md); the deployed
contract addresses and the frozen on-chain interface live in
`abi/nox_deployment.json` and `abi/NOX_CONTRACTS.md`.

## Command reference

`make help` prints the live catalog. The targets you will reach for:

```
  build        nonos-mk, nonos-mk-core, nonos-mk-capsules,
               nonos-mk-check, nonos-mk-bootloader, nonos-mk-esp
  run          nonos-mk-run, nonos-mk-run-serial, nonos-mk-run-wizard,
               nonos-mk-debug
  verify       nonos-mk-verify-attestation (no keys, nothing rebuilt),
               nonos-mk-attestation-receipt, nonos-mk-zk-verify-live,
               nonos-mk-attestation CAP=<name>, nonos-mk-verify-trust,
               nonos-mk-host-trust-verify, nonos-mk-check-trust-manifest
  zk           nonos-mk-zk-report, nonos-mk-zk-tools,
               nonos-mk-zk-ceremony (deliberately re-run a setup)
  nox          nonos-mk-nox CONTRIB=0x..., nonos-mk-nox-receipt,
               nonos-mk-nox-verify RECEIPT=..., nonos-mk-nox-merkle
               CLAIMS=..., nonos-mk-nox-registry
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
                         signed manifests, proof trailers, the
                         ceremony transcript and verifying key
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
