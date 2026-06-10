# NØNOS

NONOS is a zero-state microkernel operating system in Rust where nothing
runs unless it can prove itself. Every application is a signed, sandboxed
capsule carrying a Groth16 zero-knowledge proof of exactly what it is and
what it is allowed to touch, and the kernel re-verifies that proof at
every single spawn. There is no override, no debug flag, no unsigned
path. The boot chain, the kernel, the drivers and the desktop all hold to
the same rule: verify, then run.

This is not a design document. The system boots to a real desktop with a
damage-tracked compositor, window management, a terminal, a file manager
and some seventy capsules, on a driver stack that covers NVMe, AHCI,
e1000, RTL8139/8169, xHCI, USB HID and mass storage, HDA audio, i2c and
PS/2, with a full TCP state machine behind it. One command builds it,
proves it and boots it.

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
policy root it was built under, over BLS12-381 with 192-byte proofs and
seven public inputs. The verifying key comes from a five-contributor
external trusted-setup ceremony whose transcript ships in this repo;
nobody, including us, holds the trapdoor.

## Verify it yourself

Do not take any of the above on faith. Clone with submodules and run the
live check; the canonical ceremony keys install from the trust keystore
and the transcript re-verifies on your machine:

```sh
git clone --recursive https://github.com/NON-OS/nonos-micro-kernel.git
cd nonos-micro-kernel
make nonos-mk-attestation-receipt
```

That target runs a real pairing check on every bundled capsule against
the ceremony verifying key (fingerprint 6cd2015037ea6181) and writes a
receipt you can re-check independently. As of this writing the fleet
stands at 59 capsules verified, 0 failed, and the runtime gate enforces
all of them at every boot. To watch it boot:

```sh
make nonos-mk-run
```

## The capsule model

A capsule is the unit of everything: one application, one address space,
one signed manifest, one proof. The kernel keeps primitives; capsules
own policy.

```
   userland/capsule_foo/          what you write
   ----------------------         -------------------------------
   Cargo.toml                     a normal Rust crate, two path
   src/main.rs                    dependencies, no kernel imports
   src/foo/...                    your app: manifest, events, paint
   Capsule.mk                     13 lines of metadata
            |
            |   make nonos-mk-foo-sign
            v
   +-----------------------------------------------------------+
   |  the pipeline you cannot skip                             |
   |                                                           |
   |  cross-compile -> sign manifest -> mint NONOS-ID cert     |
   |       -> generate Groth16 attestation proof               |
   |       -> verify it the way the kernel will                |
   +-----------------------------------------------------------+
            |
            v
   foo.capsule.zk ......... ELF + proof trailer, ready to spawn
```

The full walk from empty folder to your window on the desktop is in
[QUICKSTART.md](QUICKSTART.md); it was executed end to end to build
`userland/capsule_hello`, which stays in the tree as the guide's living
reference. The syscall ABI and IPC wire format are specified in
`abi/syscalls.toml` and `abi/wire.toml`, so the contract is
language-neutral even though Rust is the paved road.

## Earning NOX for securing NONOS

Verifiable security work earns receipts, and accepted receipts settle on
Ethereum mainnet from reward pools funded only by real demand revenue.
No minting, no yield; usage finances security.

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

## Repository map

```
   src/                  the microkernel: mem, sched, ipc, syscall,
                         security, drivers as capsule hosts
   nonos-bootloader/     UEFI bootloader: verified boot, firmware
                         terminal, ZK attestation tooling under tools/
   userland/             capsules, the SDK family, app skeleton,
                         toolkit, libc
   nonos-data/           trust keystore (submodule): publisher keys,
                         signed manifests, proof trailers, the
                         ceremony transcript and verifying key
   abi/                  machine-readable contracts: syscalls, wire
                         format, schemas, the mainnet deployment
   nonos-mk/             capsule build include (submodule)
   QUICKSTART.md         empty folder to attested capsule
   CONTRIBUTING-ZK.md    verifiable work, receipts and claims
```

## License

GNU AGPL-3.0. See [LICENSE](LICENSE).

<p align="center">
  <strong>NØNOS — Sovereignty from ∅</strong>
</p>
