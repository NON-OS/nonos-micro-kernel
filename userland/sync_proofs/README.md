# sync_proofs

Host-runnable proofs that the kernel synchronisation primitives satisfy the
properties their Lean models state.

The real permit arithmetic of the counting semaphore (`src/sys/sync/semaphore`)
and the sequence discipline of the seqlock (`src/sys/sync/seqlock`) are pulled
in verbatim through `#[path]`, so the proofs run against the code the kernel
executes, not a copy of it.

- `src/spec` restates the contract in plain Rust, independent of the
  implementation.
- `src/refinement_tests.rs` runs the real functions against the spec over
  sampled inputs (`cargo test`).
- `src/kani_proofs.rs` proves the same properties for every input with Kani
  (`cargo kani`).

The Lean models are `verification/lean/Nonos/Semaphore.lean` and
`verification/lean/Nonos/Seqlock.lean`.

Run:

    cargo test --release
    cargo kani
