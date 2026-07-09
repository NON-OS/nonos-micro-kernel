/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the permutation argument, the grand-product multiset-equality
gadget the mega-AIR uses to wire values across regions. It certifies that two
sequences are rearrangements of each other by checking a running product
`prod_i (a_i + X) = prod_i (b_i + X)`, which holds as a function of `X` exactly
when the two sequences carry the same values with the same counts. This module
proves the combinatorial core over the integers: the count of every value is a
permutation invariant, so two sequences that are rearrangements agree on every
count, and the grand product cannot separate them; conversely a value present in
one and absent from the other breaks the count and is caught. That the product
equality at a random `X` implies the polynomial identity is the Schwartz-Zippel
step, sound once `X` is drawn after the sequences commit.

`userland/stark_proofs` discharges the argument on the real `permutation.rs` code
in `air_tests.rs`: `a_permutation_argument_verifies`,
`an_honest_permutation_chain_verifies`, `a_non_permutation_is_rejected`, and
`a_wrong_permutation_output_is_rejected`.
-/

namespace Nonos.Stark.Permutation

/-- The count of a value in a sequence. -/
def count (l : List Nat) (v : Nat) : Nat :=
  (l.filter (fun w => w == v)).length

/-- Count is additive over concatenation. -/
theorem count_append (a b : List Nat) (v : Nat) :
    count (a ++ b) v = count a v + count b v := by
  unfold count
  rw [List.filter_append, List.length_append]

/-- Prepending raises one value's count by one and leaves the rest. -/
theorem count_cons (w : Nat) (rest : List Nat) (v : Nat) :
    count (w :: rest) v = (if w = v then 1 else 0) + count rest v := by
  unfold count
  by_cases h : w = v
  · subst h; simp [List.filter_cons]; omega
  · have hb : (w == v) = false := beq_eq_false_iff_ne.mpr h
    simp [List.filter_cons, hb, h]

/-- The count of every value is a permutation invariant: two sequences that are
    rearrangements of each other carry the same value the same number of times.
    This is exactly what the grand product certifies, so it cannot tell two
    permutations apart. -/
theorem perm_preserves_count {a b : List Nat} (h : a.Perm b) (v : Nat) :
    count a v = count b v := by
  induction h with
  | nil => rfl
  | cons x _ ih => rw [count_cons, count_cons, ih]
  | swap x y l => rw [count_cons, count_cons, count_cons, count_cons]; omega
  | trans _ _ ih1 ih2 => rw [ih1, ih2]

/-- A value in one sequence but not the other breaks a count, so the two are not
    a permutation: the argument rejects it. A value present in `a` occurs there
    at least once, and absent from `b` it occurs zero times, so their counts
    differ and no rearrangement relates them. -/
theorem a_missing_value_is_not_a_permutation (a b : List Nat) (v : Nat)
    (ha : v ∈ a) (hb : v ∉ b) : ¬ a.Perm b := by
  intro hp
  have hca : 1 ≤ count a v := by
    unfold count
    have : v ∈ a.filter (fun w => w == v) := by
      rw [List.mem_filter]; exact ⟨ha, by simp⟩
    exact List.length_pos_of_mem this
  have hcb : count b v = 0 := by
    unfold count
    suffices h : b.filter (fun w => w == v) = [] by simp [h]
    apply List.eq_nil_iff_forall_not_mem.mpr
    intro w hw
    rw [List.mem_filter] at hw
    have hwv : w = v := by simpa using hw.2
    exact hb (hwv ▸ hw.1)
  rw [perm_preserves_count hp] at hca
  omega

end Nonos.Stark.Permutation
