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

[QUICKSTART.md](QUICKSTART.md) takes you from a clean checkout to the full system
booting in QEMU.

[BUILD.md](BUILD.md) is for assembling your own kernel. `make menuconfig` walks
the profiles and the security options and writes a config; `make from-config`
builds precisely that.

[ATTESTATION.md](ATTESTATION.md) is the trust model in full: how the bootloader
proves the kernel, how the kernel proves each capsule, and how both are checked.

[CONTRIBUTING-ZK.md](CONTRIBUTING-ZK.md) is the route to writing a capsule,
signing it, and getting it enrolled and attested.

[SECURITY.md](SECURITY.md) sets out reporting, scope, and the trusted computing
base. [REWARDS.md](REWARDS.md) sets out what securing NONOS pays.

## Building

    make menuconfig            # choose what goes in your kernel
    make from-config           # build that

    make nonos-mk-zerostate    # the full ZeroState system, attested
    make nonos-mk-run          # build it, prove it, boot it in QEMU

Run `make` with no target to list everything.

## License

AGPL-3.0-or-later. Redistributable device firmware is not part of the source and
carries its own terms.
