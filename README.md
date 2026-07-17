# NØNOS

NONOS is a privacy-first microkernel operating system in Rust where nothing runs
unless it can prove itself. Every application is a signed, sandboxed capsule
carrying a transparent, post-quantum proof of exactly what it is and what it is
allowed to touch, and the kernel re-verifies that proof at every spawn. There is
no override, no debug flag, no unsigned default path. The boot chain, the
kernel, the drivers and the desktop all hold to the same rule: verify, then run.

This is not a design document. The system boots to a real desktop — compositor,
window management, terminal, file manager, some sixty capsules — on a driver
stack covering NVMe, AHCI, e1000, RTL8139/8169, xHCI, USB HID and mass storage,
HDA audio, i2c and PS/2. One command builds it, proves it and boots it.

## Start here

Pick the path that fits you. Each is a self-contained, step-by-step guide.

- **Run it** — [QUICKSTART.md](QUICKSTART.md): install the toolchain, build the
  full system, boot it in QEMU.
- **Build your own kernel** — [BUILD.md](BUILD.md): choose a profile and the
  security options with `make menuconfig`, then `make from-config`.
- **Understand the trust model** — [ATTESTATION.md](ATTESTATION.md): the
  two-layer STARK attestation, the shared verifier, and how it is proven.
- **Write a capsule** — [CONTRIBUTING-ZK.md](CONTRIBUTING-ZK.md): the capsule
  ABI, signing, enrollment, and getting it attested.
- **Report a weakness** — [SECURITY.md](SECURITY.md): scope, disclosure, and the
  trusted computing base.
- **Earn NOX** — [REWARDS.md](REWARDS.md): what securing NONOS pays.

## The one command

    make menuconfig        # choose what goes in your kernel
    make from-config       # build exactly that

    make nonos-mk-zerostate   # or: the full ZeroState system, attested
    make nonos-mk-run         # build, prove, and boot it in QEMU

`make` with no arguments prints the full target list.

## How it is proven

The attestation is machine-checked by four independent tools, all in CI: **Lean
4** (203 theorems), **Verus** (SMT), **Kani** (model checking), and **165
runnable proofs** against the real code. The proof stack lives in one crate,
`nonos-stark`, linked by both the kernel and the bootloader, so the prover and
the verifier agree by construction. See [ATTESTATION.md](ATTESTATION.md).

## License

AGPL-3.0-or-later. Redistributable device firmware is not part of the source and
carries its own terms.
