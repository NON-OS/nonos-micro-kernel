/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The spent-nullifier set, the pool's double-spend guard. A shielded withdrawal
reveals a nullifier derived from the spent note and the contract records it before
releasing funds; a second withdrawal that reveals the same nullifier is rejected.
This module proves that guard sound as a state machine over the set of spent
nullifiers: a fresh nullifier is accepted and recorded, an already-spent one is
rejected, the set only grows, and once a nullifier is recorded it can never be
spent again. Together these say no note is ever spent twice, which is the
soundness of the double-spend prevention the pool's balance safety rests on.
Structural over the integers, no polynomial machinery.
-/

namespace Nonos.Stark.NullifierSet

/-- Spend a nullifier against the spent set: reject (`none`) if it is already
    there, otherwise record it. This is the withdraw guard, marking before any
    funds move. -/
def spend (s : List Int) (nf : Int) : Option (List Int) :=
  if nf ∈ s then none else some (nf :: s)

/-- A fresh nullifier is accepted and recorded. -/
theorem a_fresh_nullifier_is_accepted (s : List Int) (nf : Int) (h : nf ∉ s) :
    spend s nf = some (nf :: s) := by
  unfold spend; rw [if_neg h]

/-- An already-spent nullifier is rejected: the direct double-spend check. -/
theorem a_spent_nullifier_is_rejected (s : List Int) (nf : Int) (h : nf ∈ s) :
    spend s nf = none := by
  unfold spend; rw [if_pos h]

/-- A successful spend records the nullifier: after it, the nullifier is in the
    set, so any later attempt to spend it will be rejected. -/
theorem spending_records_the_nullifier (s s' : List Int) (nf : Int)
    (h : spend s nf = some s') : nf ∈ s' := by
  unfold spend at h
  by_cases hm : nf ∈ s
  · rw [if_pos hm] at h; exact absurd h (by simp)
  · rw [if_neg hm] at h
    simp only [Option.some.injEq] at h
    rw [← h]; simp

/-- No note is spent twice: once a nullifier is recorded by a successful spend, a
    second spend of the same nullifier against the new set is always rejected.
    This is the core double-spend soundness. -/
theorem a_recorded_nullifier_cannot_be_respent (s s' : List Int) (nf : Int)
    (h : spend s nf = some s') : spend s' nf = none :=
  a_spent_nullifier_is_rejected s' nf (spending_records_the_nullifier s s' nf h)

/-- Spending never removes a nullifier: everything already spent stays spent. -/
theorem spend_preserves_old (s s' : List Int) (nf nf' : Int)
    (hmem : nf' ∈ s) (h : spend s nf = some s') : nf' ∈ s' := by
  unfold spend at h
  by_cases hm : nf ∈ s
  · rw [if_pos hm] at h; exact absurd h (by simp)
  · rw [if_neg hm] at h
    simp only [Option.some.injEq] at h
    rw [← h]; exact List.mem_cons_of_mem nf hmem

/-- The set only grows: a successful spend is monotone, so no earlier nullifier is
    ever freed to be reused. Combined with the no-respend fact, a spent note is
    spent permanently. -/
theorem spend_only_grows (s s' : List Int) (nf : Int)
    (h : spend s nf = some s') : ∀ x ∈ s, x ∈ s' :=
  fun x hx => spend_preserves_old s s' nf x hx h

/-- A join-split cannot reuse its own input: if a transaction presents the same
    nullifier for two inputs, the second spend against the updated set is rejected.
    A corollary of no-respend, applied within a single transaction. -/
theorem a_transaction_cannot_double_spend_one_note (s s' : List Int) (nf : Int)
    (h : spend s nf = some s') : spend s' nf = none :=
  a_recorded_nullifier_cannot_be_respent s s' nf h

/-- A worked instance: spending a fresh nullifier records it. -/
theorem honest_spend_records : spend [10, 20] 30 = some [30, 10, 20] := by decide

/-- A worked instance: spending an already-present nullifier is rejected. -/
theorem double_spend_is_rejected : spend [10, 20] 10 = none := by decide

end Nonos.Stark.NullifierSet
