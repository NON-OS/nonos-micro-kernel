# NONOS Verus proofs

Machine-checked theorems about the NONOS security algebra, verified by
[Verus](https://github.com/verus-lang/verus) (SMT deductive verification of
Rust). These are checked by the verifier, not executed at kernel time.

## What is proven

`src/capabilities.rs` mirrors `src/capabilities/bits.rs` exactly (a right is a
single power-of-two bit, a token is the OR of its rights) and proves, over all
`u64` values:

- **revoke is monotonic**: revoking never adds authority.
- **revoke drops the right**: the revoked bit is gone.
- **attenuation confines**: an attenuated token never gains a right the parent
  lacked (object-capability confinement).
- **grant preserves and adds**: granting keeps existing rights and adds exactly
  the requested one.
- **empty token grants nothing**: authority cannot be conjured from nothing.

`src/page_permissions.rs` mirrors the page-table permission bits used by
`PagePermissions::to_pte` and proves present-bit, writable-bit, user-bit,
executable/NX, non-WX, and permission-subset monotonicity properties.

`src/ipc_lengths.rs` mirrors the IPC `MAX_MESSAGE_SIZE` gate and proves that
zero-length and oversized messages are rejected while accepted lengths are
bounded by the shared `1..=1048576` rule.

Because the spec functions are the kernel's own bit operations, these are
properties of the code the kernel runs, not of a separate abstract model.

## Verify

Install the Verus toolchain (pinned in CI), then:

```sh
verus --crate-type=lib src/lib.rs
```

A clean run prints `verification results:: N verified, 0 errors`.
