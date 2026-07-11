/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the logup lookup argument. The argument certifies that every
value of a witness lies in a table by checking a single logarithmic-derivative
sum, `S = sum_i 1/(a_i + X) - sum_j m_j/(t_j + X)`, where `m_j` counts the
occurrences of table entry `t_j` among the witness. The kernel verifier uses
this to range-check the FRI query indices and for general table membership.

This module proves the multiplicity bookkeeping the argument rests on, over the
integers where it is unconditional. The soundness intuition is a counting one:
a value outside the table is counted in no multiplicity, so on the table side
its term is absent while on the witness side it is present, leaving an unmatched
term; and a value in the witness is counted at least once, so honest membership
is never undercounted. That the resulting rational sum is then nonzero at a
random `X` is the Schwartz-Zippel step, sound once `X` is drawn after the
multiplicities commit, which the transcript enforces.

`userland/stark_proofs` discharges the argument on the real `lookup.rs` code in
`air_tests.rs`: `a_lookup_of_in_table_values_verifies`,
`an_out_of_table_value_is_rejected`, `a_range_check_via_bit_limbs_verifies`,
`the_lookup_verifies_and_rejects_over_random_cases`, and, for the Fiat-Shamir
ordering, `the_lookup_is_sound_under_a_transcript_challenge`.
-/

namespace Nonos.Stark.Lookup

/-- The multiplicity of a value in the witness: how many times it occurs. -/
def multiplicity (witness : List Nat) (v : Nat) : Nat :=
  (witness.filter (fun w => w == v)).length

/-- Membership of a witness in a table: every witness value is a table entry.
    This is exactly the property the lookup argument certifies. -/
def containedIn (witness table : List Nat) : Prop :=
  ∀ v ∈ witness, v ∈ table

/-- The multiplicity recurrence: prepending a value raises its own count by one
    and leaves every other count unchanged. The whole bookkeeping is built from
    this single step. -/
theorem multiplicity_cons (w : Nat) (rest : List Nat) (v : Nat) :
    multiplicity (w :: rest) v
      = (if w = v then 1 else 0) + multiplicity rest v := by
  unfold multiplicity
  by_cases h : w = v
  · subst h; simp [List.filter_cons]; omega
  · have hb : (w == v) = false := beq_eq_false_iff_ne.mpr h
    simp [List.filter_cons, hb, h]

/-- A value absent from the table has multiplicity zero among a contained
    witness: if every witness value is in the table, nothing in the witness
    equals a non-table value, so its count is zero. This is why an out-of-table
    value leaves an unmatched term, the term the multiplicities cannot cancel. -/
theorem non_table_value_has_zero_multiplicity (witness table : List Nat)
    (hsub : containedIn witness table) (v : Nat) (hv : v ∉ table) :
    multiplicity witness v = 0 := by
  unfold multiplicity
  suffices h : witness.filter (fun w => w == v) = [] by simp [h]
  apply List.eq_nil_iff_forall_not_mem.mpr
  intro w hw
  rw [List.mem_filter] at hw
  obtain ⟨hmem, hbeq⟩ := hw
  have hwv : w = v := by simpa using hbeq
  exact hv (hwv ▸ hsub w hmem)

/-- Honest membership is never undercounted: a value that occurs in the witness
    has multiplicity at least one, so the table side must account for it. -/
theorem present_value_has_positive_multiplicity (witness : List Nat) (v : Nat)
    (hv : v ∈ witness) : 1 ≤ multiplicity witness v := by
  unfold multiplicity
  have hmem : v ∈ witness.filter (fun w => w == v) := by
    rw [List.mem_filter]
    exact ⟨hv, by simp⟩
  exact List.length_pos_of_mem hmem

/-- Conservation: the multiplicities never invent or drop witness terms. The
    count of any value in a concatenated witness is the sum of its counts in the
    parts, so the total bookkeeping is additive, which is exactly the balance
    the logup running sum tracks to zero. -/
theorem multiplicity_append (a b : List Nat) (v : Nat) :
    multiplicity (a ++ b) v = multiplicity a v + multiplicity b v := by
  unfold multiplicity
  rw [List.filter_append, List.length_append]

end Nonos.Stark.Lookup
