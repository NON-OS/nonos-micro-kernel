# crypto_proofs

Known-answer proofs for the kernel cryptographic primitives. Each test includes
the real `src/crypto` source through `#[path]` and runs it on the host against
the official standard vector. The property proved is a statement about the code
that ships, not about a re-implementation of it.

## Primitives and the standards they are checked against

| Primitive | Standard |
|-----------|----------|
| SHA-256, SHA-512 | NIST FIPS 180-4 (including the one-million-byte message) |
| SHA-3-256, SHA-3-512 | NIST FIPS 202 |
| BLAKE3 (hash, keyed, derive-key) | the reference test set |
| HMAC-SHA256 | RFC 4231 |
| HKDF-SHA256 | RFC 5869 |
| ChaCha20-Poly1305 | RFC 8439 |
| AES-128-GCM | NIST GCM test cases |
| Ed25519 | RFC 8032 |
| ECDSA P-256, P-384 | RFC 6979 |
| secp256k1 | SEC 2 generator plus an RFC 6979 sign/verify round trip |
| RSA PKCS#1 v1.5 (SHA-256) | an OpenSSL-produced 2048-bit signature |

For the authenticated schemes the tests also check that verification rejects a
tampered tag, ciphertext, associated data, or signature component.

## The trust root

Ed25519 is the signature the bootloader and the capsule loader require before
executing any code. The tests prove this verifier reproduces RFC 8032 byte for
byte: public-key derivation, deterministic signing, and verification, with
rejection of flipped R, flipped S, altered messages, and wrong keys. Proving it
is proving the gate that admits everything else.

## Constant-time primitives

The constant-time comparison, selection, and integer predicates that gate every
MAC, tag, and signature check are proven equal to the ordinary operation they
replace, over roughly a million sampled inputs and by Kani over all inputs. A
masking defect in these would accept or reject silently. The timing property is
by construction: the code is branch free and reads every byte.

## Interoperability

The RSA vector is produced by OpenSSL rather than by the kernel itself, so the
verifier is shown to agree with a reference implementation, not only with its own
output.

## Run

```sh
cd userland/crypto_proofs
cargo test --release      # runnable known-answer proofs
cargo kani                # all-input constant-time checks (requires Kani)
```
