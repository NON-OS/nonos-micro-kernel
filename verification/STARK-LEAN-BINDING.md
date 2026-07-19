<!--
NONOS Operating System
Copyright (C) 2026 NONOS Contributors
AGPL-3.0-or-later
-->

# Binding the proof corpus into the boot attestation

The kernel's transparent STARK self-attestation proves, at boot, that the image
that is about to run is the one enrolled under the trusted root. Today it commits
to `blake3(image) || boot_epoch`. This note specifies how to extend that public
commitment to also carry the **proof-corpus root**, so the boot proof states not
only "this is the enrolled image" but "this exact image is the one whose
properties were machine-checked in Lean". It is a design spec, not a stub: the
change is deliberately not half-wired, because a mismatch between the enrolling
side and the verifying side would brick the boot gate. It lands as one coherent
change with a build and a boot to prove it.

## What already exists

- `verification/lean/proof-corpus-root.sh` emits a reproducible 32-byte
  commitment over the whole proven Lean corpus (source hashes, toolchain, axiom
  profile) and refuses to form on any `sorry` or non-standard axiom. This is the
  `verification_root`.
- The CI Lean job runs it as a gate, so the root is already trustworthy.
- `nonos-stark-enroll/src/main.rs::kernel_context` (around line 58) builds the
  kernel attestation context as `blake3(image) || BOOT_EPOCH`.

## The change

Three sides move together, or none do.

1. **Enrollment (`nonos-stark-enroll`).** Extend `kernel_context` to
   `blake3(image) || BOOT_EPOCH || verification_root`, where `verification_root`
   is read from a new `--verification-root <file>` argument. The `kernel`
   subcommand passes it through. When the argument is absent the context is
   unchanged, so every existing non-attested flow is byte-for-byte identical and
   nothing breaks; the binding is active only when a root is supplied.

2. **Kernel self-attest verifier.** The kernel recomputes its own context to
   check the enrolled trailer. It must fold in the same `verification_root`,
   which means the root is baked into the kernel image at build time exactly as
   `NONOS_KERNEL_ATTEST_ROOT` is baked into the bootloader (`build.rs` emits a
   32-byte constant; the verifier reads it). The build wires
   `verification/lean/proof-corpus-root.sh` output into that constant.

3. **Makefile / mk.** The `nonos-mk-kernel-attest` path gains a step that runs
   `proof-corpus-root.sh`, writes the 32 bytes to a file, passes it to
   `nonos-stark-enroll kernel --verification-root`, and bakes the same file into
   the kernel build. A single variable (`NONOS_BIND_VERIFICATION_ROOT`, default
   off until proven) gates the whole thing so the current ship path is untouched
   until the boot test passes.

## How it is proven, not asserted

- Compile both crates.
- `make nonos-mk-zerostate` (or the desktop-gui-prod cut) with the binding on,
  so the enrolled root and the baked root are computed from the same corpus.
- Boot it in QEMU with OVMF and swtpm. A booting kernel is the proof: if the
  baked `verification_root` did not match the enrolled one, the self-attest gate
  would fail and `fatal_reset` before the desktop. Reaching `[INIT] Capsules
  spawned` means the boot STARK verified a context that includes the proof-corpus
  root.
- Negative test: perturb one Lean source (which changes the corpus root), rebuild
  only the enrollment side, and confirm the boot gate now refuses. This shows the
  binding is load-bearing, not decorative.

## What it does and does not claim

It makes the boot attestation carry a commitment to the set of properties proven
of the system, checkable by anyone: rebuild the repo, recompute the corpus root,
confirm it matches the attested public input, and re-run Lean to confirm the
axioms are standard. It does **not** prove the Lean checker itself inside the
STARK; arithmetizing a dependent type checker is not the goal and would add no
trust over running Lean. The honest claim is a reproducible binding between the
booting image and its machine-checked proof corpus, which is what
[the verification page](../docs/architecture/verification.md) and the evidence
manifest already inventory.
