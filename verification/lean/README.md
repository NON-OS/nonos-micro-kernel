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
| `Spawn.only_attested_capsules_run` | `src/security/capsule_attest` `#[must_use]` gate + `src/kernel_core/.../attest_gate.rs` (admit only on Ok) |
| `BootImage.accepted_region_stays_in_bounds` / `an_escaping_region_is_rejected` | `nonos-bootloader/boot_proofs` footer parser (Kani total + ~125k crafted footers) |
| `Loader.accepted_table_inside_file` / `truncated_header_rejected` | `userland/kernel_proofs` `elf_tests.rs` (`program_header_table_never_overflows_or_escapes_the_file`) |
| `Syscall.decode_is_total` / `known_ids_round_trip` | `userland/kernel_proofs` (`syscall_decode_is_total`, `syscall_id_decode_and_registry_agree_for_all_ids`, Kani all u64) |
| `NetParse.accepted_payload_in_bounds` / `a_pointer_ends_the_walk` | `userland/net_proofs` (`compression_pointers_terminate_and_do_not_loop`) + `fs_proofs` parser Kani harnesses |
| `BlockIO.accepted_request_stays_on_disk` / `an_escaping_request_is_rejected` | `userland/driver_proofs` (`rw_parse_is_total_and_bounded`, runnable + Kani) |
| `UsbHid.a_zero_length_record_is_rejected` / `bindings_never_exceed_the_cap` | `userland/usb_proofs` (`descriptor_walk_terminates_and_binding_count_is_bounded`) |
| `Stark.Field` ring laws / `results_are_canonical` | `userland/stark_proofs` `field_tests.rs` on the real Goldilocks code; multiplicative inverses stay code-side (`every_nonzero_element_has_an_inverse`) |
| `Stark.Merkle.acceptance_iff_recomputation` / `distinct_leaves_give_distinct_roots` | `userland/stark_proofs` `merkle_tests.rs`; binding is conditional on compression injectivity (BLAKE3 collision resistance stays assumed) |
| `Stark.Attest.accepted_leaf_is_enrolled` / `a_proof_for_one_capsule_is_rejected_for_another` | `userland/stark_proofs` `air_tests.rs` (`an_enrolled_capsule_attestation_is_accepted`, `attestation_is_denied_for_a_different_capsule_or_root`); composes Merkle binding + Fiat-Shamir binding |
| `Stark.Fri.enough_folds_reach_a_constant` / `a_mismatched_final_layer_is_rejected` | `userland/stark_proofs` `fri_tests.rs` (honest low-degree verifies, high-degree rejected) + the `FinalLayerConstant` AIR in `recursive_verifier_tests.rs` |
| `Stark.Lookup.non_table_value_has_zero_multiplicity` / `present_value_has_positive_multiplicity` | `userland/stark_proofs` `air_tests.rs` (`an_out_of_table_value_is_rejected`, `the_lookup_is_sound_under_a_transcript_challenge`) |
| `Stark.Permutation.perm_preserves_count` / `a_missing_value_is_not_a_permutation` | `userland/stark_proofs` `air_tests.rs` (`a_permutation_argument_verifies`, `a_non_permutation_is_rejected`) |
| `Stark.CopyConstraint.wiring_forces_equality` / `a_broken_wire_is_rejected` | `userland/stark_proofs` `air_tests.rs` (`a_value_is_bound_across_two_fused_regions`) |
| `Stark.Polynomial.eval_mul` / `zerofier_vanishes_at_a_point` / `zerofier_nonzero_off_the_points` | `userland/stark_proofs` `poly_tests.rs` (`evaluation_matches_the_defining_sum`, `the_low_degree_extension_recovers_the_polynomial`) |
| `Stark.Transcript.challenge_is_determined` / `order_changes_the_state` | `userland/stark_proofs` `transcript_tests.rs` (determinism, one-bit change re-randomizes, `absorb_order_matters`) |
| `Attestation.no_impersonation` / `sender_independent_of_claim` | `userland/fs_proofs` caller attestation (kernel mirror path, impersonation rejection, `proof_split_caller_no_impersonation` Kani) |
| `Zeroization.no_secret_survives` / `reused_region_leaks_nothing` | kernel zeroization on capsule teardown and page reclaim |

The proofs use only core Lean (no mathlib), so any recent `leanprover/lean4`
toolchain checks them.

## Reproduce

```sh
cd verification/lean
lake build          # 0 errors == all theorems verified
```

The `lean` CI job runs this on every push.
