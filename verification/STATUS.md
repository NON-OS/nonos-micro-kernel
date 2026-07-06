# NØNOS Verification Status

Last local run: 2026-07-06.

## Source Hygiene

Command:

```sh
PATH="$HOME/.cargo/bin:$PATH" cargo clippy --manifest-path nonos-verify/Cargo.toml --all-targets -- -D warnings
PATH="$HOME/.cargo/bin:$PATH" cargo run --manifest-path nonos-verify/Cargo.toml -- hygiene
```

Result:

```text
nonos-verify clippy: pass
production-source-hygiene: pass
```

Coverage:

- Production Rust source under `src/` and `userland/`.
- Excludes proof crates and build outputs.
- Fails on production `.unwrap(`, `.expect(`, `panic!`, `todo!`, `unimplemented!`, `unreachable!`, `#[allow(dead_code)]`, and temporary comment markers.

## Runnable Proofs

Command:

```sh
cd userland/fs_proofs
PATH="$HOME/.cargo/bin:$PATH" cargo test --release
```

Result:

```text
65 passed; 0 failed; 0 ignored
```

Coverage:

- VFS store operations: create, open, read, write, append, close, unlink, mkdir, rmdir, rename, copy, truncate, chmod, stat, usage.
- VFS path security: duplicate slash collapse, dot and parent resolution, root clamping, `/capsules` read-only guard after normalization.
- VFS wire protocol: short buffers, bad magic, bad version, bad length, oversized payloads, trailing bytes, response framing.
- Caller attestation: kernel mirror path, userspace sender matching, impersonation rejection.
- File manager logic: listing, directory deduplication, prefix filtering, extension handling, type classification, formatting helpers.
- Fuzz-style hostile input: request decode, caller split, path normalization.
- Network parser proofs: Ethernet, IPv4, and UDP parser bounds.

## Kani

Command:

```sh
cd userland/fs_proofs
PATH="$HOME/.cargo/bin:$PATH" cargo kani --output-format terse
```

Result:

```text
Complete - 7 successfully verified harnesses, 0 failures, 7 total.
```

Harnesses:

- `proof_decode_request_total`: all bounded request byte strings either decode safely or return a decode error.
- `proof_split_caller_no_impersonation`: a non-kernel sender cannot claim a different caller pid through the VFS payload.
- `proof_normalize_short_invariants`: bounded hostile path bytes normalize to an absolute canonical path without slash, dot, parent, or trailing slash defects.
- `proof_normalize_branch_cases`: parent-dir, duplicate slash, dot, trailing slash, relative path, and empty path branches preserve canonical invariants.
- `proof_eth_parse_total`: bounded Ethernet frames never panic and accepted frames expose only in-bounds payloads.
- `proof_ipv4_parse_total`: bounded IPv4 packets never panic and accepted packets expose only in-bounds payloads.
- `proof_udp_parse_total`: bounded UDP segments never panic and accepted segments expose only in-bounds payloads.

## Crypto KATs

Command:

```sh
cd userland/crypto_proofs
PATH="$HOME/.cargo/bin:$PATH" cargo test --release
```

Result:

```text
49 passed; 0 failed; 0 ignored
```

Coverage:

- SHA-256 known-answer tests from FIPS 180-4.
- SHA-512 known-answer tests from FIPS 180-4.
- SHA-3 256/512 known-answer tests from FIPS 202.
- BLAKE3 hash, keyed-hash and derive-key vectors from the official test set.
- HMAC-SHA-256 known-answer tests from RFC 4231.
- HKDF-SHA-256 extract and expand vectors from RFC 5869.
- ChaCha20-Poly1305 AEAD vector from RFC 8439, plus tamper rejection.
- AES-128-GCM vectors from the NIST GCM specification, plus tamper rejection.
- Ed25519 signing and verification vectors from RFC 8032, plus tamper rejection.
- P-256 ECDSA verification vectors from RFC 6979, plus tamper rejection.
- P-384 ECDSA verification vector from RFC 6979 (SHA-384), plus tamper rejection.
- secp256k1 scalar-multiplication anchored on the SEC 2 generator, plus
  deterministic RFC 6979 sign/verify round-trip and tamper rejection.
- RSA PKCS#1 v1.5 (SHA-256) verification of an OpenSSL-produced 2048-bit
  signature (cross-implementation interop), plus tamper rejection.

## Verus

Command:

```sh
cd verification/verus
/private/tmp/nonos-verus-0.2026.06.28/verus-x86-macos/verus --crate-type=lib src/lib.rs
```

Result:

```text
verification results:: 25 verified, 0 errors
```

Theorems:

- `revoke_is_monotonic`: revocation cannot create authority.
- `revoke_drops_the_right`: revoking a bit removes that bit.
- `attenuation_confines`: attenuation cannot grant rights absent from the parent token.
- `grant_preserves_and_adds`: granting preserves existing rights and grants the requested nonzero right.
- `empty_token_grants_nothing`: an empty token grants no right.
- `zero_length_is_rejected`: zero-byte IPC messages fail the shared IPC length gate.
- `oversized_message_is_rejected`: messages over `MAX_MESSAGE_SIZE` fail the shared IPC length gate.
- `accepted_message_is_bounded`: accepted IPC lengths are nonzero and at most `MAX_MESSAGE_SIZE`.
- `send_reply_share_the_same_length_gate`: send and reply use the same `1..=1048576` message-size rule.
- `encoded_pte_is_present`: encoded page-table entries are present.
- `writable_bit_matches_permission`: encoded writable permission matches the source permission.
- `user_bit_matches_permission`: encoded user permission matches the source permission.
- `executable_bit_matches_permission`: encoded NX state is the inverse of source executable permission.
- `no_wx_when_permission_rejects_wx`: non-WX permissions do not encode writable-executable pages.
- `permission_subset_is_monotonic`: a permission subset cannot have write, user, or execute authority absent from the parent.

## Target Builds

Commands:

```sh
cd userland/capsule_vfs
PATH="$HOME/.cargo/bin:$PATH" cargo build --release --target ../x86_64-nonos-user.json -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem

cd userland/capsule_file_manager
PATH="$HOME/.cargo/bin:$PATH" cargo build --release --target ../x86_64-nonos-user.json -Zbuild-std=core,alloc -Zbuild-std-features=compiler-builtins-mem
```

Result:

```text
Finished release profile for both capsules.
```

## CI Pinning

- Kani GitHub Action: `model-checking/kani-github-action@v1`, pinned input `kani-version: "0.67.0"`.
- Verus release: `release/0.2026.06.28.1847ab3`.
- Verus Linux asset: `verus-0.2026.06.28.1847ab3-x86-linux.zip`.
- Verus Linux Rust toolchain: `1.96.0-x86_64-unknown-linux-gnu`.
