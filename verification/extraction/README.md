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

Extraction covers the pure safe decision cores: the capability bit
operations and their call graph (`Capability::bit`) in `caps/`, and the
user-copy range policy (`check_range`, with the exact error variant on every
rejecting path and totality on every input) plus the page-permission
encoding (`to_pte_flags`, `is_wx_violation`) in `policy/`. The proofs in
`lean/NonosExtraction/PolicyRefinement.lean` bind the policy to
`Nonos.Isolation`: acceptance is exactly the model's `Accepts`, and the
extracted encoder can never emit a writable page without NX unless the
permission was a W^X violation. The `Vec`-returning conversions in bits.rs
use iterator adapters outside Aeneas's supported fragment and stay covered
by the kernel_proofs differential harnesses and Kani. The unsafe hardware
glue is out of extraction scope by design and remains under Kani and Verus.

One external definition is provided by hand, as Aeneas prescribes for core
functions it treats as opaque: `Option::ok_or`, four lines in
`lean/Policy/FunsExternal.lean`, entering the trusted base with the
documented core semantics. Everything else in `lean/Policy` and
`lean/NonosExtraction/Caps.lean` is generated and never edited.

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

cd ../policy
charon cargo --preset=aeneas \
  --start-from 'nonos_policy::usercopy::policy::check_range' \
  --start-from 'nonos_policy::to_pte_flags' \
  --start-from 'nonos_policy::is_wx_violation' \
  --dest-file policy.llbc
aeneas -backend lean -split-files policy.llbc -dest ../lean/Policy
# FunsExternal.lean is hand-written from FunsExternal_Template.lean; do not
# overwrite it.

cd ../lean && lake exe cache get && lake build   # 0 errors == verified
```
