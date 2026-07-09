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
#print axioms Nonos.Stark.Attest.a_proof_for_one_capsule_is_rejected_for_another
#print axioms Nonos.Stark.CopyConstraint.wiring_forces_equality
#print axioms Nonos.Stark.Field.mul_distributes_over_add
#print axioms Nonos.Stark.Fri.final_layer_accepts_iff_matches
#print axioms Nonos.Stark.Lookup.non_table_value_has_zero_multiplicity
#print axioms Nonos.Stark.Merkle.distinct_leaves_give_distinct_roots
#print axioms Nonos.Stark.Permutation.perm_preserves_count
#print axioms Nonos.Stark.Polynomial.eval_mul
#print axioms Nonos.Stark.Polynomial.zerofier_nonzero_off_the_points
#print axioms Nonos.Stark.Polynomial.factor
#print axioms Nonos.Stark.Polynomial.roots_divide
#print axioms Nonos.Stark.Polynomial.agreement_divides_by_zerofier
#print axioms Nonos.Stark.Transcript.order_changes_the_state
#print axioms Nonos.Syscall.decode_agrees_with_the_registry
#print axioms Nonos.UsbHid.bindings_never_exceed_the_cap
