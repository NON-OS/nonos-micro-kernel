# NØNOS verification architecture

This document describes how NØNOS is verified and what the verification does and
does not establish. It is written for engineers and cryptographers who want to
audit the claims rather than take them on faith. Every property below is checked
by machine on the code in this repository, and every check is reproducible from
a clean checkout.

## The thesis

A proof is only as useful as the distance between the thing proved and the thing
that runs. A kernel that carries a formal model in Lean or Coq has proved
something about the model. Whether the C or Rust that boots on the machine
refines that model is a separate question, and usually an unanswered one. That
gap is where real defects live.

NØNOS is verified on the other side of that gap. The proofs run the actual
`src/` and capsule source, included unmodified through Rust's `#[path]`
mechanism, and check it against the properties we care about. Where a property
is naturally stated as an abstract theorem, we state it in Lean and then show,
with a second proof over the real code, that the implementation satisfies it. The
model is never left standing on its own.

## The four instruments

We use four verification techniques, each where it is the right tool.

Runnable known-answer tests and fuzzing check the real primitives against the
published standards and against large adversarial input sets. This is the right
instrument for cryptographic primitives, where correctness means agreement with
a specification vector, and for parsers, where safety means no panic and no
out-of-bounds access on any input. The crates under `userland/*_proofs` and
`nonos-bootloader/boot_proofs` include the real source and run it on the host.

Kani model-checks the same code over all inputs within a bound rather than a
sample. It discharges totality claims: a decode never panics, an arithmetic
step never overflows, a returned slice never escapes its buffer, for every input
of the checked shape. Kani harnesses sit beside the runnable tests under
`#[cfg(kani)]`.

Verus proves deductive properties directly on Rust, with an SMT backend. It is
used for the bit-level security algebra of the kernel: capabilities, IPC length
gates, and page-table permission encodings. See `verification/verus`.

Lean 4 states the abstract security theorems and proves them about a
mathematical model. Each Lean theorem names the code proof that discharges the
same property on the implementation, so the model connects to what runs. See
`verification/lean`.

## How the layers connect

For every property that has both a specification and an implementation proof,
the two are linked by name. The Lean theorem `AntiRollback.no_rollback_after_boot`
states that once a version boots, no strictly older version is ever accepted
again. The Rust and Kani harnesses in `boot_proofs` prove the real bootloader
`check_kernel_version` and `update_kernel_version` satisfy exactly that, over
every `u64`. The Lean theorem `Capability.attenuate_confines` states that an
attenuated token grants nothing the parent lacked; the Verus proof in
`capabilities.rs` proves the kernel's `bits & mask` operation implements it. The
mapping is recorded in `verification/lean/README.md`.

The result is a refinement chain: an abstract security theorem in Lean, refined
onto the real Rust by Verus or Kani, resting on primitives checked against the
standard vectors. A reader who trusts the chain need only audit the links.

## Coverage by subsystem

### Cryptography

The crate `userland/crypto_proofs` includes the real `src/crypto` source and
checks it against the official vectors:

- SHA-256 and SHA-512 against NIST FIPS 180-4, including the one-million-byte
  long message.
- SHA-3-256 and SHA-3-512 against NIST FIPS 202.
- BLAKE3 hash, keyed hash, and derive-key against the reference test set.
- HMAC-SHA256 against RFC 4231, including that the constant-time verify rejects a
  tampered tag.
- HKDF-SHA256 extract and expand against RFC 5869.
- ChaCha20-Poly1305 against RFC 8439, including that decryption rejects a tampered
  tag, ciphertext, or associated data.
- AES-128-GCM against the NIST GCM test cases, with and without associated data.
- Ed25519 against RFC 8032: public-key derivation, deterministic signing byte for
  byte, and verification, plus rejection of flipped R, flipped S, altered
  messages, and wrong keys.
- P-256 and P-384 ECDSA verification against RFC 6979.
- secp256k1 scalar multiplication anchored on the SEC 2 generator, plus a
  deterministic RFC 6979 sign and verify round trip.
- RSA PKCS#1 v1.5 (SHA-256) verification of a 2048-bit signature produced by
  OpenSSL, so the check is proven to interoperate with a reference implementation
  rather than only with itself.

The constant-time comparison, selection, and integer-predicate primitives that
gate every MAC, tag, and signature check are proven functionally correct
(equal to the ordinary operation they replace) over roughly a million sampled
inputs and by Kani over all inputs. A masking bug in these would silently accept
or reject; the proof rules that out. The timing property itself is by
construction: the code is branch free and touches every byte.

Ed25519 is the trust root. The bootloader and the capsule loader execute code
only if this verifier accepts the signature, so proving it is byte for byte the
RFC 8032 algorithm is proving the gate that admits all other code.

### Boot

The crate `nonos-bootloader/boot_proofs` runs the real `security::anti_rollback`
decision logic and the real `image_format` footer parser, with only the TPM and
NVRAM write shimmed. It proves that version zero is rejected, that nothing boots
without a trusted floor, that booting a version raises the floor and no older
version boots afterward, that a too-old boot leaves the stored state untouched
(the check runs before any commit), and that the floor never decreases. Over
roughly 125,000 crafted image footers with adversarial region offsets and sizes,
the parser never panics and never returns a region slice that escapes the input.
Kani extends the core claims over all inputs.

