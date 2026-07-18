# NØNOS

A microkernel where nothing runs unless it can prove itself.

Every program on NONOS is a signed, sandboxed capsule that carries a
transparent, post-quantum proof of what it is and what it is allowed to touch.
The kernel checks that proof before every spawn, and the bootloader checks the
kernel the same way before it jumps. There is no unsigned path to execution, no
debug switch that turns the check off, no privileged account that can wave code
through. Trust flows one direction only, downward from a root you can read.

NONOS is written in Rust, capability-based, and RAM-resident: it keeps no mutable
state on disk and leaves nothing behind after power-off. That is the ZeroState
model the system is named for.

It boots. The default image comes up to a graphical desktop with a damage-tracked
compositor, window management, a terminal and a file manager, on real drivers for
NVMe and AHCI storage, xHCI USB and mass storage, e1000 and Realtek networking,
Intel and Realtek Wi-Fi, HDA audio, i2c and PS/2 input. Around sixty capsules
ship in it. One command builds the whole system, proves it, and boots it.

The attestation is checked, not asserted. Four independent tools verify it on
every change: Lean 4 with 203 theorems, Verus, Kani, and 165 runnable proofs
against the real code. The prover and the verifier are a single crate linked into
both the kernel and the bootloader, so the proof that gets written is the proof
that gets read.

## Documentation

The full reference is the NØNOS documentation wiki in [docs/](docs/), mounted as a
submodule so a checkout always carries the matching revision. It is written
against the source tree, with file and line references, so every claim can be
checked against the code. Start with the
[architecture overview](docs/architecture/overview.md), then the
[verification scope](docs/architecture/verification.md) and the
[attestation model](docs/security/attestation.md).

The common tasks have a short guide at the repository root:

[QUICKSTART.md](QUICKSTART.md) goes from a clean checkout to the system booting in
QEMU. [BUILD.md](BUILD.md) assembles your own kernel with `make menuconfig`.
[CONTRIBUTING.md](CONTRIBUTING.md) is where to start; [CONTRIBUTING-ZK.md](CONTRIBUTING-ZK.md)
is the capsule path. [SECURITY.md](SECURITY.md) covers reporting and scope, and
[REWARDS.md](REWARDS.md) covers what securing NONOS pays.

## Building

    make menuconfig            # choose what goes in your kernel
    make from-config           # build that

    make nonos-mk-zerostate    # the full ZeroState system, attested
    make nonos-mk-run          # build it, prove it, boot it in QEMU

Run `make` with no target to list everything.

## License

AGPL-3.0-or-later. Redistributable device firmware is not part of the source and
carries its own terms.
