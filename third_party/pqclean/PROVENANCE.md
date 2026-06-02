# PQClean vendored snapshot

This directory holds a vendored snapshot of [PQClean](https://github.com/PQClean/PQClean),
the reference C implementations of post-quantum cryptographic primitives.

## Why it is vendored

Upstream PQClean enters read-only archive status in July 2026. The kernel
build cannot depend on a moving target, and we need byte-stable inputs to
the signing pipeline. The snapshot is committed in full so the trust chain
audit can replay any past kernel build without external network access.

## What we actually compile

Only the algorithms used by NONOS:

- `crypto_sign/ml-dsa-65/clean/` -- hybrid manifest + cert signatures
- `crypto_kem/ml-kem-512/clean/`
- `crypto_kem/ml-kem-768/clean/`
- `crypto_kem/ml-kem-1024/clean/`
- `common/fips202.c`, `common/randombytes.c`, `common/aes.c` (as needed)

All other algorithm directories (Falcon, HQC, McEliece, SPHINCS+, ML-DSA-44,
ML-DSA-87) are present on disk but never compiled by `build.rs` (kernel) or
`nonos-sign/build.rs` (host signer). Cargo features in the root
`Cargo.toml` and `nonos-sign/Cargo.toml` gate the per-algorithm build steps.

## Tamper detection

The vendored tree's git object hash is pinned. CI verifies the hash on
every push via `nonos-ci/check-pqclean-pin.sh`. Any modification to any
file under this directory changes the tree hash and fails the gate.

Current pin:

```
PQCLEAN_TREE_SHA=dd152ebaa9a001f9eb0d6b9c20dc82f8767335a0
```

To intentionally update the snapshot:

1. Replace the tree contents.
2. Run `git add third_party/pqclean`.
3. Read the new hash with `git rev-parse HEAD:third_party/pqclean` (use
   `--no-index` against the index for an uncommitted check).
4. Update `PQCLEAN_TREE_SHA` above and in `nonos-ci/check-pqclean-pin.sh`.
5. Re-run the host-signer test suite (`cd nonos-sign && cargo test
   --release --test artifacts`) and confirm the manifest verifier still
   round-trips every baked capsule under the new C implementation.

## Build flags

Kernel build (`build.rs`):

```
-ffreestanding -fno-builtin -fno-stack-protector -fno-pic -mno-red-zone -mcmodel=kernel
```

Host signer build (`nonos-sign/build.rs`):

Default Rust toolchain `cc` defaults; no kernel-only flags.

## Compliance note

Upstream PQClean's SECURITY.md does not make explicit security claims and
recommends an expert review before production use. The NONOS production
posture requires hybrid Ed25519 + ML-DSA-65 so the trust chain never
relies on PQClean alone -- classical signatures act as a defense-in-depth
backstop for any latent issue in the C implementation.
