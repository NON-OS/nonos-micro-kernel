/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The axiom profile of the specification layer, checked by Lean itself rather
than asserted. `#print axioms` prints the exact axiom closure of a theorem:
for every flagship theorem below the expected output names at most Lean's
three standard axioms (propext, Classical.choice, Quot.sound) and never
`sorryAx`, which is what a `sorry` would introduce. The CI lean job runs this
file and publishes the output as evidence, so a green run leaves a record of
what was proven and on what foundations, not just an exit code.
-/

import Nonos

#print axioms Nonos.AntiRollback.no_rollback_after_boot
#print axioms Nonos.Authorization.empty_token_denied
#print axioms Nonos.BlockIO.accepted_request_stays_on_disk
#print axioms Nonos.BootImage.accepted_region_stays_in_bounds
#print axioms Nonos.Capability.attenuate_confines
#print axioms Nonos.CapabilityBits.word_chain_never_widens
#print axioms Nonos.Crypto.wrong_tag_rejected
#print axioms Nonos.Ipc.zero_length_rejected
#print axioms Nonos.Isolation.no_wx_page
#print axioms Nonos.Loader.accepted_entry_inside_file
#print axioms Nonos.NetParse.a_pointer_ends_the_walk
#print axioms Nonos.Paging.confined_preserves_no_wx
#print axioms Nonos.Path.leading_dotdot_neutralized
#print axioms Nonos.Secure.every_trace_is_secure
#print axioms Nonos.Spawn.only_attested_capsules_run
#print axioms Nonos.Stark.AssociationSet.an_excluded_deposit_cannot_pass
#print axioms Nonos.Stark.AssociationSet.the_registry_only_grows
#print axioms Nonos.Stark.Attest.a_proof_for_one_capsule_is_rejected_for_another
#print axioms Nonos.Stark.Constraint.constraint_holds_iff_quotient_exists
#print axioms Nonos.Stark.Constraint.the_quotient_is_pinned
#print axioms Nonos.Stark.CopyConstraint.wiring_forces_equality
#print axioms Nonos.Stark.Field.mul_distributes_over_add
#print axioms Nonos.Stark.Extension.low_degree_extension_is_unique
#print axioms Nonos.Stark.Commitment.commitment_binds_value
#print axioms Nonos.Stark.FeeRouter.route_conserves
#print axioms Nonos.Stark.FeeRouter.an_accepted_fee_is_within_cap
#print axioms Nonos.Stark.Fold.eval_split
#print axioms Nonos.Stark.Instance.honest_run_has_a_quotient
#print axioms Nonos.Stark.Instance.tampered_run_has_no_quotient
#print axioms Nonos.Stark.Fold.fold_length
#print axioms Nonos.Stark.Fold.the_honest_fold_reaches_a_constant
#print axioms Nonos.Stark.Fri.final_layer_accepts_iff_matches
#print axioms Nonos.Stark.Lookup.non_table_value_has_zero_multiplicity
#print axioms Nonos.Stark.Merkle.distinct_leaves_give_distinct_roots
#print axioms Nonos.Stark.NullifierSet.a_recorded_nullifier_cannot_be_respent
#print axioms Nonos.Stark.NullifierSet.spend_only_grows
#print axioms Nonos.Stark.Permutation.perm_preserves_count
#print axioms Nonos.Stark.Pool.every_reachable_pool_is_solvent
#print axioms Nonos.Stark.RootWindow.a_pushed_root_is_accepted
#print axioms Nonos.Stark.RootWindow.the_window_never_exceeds_the_cap
#print axioms Nonos.Stark.Staking.rewards_never_exceed_fees
#print axioms Nonos.Stark.RunningSum.final_is_total
#print axioms Nonos.Stark.RunningSum.conservation
#print axioms Nonos.Stark.RunningSum.follows_pins_the_trace
#print axioms Nonos.Stark.Polynomial.eval_mul
#print axioms Nonos.Stark.Polynomial.zerofier_nonzero_off_the_points
#print axioms Nonos.Stark.Polynomial.factor
#print axioms Nonos.Stark.Polynomial.roots_divide
#print axioms Nonos.Stark.Polynomial.agreement_divides_by_zerofier
#print axioms Nonos.Stark.Polynomial.root_bound
#print axioms Nonos.Stark.Transcript.order_changes_the_state
#print axioms Nonos.Syscall.decode_agrees_with_the_registry
#print axioms Nonos.UsbHid.bindings_never_exceed_the_cap

-- Assurance capstone: the guarantees composed, post-quantum hybrid authority,
-- and the admission theorem that holds over the exact gated function the loader
-- runs (attested AND rollback-fresh AND post-quantum-authorized).
#print axioms Nonos.Assurance.authority
#print axioms Nonos.Assurance.freshness
#print axioms Nonos.Assurance.integrity
#print axioms Nonos.Assurance.classical_break_insufficient
#print axioms Nonos.Assurance.no_pq_no_authority
#print axioms Nonos.Assurance.only_ok_capsules_run
#print axioms Nonos.Assurance.run_capsule_pq_authorized
#print axioms Nonos.Assurance.unattested_never_runs
#print axioms Nonos.Assurance.stale_never_runs
#print axioms Nonos.Assurance.unsigned_pq_never_runs

-- Kernel-mechanism modules added alongside the capstone. One flagship theorem
-- from each is profiled so the CI evidence records its axiom closure and the
-- sorryAx gate covers it.
#print axioms Nonos.MemGrant.run_conserves
#print axioms Nonos.MemGrant.run_granted_le_capacity
#print axioms Nonos.Scheduler.rotate_mem
#print axioms Nonos.PageTable.mapAllChecked_safe
#print axioms Nonos.Iommu.empty_grant_denies
#print axioms Nonos.Dispatch.serviced_requires_cap
#print axioms Nonos.Dispatch.denied_below
#print axioms Nonos.Quota.acquireAll_used_le_cap
#print axioms Nonos.Interval.disjoint_not_mem
#print axioms Nonos.Interval.mem_of_subset
#print axioms Nonos.Refcount.dec_from_one_dead
#print axioms Nonos.Timer.tickAll_monotone
#print axioms Nonos.Endpoint.recv_was_sent
#print axioms Nonos.Heap.double_free_safe
#print axioms Nonos.Heap.free_not_allocated
#print axioms Nonos.Fd.close_not_open
#print axioms Nonos.Ring.run_count_le_cap
#print axioms Nonos.Bounds.index_in_buffer
#print axioms Nonos.Nonce.issue2_distinct
#print axioms Nonos.Priority.preempts_total
#print axioms Nonos.Zeroize.wiped_is_zero
#print axioms Nonos.Mmio.empty_grant_denies
#print axioms Nonos.CapTable.revoke_not_holds
#print axioms Nonos.CapTable.grant_then_revoke
#print axioms Nonos.Vfs.resolve_dotdots_root
#print axioms Nonos.Rng.drawN_advances
#print axioms Nonos.Tlb.invalidate_evicts
