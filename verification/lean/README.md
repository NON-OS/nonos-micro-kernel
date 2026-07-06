# NØNOS Lean specification proofs

Lean 4 proofs of NØNOS's **abstract security theorems**. They are the top of a
refinement chain: Lean states and proves the property about a mathematical
model, and the code-level proofs elsewhere in this repository discharge that
same property on the implementation that actually runs.

```
Lean (this directory)      abstract security theorem        "the model is safe"
        │  refined onto
        ▼
Verus / Kani               the real Rust operations         "the code matches the model"
        │  rests on
        ▼
Runnable KATs / fuzz       the real primitives              "the primitives are correct"
```

This is the difference from a Lean-only kernel: a model proof on its own has a
model-implementation gap. Here each Lean theorem has a named counterpart proving
the *code*:

| Lean theorem | Refined onto the code by |
|---|---|
| `AntiRollback.update_never_lowers_floor`, `no_rollback_after_boot` | `nonos-bootloader/boot_proofs` (runnable + Kani over all u64) |
| `Capability.grant_adds` / `revoke_drops` / `attenuate_confines` | `verification/verus/src/capabilities.rs` (Verus over the bit ops) |

The proofs use only core Lean (no mathlib), so any recent `leanprover/lean4`
toolchain checks them.

## Reproduce

```sh
cd verification/lean
lake build          # 0 errors == all theorems verified
```

The `lean` CI job runs this on every push.
