# NØNOS

A microkernel where nothing runs unless it can prove itself.

Every program on NONOS is a signed, sandboxed capsule that carries a
transparent, post-quantum proof of what it is and what it is allowed to touch.
The kernel checks that proof before every spawn, and the bootloader checks the
kernel the same way before it jumps. There is no unsigned path to execution, no
debug switch that turns the check off, no privileged account that can wave code
through. Trust flows one direction only, downward from a root you can read.

NONOS is written in Rust, capability-based, and RAM-resident: it keeps no
mutable state on disk and leaves nothing behind after power-off. That is the
ZeroState model the system is named for. It targets x86_64, ARM64 and RISC-V;
x86_64 is the production tree, the other two are architecture-ready backends
behind the same arch boundary.

It boots. The default image comes up to a graphical desktop with a
damage-tracked compositor, window management, a terminal and a file manager, on
real drivers for NVMe and AHCI storage, xHCI USB and mass storage, e1000 and
Realtek networking, Intel and Realtek Wi-Fi, HDA audio, i2c and PS/2 input.
Fifty-nine capsules come up in it, out of seventy-eight in the tree. One
command builds the whole system, proves it, and boots it.

The attestation is checked, not asserted. Four independent tools verify it on
every change: Lean 4 with 1,076 theorems and no sorry, Verus, Kani with 82
harnesses, and 34 crates of runnable proofs against the real code. The prover
and the verifier are a single crate linked into both the kernel and the
bootloader, so the proof that gets written is the proof that gets read.

## Documentation

The full reference is the [NØNOS documentation wiki](https://github.com/NON-OS/nonos-docs),
mounted at [docs/](docs/) as a submodule so a checkout always carries the
matching revision. It is written against the source tree, with file and line
references, so every claim can be checked against the code. Start with the
[architecture overview](https://github.com/NON-OS/nonos-docs/blob/main/architecture/overview.md),
then the
[verification scope](https://github.com/NON-OS/nonos-docs/blob/main/architecture/verification.md)
and the
[attestation model](https://github.com/NON-OS/nonos-docs/blob/main/security/attestation.md).

The common tasks each have a short guide in the wiki:

The [quickstart](https://github.com/NON-OS/nonos-docs/blob/main/build/quickstart.md)
goes from a clean checkout to the system booting in QEMU.
[Configuring a kernel](https://github.com/NON-OS/nonos-docs/blob/main/build/menuconfig.md)
assembles your own kernel with `make menuconfig`.
[Contributing](https://github.com/NON-OS/nonos-docs/blob/main/community/contributing.md)
is where to start;
[verifiable work](https://github.com/NON-OS/nonos-docs/blob/main/community/verifiable-work.md)
is the capsule path.
[Reporting](https://github.com/NON-OS/nonos-docs/blob/main/security/reporting.md)
covers security reports and scope, and
[rewards](https://github.com/NON-OS/nonos-docs/blob/main/community/rewards.md)
covers what securing NONOS pays.

## Community

<div align="center">

**[`discord.gg/nonos`](https://discord.gg/nonos)**

[![Discord](https://img.shields.io/badge/join-5865F2?style=flat-square&logo=discord&logoColor=white&label=%20&labelColor=5865F2)](https://discord.gg/nonos)

</div>

Design discussion, capsule work and security reports start there. Verified
contributors get the capsule-signing channels.

## Building

    make                       # the full ZeroState system, attested, as a bootable image
    make qemu                  # boot it in QEMU

    make menuconfig            # choose what goes in your kernel
    make from-config           # build that

## License

AGPL-3.0-or-later. Redistributable device firmware is not part of the source
and carries its own terms.