### Kernel isolation and authorization

The crate `userland/kernel_proofs` runs the real kernel source for the memory
and authorization boundary:

- W^X. A permission set that is not a write-execute violation never encodes a
  page-table entry that is both writable and executable, so a mapper that rejects
  `is_wx_violation` cannot install a W+X page. Proven over the real `to_pte_flags`
  encoding, runnable and by Kani.
- User copy. The `check_range` guard on every kernel-to-user copy is total, and
  an accepted range is page aligned and lies inside user space without wrapping.
- Syscall decode. Decoding an untrusted `u64` syscall id never panics for any
  value, the id table is consistent with the name table, and known ids round
  trip.
- Authorization. The real `is_allowed` cap table denies an empty token every
  syscall, permits a crypto syscall only for a token that grants the crypto
  capability, and never removes access when a capability is added.
- Loader. The capsule ELF header and program-header-bounds parsers reject a
  truncated header and never let `phoff + phnum * phentsize` overflow or run past
  the file, over adversarial headers with large offsets and counts.

### IPC and paging

The Verus crate proves the kernel's IPC length gate (zero rejected, oversized
rejected, accepted lengths bounded, send and reply sharing one gate) and the
page-permission encoding (present, writable, user, and no-execute bits matching
their permissions, no W+X under a non-violating permission, and permission
subset monotonic). The Lean modules `Ipc` and `Paging` state the same
properties abstractly.

### Network

The crate `userland/net_proofs` runs the real capsule parsers for DNS, ICMP,
ARP, TCP, and DHCP over large adversarial input sets. The DNS response parser
terminates and never panics across every two-byte compression-pointer value,
including self-referential pointers, so the classic compression-pointer loop is
absent. The ICMP and TCP parsers never return a payload slice outside the input.
The TCP reassembly buffer never panics on hostile streams of overlapping,
out-of-order, and sequence-wrapping segments, and joins contiguous data in order
while stopping at gaps. The DHCP option parser rejects an option whose length
runs past the packet rather than reading out of bounds.

### Drivers

The crate `userland/driver_proofs` proves the AHCI read/write request parser
never accepts a request that reaches past the disk: an accepted request has a
bounded sector count and `lba + count` neither overflows nor exceeds capacity.
The crate `userland/usb_proofs` proves the USB HID configuration-descriptor
parser, which runs on fully device-controlled data, never panics, rejects a
descriptor claiming a zero length rather than looping on it, and never returns
more bindings than its fixed cap.

## Reproducing the proofs

Each layer is checked independently.

```sh
# Runnable proofs and fuzzing (host)
for c in crypto_proofs net_proofs kernel_proofs driver_proofs usb_proofs; do
  ( cd userland/$c && cargo test --release )
done
( cd nonos-bootloader/boot_proofs && cargo test --release )

# Kani, all-input model checking
( cd userland/kernel_proofs && cargo kani )

# Verus, deductive proofs on Rust
( cd verification/verus && verus --crate-type=lib src/lib.rs )

# Lean, specification theorems
( cd verification/lean && lake build )
```

The continuous integration workflow runs all of these on every push. Each proof
job reports success only when its checker reports zero errors.

## What we do not claim

The verification is broad but bounded, and it is worth being exact about the
edges.

We prove the cryptographic primitives are functionally correct against their
standards and that their comparison logic is constant time by construction. We
do not run a timing side-channel analysis such as dudect, and we do not run a
static constant-time verifier such as ct-verif. A residual timing leak in code
we have not marked constant time would not be caught here.

We prove memory safety and the stated security invariants on the surfaces listed
above. We do not prove full functional correctness of the whole kernel. That is a
different and far larger undertaking, on the scale of seL4, and it is not what
this stack sets out to do.

The post-quantum signature path (ML-DSA-65) is a C library reached through an
FFI shim. It cannot be host-included as Rust and so is not covered by these
crates. It requires a bootloader-side FFI harness, which is future work.

The Kani and Verus and Lean proofs are checked by their respective tools, pinned
to specific versions in CI. The runnable proofs are checked by `cargo test`.
None of these tools proves the absence of a defect outside the property it
checks.

## Findings

Writing the proofs surfaced two defects in the kernel ZK verifier, both fixed in
the same change that added the proof that catches them. The PLONK verifier's
`ct_is_all_zero` returned one for an all-zero input but `0xFE` otherwise, so the
verifier's `1 - ct_is_all_zero(commitment)` underflowed for any nonzero
commitment: a panic in debug, and in release a wrapped value that corrupts the
validity flag. The range verifier accepted a degenerate zero-bit proof, in which
the structure checks pass vacuously and no bit proofs are examined, so attacker
bytes verified as valid. Both are fixed and both are now covered by the fuzz that
found them.

One hygiene issue was reported rather than changed: the NVMe identify parsers
read fixed offsets with no length check on their byte-slice argument. This is
safe today because the only caller passes a fixed 4096-byte DMA slice, but the
public function does not enforce that precondition and would panic on a short
slice. It is left to the driver owner because it is not currently reachable.
