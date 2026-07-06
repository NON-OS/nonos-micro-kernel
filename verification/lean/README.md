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
| `Isolation.no_wx_page` | `userland/kernel_proofs` W^X (`to_pte_flags`, runnable + Kani) |
| `Isolation.accepted_stays_in_user_space` | `userland/kernel_proofs` user-copy (`check_range`, runnable + Kani) |
| `Authorization.empty_token_denied` / `allow_monotone` | `userland/kernel_proofs` cap-table (`is_allowed`, runnable) |
| `Ipc.zero_length_rejected` / `oversized_rejected` | Verus `ipc_lengths.rs` + `IpcMessage::new` |
| `Paging.subset_trans` / `confined_preserves_no_wx` | Verus `page_permissions.rs` (bit ops) |
| `Crypto.wrong_tag_rejected` / `acceptance_iff_equal` | `userland/crypto_proofs` `ct_eq` correctness + KATs (#265) |
| `Path.leading_dotdot_neutralized` / `up_moves_toward_root` | `userland/fs_proofs` path normalization (Kani + runnable) |
| `Capability.attenuate_is_glb` / `grant_is_lub` | Verus `capabilities.rs` confinement legs; the bound-optimality legs are model-side algebra |
| `Paging.meet_is_glb` / `subset_antisymm` | Verus `page_permissions.rs` (`permission_subset_is_monotonic`) |
| `Authorization.denied_below` / `subsumes_trans` | `userland/kernel_proofs` cap-table (contrapositive of `allow_monotone`) |

The proofs use only core Lean (no mathlib), so any recent `leanprover/lean4`
toolchain checks them.

## Reproduce

```sh
cd verification/lean
lake build          # 0 errors == all theorems verified
```

The `lean` CI job runs this on every push.
