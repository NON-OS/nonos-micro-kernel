# Refinement status

A Lean theorem proves a property of a Lean model. Whether that says anything
about the running kernel depends on how tightly the model is bound to the Rust.
This file states, honestly, the binding level of each module, so a reader can
tell a proof about the code from a proof about a model that the code is meant to
match.

There are three levels.

```
  level 1  extraction        the Rust is translated to Lean by Aeneas and the
                             model is proven to refine the extracted code
  level 2  differential      a proof crate includes the real Rust via #[path]
           and model check    and checks it against the model over the input
                             space (sampled by cargo test, exhaustive by Kani)
  level 3  specification      the model is machine-checked in Lean and states
                             the property the implementation must satisfy; the
                             mechanical tie to the Rust is not yet in place
```

A level 3 entry is not a claim about the code. It is a checked statement of the
contract the code owes. Moving an entry to level 2 or 1 is tracked work.

## Level 1: extraction (Aeneas)

| Module | Kernel code | Binding |
| --- | --- | --- |
| `Capability`, `CapabilityBits`, `Authorization` | `src/capabilities` | `verification/extraction` translates the Rust with Charon and Aeneas into `NonosExtraction/Caps.lean`; `Refinement.lean` and `PolicyRefinement.lean` prove the model refines it |

## Level 2: differential and Kani proof crate

| Module | Kernel code | Proof crate |
| --- | --- | --- |
| `Semaphore` | `src/sys/sync/semaphore` | `userland/sync_proofs` (differential tests and Kani over the permit arithmetic in `pure.rs`) |
| `Seqlock` | `src/sys/sync/seqlock` | `userland/sync_proofs` (differential tests and Kani over the sequence discipline in `pure.rs`) |
| `Buddy` | `src/memory/buddy_alloc` | `userland/mechanism_proofs` (differential tests and Kani over the order and buddy-address arithmetic in `constants/helpers.rs`) |
| `Bitmap` | `src/memory/phys/bitmap` | `userland/mechanism_proofs` (differential tests and Kani over the bit-index arithmetic in `index.rs`) |
| `Isolation`, `Paging` | `src/memory/paging` | `userland/kernel_proofs` (page permission W xor X, over all bit patterns) |
| `Loader` | `src/elf` loader | `userland/kernel_proofs` (segment bounds) |
| `Syscall` | `src/syscall` numbers | `userland/kernel_proofs` (decode totality and registry agreement) |

The proof crates run in CI: `.github/workflows/verify.yml` runs each crate's
`cargo test` and clippy, and a Kani job runs the model checks for all inputs.

## Level 3: specification, binding pending

These modules are machine-checked in Lean with no `sorry`, and each names the
kernel subsystem it abstracts. The mechanical tie to that code is the backlog.

| Module | Kernel subsystem |
| --- | --- |
| `Mutex` | `src/sys/sync/irq_mutex` |
| `Rwlock` | `src/sys/sync/irq_rwlock` |
| `Spinlock` | `spin::Mutex` wrappers in `src/sys/sync` |
| `Vma`, `Interval`, `PageTable`, `Bounds` | `src/memory` address space and paging |
| `MemGrant`, `Quota`, `Refcount`, `Heap`, `Zeroize` | `src/memory` grants and allocation |
| `Iommu`, `Mmio` | device DMA and register windows under `src/hardware` |
| `Tlb` | TLB shootdown under `src/memory` |
| `CapTable`, `Dispatch` | capability tables and the syscall dispatch under `src/syscall` |
| `Scheduler`, `Priority`, `Timer`, `Ring` | `src/process/scheduler` |
| `Epoch` | RCU-style reclamation across the kernel |
| `Signal` | signal delivery |
| `Reaper` | the process table |
| `Nonce`, `Rng` | entropy and nonce issuance under `src/crypto` |
| `Endpoint`, `Fd`, `Vfs` | IPC endpoints, descriptor tables, and the VFS |
| `TokenBucket` | rate limiting in the network path |
| `Cow` | copy-on-write pages |

`Futex` already has a code-bound proof: the wait-queue discipline in
`src/syscall/microkernel/futex/queue.rs`, which `wait.rs` and `wake.rs` call, is
checked against `Nonos/Futex.lean` by `mechanism_proofs` (first in first out
order, exact wake count, no lost waiter). The futex module lives on a feature
branch not yet merged to main, so that binding lands at level 2 when the branch
does.

The attestation, anti-rollback and STARK modules bind to code through the
differential harnesses and published-vector checks described in the top-level
`ARCHITECTURE.md`.
