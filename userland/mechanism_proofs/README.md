# mechanism_proofs

Host-runnable proofs binding kernel mechanisms to the properties their Lean
models state. Each module pulls in the real Rust via `#[path]` and runs it, so
the property is proven of the code the kernel executes, not a copy of it.

Modules land here as their Lean model moves from a specification to a code-bound
proof. The binding level of every model is recorded in
`verification/lean/REFINEMENT.md`.

Bound so far:

- `buddy`: the order, size and buddy-address arithmetic in
  `src/memory/buddy_alloc/constants/helpers.rs`, which the allocator runs. A
  split conserves size (`order_to_size(k+1) == 2 * order_to_size(k)`) and the
  buddy address is an involution (`buddy_address(buddy_address(a, o), o) == a`).
  Lean: `Nonos/Buddy.lean`.

Run:

    cargo test --release
    cargo kani
