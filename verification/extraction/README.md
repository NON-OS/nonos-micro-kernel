# Rust to Lean extraction

This directory makes the refinement between the Lean specification and the
kernel machine-checked instead of nominal. The pipeline lowers the real
kernel source to Lean through the compiler's own MIR and proves the security
semantics about the result:

```
src/capabilities/bits.rs          the code that ships
        |  rustc MIR, via Charon (#[path] include, never a copy)
        v
caps/caps.llbc                    the borrow-checked IR of that exact code
        |  Aeneas, Lean backend
        v
lean/NonosExtraction/Caps.lean    the same functions as Lean definitions
        |  lean/NonosExtraction/Refinement.lean
        v
Grants / capsOf semantics         the objects the lattice theorems quantify over
```

`Caps.lean` is generated, never edited; regenerating it from an unchanged
kernel is byte-stable, and any kernel change shows up as a diff here. The
theorems in `Refinement.lean` prove the extracted `has_capability`,
`add_capability`, and `remove_capability` are total and compute exactly the
`capsOf` denotation from `verification/lean/Nonos/CapabilityBits.lean`, so
every lattice theorem proven there now holds of the extracted-from-real-Rust
definitions by transitivity.

## Scope, honestly stated

Extraction covers the pure safe decision core: the capability bit operations
and their call graph (`Capability::bit`). The `Vec`-returning conversions in
the same file use iterator adapters outside Aeneas's supported fragment and
stay covered by the kernel_proofs differential harnesses and Kani. The
unsafe hardware glue is out of extraction scope by design and remains under
Kani and Verus.

## Trusted base and pins

- Charon `nightly-2026.07.02` (v0.1.218), Aeneas `nightly-2026.07.06-45061fa`,
  prebuilt release binaries; Charon drives rustc `nightly-2026-06-01`.
- The Aeneas Lean library at the same commit, with Lean `v4.30.0-rc2` and
  mathlib as its dependency. The core `verification/lean` package stays
  core-only on its own pinned toolchain; this project rebuilds it from the
  same sources under rc2 through a lake path dependency.
- The claim "Caps.lean is the real bits.rs" rests on Charon reading the MIR
  of the included file and on Aeneas's translation soundness, the same way
  the host proofs rest on rustc.
- The refinement theorems depend on Lean's standard axioms only (propext,
  Classical.choice, Quot.sound); `#print axioms` shows no sorry. The Aeneas
  library itself carries sorries in modules these proofs do not use (Slice,
  StringIter); they do not enter the axiom profile of any theorem here.

## Reproduce

```sh
cd verification/extraction/caps
charon cargo --preset=aeneas \
  --start-from 'nonos_caps::capabilities::bits::has_capability' \
  --start-from 'nonos_caps::capabilities::bits::add_capability' \
  --start-from 'nonos_caps::capabilities::bits::remove_capability' \
  --dest-file caps.llbc
aeneas -backend lean caps.llbc -dest ../lean/NonosExtraction
cd ../lean && lake exe cache get && lake build   # 0 errors == verified
```
