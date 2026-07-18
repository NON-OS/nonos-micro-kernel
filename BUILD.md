# Building the NONOS kernel

The kernel is a single `cargo` crate. What goes into an image, which capsules,
which drivers, and whether the spawn gate is enforced, is selected entirely by
cargo features. This document explains how to choose them.

## Quick start

Configure a kernel interactively, then build it:

    make menuconfig      # choose a profile and options, writes .nonos-config
    make from-config     # build exactly what you chose

`make` with no arguments prints the target list. It never builds silently.

## Profiles

A profile is a composite feature declared in `Cargo.toml` under `[features]`,
named `microkernel-<name>`. Each one is a coherent, bootable selection: a base
runtime plus a fixed set of capsules and drivers. Profiles are the unit of
choice because an arbitrary mix of leaf features does not necessarily boot;
a profile is a combination that has been assembled to.

The catalogue, grouped:

The base profiles are `microkernel-core` (scheduler, memory, crypto, no capsules)
and `microkernel-capsules` (core plus proof-io, ramfs and keyring).

The single-capsule and single-driver profiles put one capsule or one driver on
top of the core, for bring-up and tests: `microkernel-ramfs`,
`microkernel-driver-nvme`, and so on.

The network profiles are the layered stack, each one adding a layer, from
`microkernel-net-l2` through `microkernel-net-sockets`.

The desktop profiles are `microkernel-desktop-gui` (compositor, apps, core
drivers) and `microkernel-full-gui` (the desktop plus every driver).

List them at any time:

    tools/nonos-config --list

## The canonical production target

`make nonos-mk-zerostate` builds the complete, release-tested system: every
capsule and driver, the transparent STARK spawn gate enforced, dual-signature
(Ed25519 + ML-DSA-65), the anti-rollback index bound into the signature, and the
TPM measured-boot path. This is the reference image; the other `-prod` targets
are narrower cuts of the same recipe for a single capsule, driver, or lane.

## Security options

Three choices change what a signed image is trusted to do. `make menuconfig`
prompts for each; you can also set them by hand in `.nonos-config`.

Capsule attestation (`nonos-stark-attest`) has the spawn gate verify a
post-quantum STARK proof that the capsule's measurement is enrolled under the
policy root before it runs. With attestation off, the legacy path is used.

Kernel self-attestation (`NONOS_STARK_KERNEL_ATTEST=1`) has the bootloader verify
the kernel's own STARK proof against the enrolled kernel root before the jump. It
needs an enrolled kernel image, so it is off by default.

The anti-rollback index (`NONOS_ROLLBACK_INDEX`) is the floor a signed image must
meet. The TPM monotonic counter refuses anything below it, so raising the index
at release time burns older images.

## .nonos-config

`tools/nonos-config` writes `.nonos-config` at the repo root: plain `make`
variables, so the build reads them directly and you can edit them by hand.

    NONOS_PROFILE := microkernel-full-gui
    NONOS_ATTEST := 1
    NONOS_STARK_KERNEL_ATTEST := 0
    NONOS_ROLLBACK_INDEX := 1

The file is per-developer and is not committed.

## Toolchain

The pinned toolchain is in `rust-toolchain.toml`. `make nonos-mk-check-deps`
verifies it is installed before a build starts. On macOS, ensure the rustup
toolchain is ahead of any Homebrew rust on `PATH`.
