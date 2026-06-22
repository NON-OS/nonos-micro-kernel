# NØNOS Threshold Signing

FROST threshold signatures over Ristretto255. Multiple parties collaborate to produce a single Ed25519-compatible signature without any party ever holding the complete private key.

Status: Production ready. Pending bootloader integration.

---

## Why threshold signatures

Traditional multi-sig requires collecting multiple separate signatures and verifying each one. If you need 3-of-5 approval, you store and verify 3 signatures.

Threshold signatures are different. The 3-of-5 parties collaborate through a two-round protocol to produce a single 64-byte signature. Anyone verifying it sees one signature, one public key - indistinguishable from regular Ed25519. But cryptographically, that signature could only have been created if at least 3 of the 5 keyholders participated.

No single party ever holds the full private key. Not during setup, not during signing, not ever.

---

## How it works

**Key generation** splits a secret into n shares with threshold t. Each participant gets one share. The shares are constructed so that any t of them can reconstruct the signing capability, but t-1 shares reveal nothing.

**Signing** happens in two rounds:
1. Each participant generates a random commitment and shares it
2. After seeing all commitments, each participant produces a signature share
3. Any t signature shares combine into the final signature

The magic is in the math - Shamir secret sharing plus Schnorr signatures plus Lagrange interpolation.

---

## The tools

```
threshold-keygen          Generate t-of-n key shares (one-time setup)
threshold-round1          Generate signing commitment (each participant)
threshold-create-package  Bundle message + commitments (coordinator)
threshold-round2          Generate signature share (each participant)
threshold-aggregate       Combine shares into final signature
threshold-verify          Verify a signature
threshold-sign            All-in-one for testing (not for production)
```

---

## Build

```
cargo build --release -p nonos-threshold-sign
```

---

## Distributed signing

This is how it works when participants don't trust each other and run on separate machines.

### Setup (once)

Someone trusted generates the key shares:

```
threshold-keygen -t 3 -n 5 -o keys/
```

This creates `key_share_1.json` through `key_share_5.json` plus `public_key_package.json`. Distribute each share to its participant over a secure channel. Delete the shares after distribution - the coordinator shouldn't keep copies.

### Round 1

Each participant runs independently:

```
threshold-round1 -k my_key_share.json -c my_commitment.json -n my_nonces.json
```

They send `my_commitment.json` to the coordinator and keep `my_nonces.json` secret.

### Package creation

Coordinator collects commitments and bundles them with the message:

```
threshold-create-package -m message.bin -c commit_1.json commit_2.json commit_3.json -o signing_package.json
```

Sends `signing_package.json` back to all participants.

### Round 2

Each participant generates their signature share:

```
threshold-round2 -p signing_package.json -n my_nonces.json -k my_key_share.json -o my_sig_share.json
```

They send `my_sig_share.json` to the coordinator and delete `my_nonces.json` - nonces must never be reused.

### Aggregation

Coordinator combines the shares:

```
threshold-aggregate -p signing_package.json -s sig_1.json sig_2.json sig_3.json -k public_key_package.json -o signature.bin
```

Output is a standard 64-byte signature.

### Verification

Anyone can verify with just the public key:

```
threshold-verify -m message.bin -s signature.bin -k public_key_package.json
```

---

## Testing locally

For development, you can run everything on one machine:

```
threshold-keygen -t 2 -n 3 -o test_keys/

threshold-sign \
  -m message.txt \
  -k test_keys/key_share_1.json test_keys/key_share_2.json \
  -p test_keys/public_key_package.json \
  -o signature.bin

threshold-verify -m message.txt -s signature.bin -k test_keys/public_key_package.json
```

This is for testing only. Production deployments must use the distributed workflow.

---

## Security

**What's protected:**
- Secret shares are zeroized in memory on drop
- Nonces are zeroized after signing
- No party ever reconstructs the full private key
- t-1 colluding parties learn nothing about the key

**What you must do:**
- Distribute key shares over secure channels
- Store key shares in protected storage (HSM if possible)
- Never reuse nonces - delete them after round 2
- Verify the coordinator isn't malicious (commitments are binding)

**Domain separation:**
```
NONOS:FROST:v1           Base domain
NONOS:FROST:COMMIT:v1    Binding factor
NONOS:FROST:CHALLENGE:v1 Schnorr challenge
```

---

## File formats

**Key share** (JSON, keep secret):
```json
{
  "participant_id": 1,
  "secret_share": "hex...",
  "public_share": "hex...",
  "group_public_key": "hex...",
  "verification_shares": [...],
  "config": {"threshold": 3, "total_signers": 5}
}
```

**Public key package** (JSON, share freely):
```json
{
  "group_public_key": "hex (32 bytes)",
  "verification_shares": {"1": "hex...", ...},
  "config": {"threshold": 3, "total_signers": 5}
}
```

**Signature** (binary, 64 bytes):
```
[R: 32 bytes][s: 32 bytes]
```

Standard Ed25519 format. Works with any Ed25519 verifier.

---

## Bootloader integration

The bootloader will verify FROST signatures on:
- Kernel binaries (replacing current multi-sig)
- Configuration updates

A single `group_public_key` gets embedded in the bootloader. At runtime, it verifies that the signature was created by at least t of the n authorized parties - without needing to know who specifically signed.

---

## References

- FROST paper: https://eprint.iacr.org/2020/852
- Ristretto: https://ristretto.group/
- Ed25519: RFC 8032

---

## License

AGPL-3.0
