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
- `phys::bitmap`: the bit-index arithmetic in
  `src/memory/phys/bitmap/index.rs`, which `bit_ops.rs` calls. A mask selects
  exactly one bit, and the byte and bit position reconstruct the index
  losslessly. Lean: `Nonos/Bitmap.lean`.
- `region`: the half-open range algebra in `src/memory/region/overlap.rs`, which
  `MemRegion::overlaps`, `contains` and `contains_range` delegate to. Overlap is
  symmetric and is exactly the negation of disjointness. Lean:
  `Nonos/Interval.lean` and `Nonos/Vma.lean`.
- `timer`: the load-balancer elapsed-tick test in
  `src/process/scheduler/smp/interval.rs`, which `should_balance` delegates to. A
  tick wraparound saturates to no time elapsed. Lean: `Nonos/Timer.lean`.
- `quota`: the resource-token check in `src/capabilities/resource/limits.rs`,
  which `has_bytes` and `has_ops` delegate to. A request is covered exactly when
  it is within the remaining budget. Lean: `Nonos/Quota.lean`.
- `ring`: the input-ring index arithmetic in
  `src/kernel_core/surface_registry/ring_math.rs`, which `input_ring.rs` calls. A
  wrapped index stays within the capacity, and a full ring is detected when the
  head would reach the tail. Lean: `Nonos/Ring.lean`.
- `mmio`: the window-validity arithmetic in
  `src/drivers/security/mmio_range.rs`, which `validate_mmio_region` delegates
  to. A valid window is non-empty and does not wrap the address space. Lean:
  `Nonos/Mmio.lean`.

Run:

    cargo test --release
    cargo kani
