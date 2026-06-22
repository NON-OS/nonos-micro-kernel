# NONOS transparent attestation tools

This crate contains the host-side transparent enrolled-secret tooling used by
the bootloader and capsule attestation pipeline. It has no setup trapdoor and
no verifying key registry.

## Binaries

- `transparent-enroll`: derives enrolled Pedersen secrets from labels and emits
  `device_root.bin`, `device_commitments.bin`, and `device_secrets.txt`.
- `transparent-prove`: proves knowledge of an enrolled secret opening for a
  caller-supplied context.
- `transparent-verify`: verifies the proof against a root and context.

## Proof Statement

The prover proves knowledge of `(x, r)` such that `C = x*G + r*H`, and proves
that `C` is a leaf under the supplied Merkle root. The Fiat-Shamir challenge
binds the root and context so proofs are not replayable across capsules,
kernels, epochs, or boot challenges.

## Wire Format

`C[32] || A[32] || z_x[32] || z_r[32] || depth[1] || siblings[32*depth] ||
dirs[ceil(depth/8)]`

The verifier rejects malformed points, invalid Merkle paths, and failed
opening equations.

## Boot Contexts

Static embedded boot proof:

`device_root[32] || kernel_hash[32]`

Runtime sidecar boot proof:

`kernel_hash[32] || boot_nonce[32] || machine_id[32] || timestamp_be[8]`

Production boot policy requires a runtime sidecar when a pending challenge is
present or production mode is selected.
