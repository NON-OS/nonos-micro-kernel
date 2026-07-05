# NONOS verification

NONOS proves its guarantees about the code that actually runs, and re-runs
those proofs on every push. Three layers, strongest at the bottom:

## 1. Runnable proofs

A host crate that includes the **real capsule source** via `#[path]` (only the
syscall clock is shimmed) and executes it with `cargo test`:

- **Store operations**: mkdir-p, chmod enforcement, truncate zero-fill,
  recursive copy/rmdir, fd reindex, child counts, usage, mtime.
- **Path security**: canonicalization and the `/capsules` read-only guard,
  including slash-smuggling.
- **Protocol codec**: hostile/malformed input handling.
- **Caller attestation**: userspace impersonation is rejected.
- **File-manager logic**: listing parse, dedup, type classification.
- **Wire parser proofs**: Ethernet, IPv4, and UDP parser bounds and round trips.
- **Fuzz proofs**: millions of structured and random inputs asserting the
  parsers never panic and never violate their invariants (no-impersonation,
  path canonicalization).

Writing these already found and fixed real bugs (directory dedup, dotfile
classification). Run:

```sh
cd userland/fs_proofs
PATH="$HOME/.cargo/bin:$PATH" cargo test --release
```

## 2. Kani

`userland/fs_proofs/src/kani_proofs.rs` proves, over every input (bounded), that
the untrusted-input surfaces are **panic-free and UB-free**, plus the
**authority** theorem (no userspace impersonation) and the **canonicalization**
theorem (every normalized path is rooted and slash-clean). It also proves
bounded Ethernet, IPv4, and UDP parser payload bounds. Run:

```sh
cd userland/fs_proofs
PATH="$HOME/.cargo/bin:$PATH" cargo kani --output-format terse
```

## 3. Verus

`verification/verus` proves theorems about the kernel's own capability bit
operations, page-table permission encoding, and IPC message length guards. Run:

```sh
cd verification/verus
verus --crate-type=lib src/lib.rs
```

## Why this is the strong position

A model-checked or SMT-verified theorem about a separate abstract model still
has to trust that the running implementation matches the model. Here the specs
*are* the code the kernel runs (bit operations, the real parsers, the real
store), so there is **no model-implementation gap**. Machine-checked rigor is
applied to implementation paths, not to a detached sketch.

All three layers run in CI (`.github/workflows/verify.yml`) on every push, so
the guarantees are reproducible by anyone, not asserted.
