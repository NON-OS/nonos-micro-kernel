# Attestation

NONOS trusts nothing it cannot check. Two layers enforce that with a
transparent, post-quantum STARK: the bootloader checks the kernel, and the
kernel checks every capsule. Both use one verifier, so the proof a prover writes
is the proof a verifier reads, byte for byte.

## What a proof says

A membership proof shows that a measurement (the BLAKE3 hash of an image) is a
leaf of a Merkle tree whose root the verifier already trusts, and that the proof
was drawn under a specific context: the image, its granted capabilities, the
policy epoch. It is a FRI-STARK over the Goldilocks field, hash-based, with no
trusted setup and no pairing, sound against a quantum adversary. The soundness
is money-grade: extension-field challenges, proof-of-work grinding and a
blown-up rate, accounted in `Soundness.lean`.

## The two layers

The bootloader already measures the kernel with BLAKE3. Before it jumps, it
verifies the kernel's own STARK trailer, carried in the image footer, against the
enrolled kernel root. The code is in
`nonos-bootloader/src/kernel_verify/stark_attest.rs`. A zeroed root trusts
nothing, so an un-enrolled build cannot be spoofed.

The kernel gates every spawn the same way. It verifies the capsule's STARK
trailer against the policy root, bound to the capsule's measurement and its
granted capabilities, in `src/security/capsule_attest/stark.rs`. No proof, no
spawn.

## One shared verifier

The proof stack lives in the `nonos-stark` crate, linked by both the kernel and
the bootloader. It carries its own keccak256 and BLAKE3, so the Fiat-Shamir
transcript and the measurement are identical on the prover and the verifier.
Extracting it was bounded: the whole module reached the rest of the kernel
through exactly seven hash calls.

## Enrollment

`nonos-stark-enroll` measures the capsule set, commits it to one policy tree
padded to a fixed depth with a domain-separated slot no ELF can occupy, and
emits each capsule's trailer bound to its identity. It re-runs the exact
spawn-gate parse on every trailer before writing it, so a trailer that would be
refused at boot fails at build. `make menuconfig` and the production build wire
this in; see [BUILD.md](BUILD.md).

## Machine-checked

The design is proven across four independent tools, all in CI.

Lean 4 carries 203 theorems: membership soundness, context and capability
binding, measurement injectivity, the money-grade soundness budget, and
signing-key rollback and revocation, under `verification/lean/Nonos/Stark/`.

Verus SMT-checks the trailer length bound and the gate predicate, in
`verification/verus/src/stark_attestation.rs`.

Kani proves the untrusted-trailer deserializer is total on any input, in
`userland/stark_proofs/src/kani_proofs.rs`.

The runnable suite is 165 tests against the real code, including a proof that a
hostile length prefix is refused without over-allocating, under
`userland/stark_proofs/`.

These prove the model and the parser; a QEMU boot proves the wiring.

## Hardening

The proof deserializer read a 32-bit count from an untrusted trailer and passed
it to `Vec::with_capacity` before any data backed it, so a capsule could force a
multi-gigabyte reservation at the spawn gate. Every length is now capped at the
bytes remaining, since each element consumes at least one. Proven three ways:
Kani totality, a runnable proof-of-concept, and a Verus invariant.
