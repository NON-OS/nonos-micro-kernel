/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The running-sum accumulator, the additive argument value conservation rests on. A
STARK enforces "the balance is preserved" as a column that starts at zero and, row
by row, adds one addend, with a boundary check on the final row. This is the gate a
shielded pool uses to prove sum(inputs) = sum(outputs) + fee with no inflation and
no negative balances. This module proves the two facts the gate needs: the
accumulator ends exactly at the total of its column (so the boundary check is the
sum), and the per-row transition constraints pin the whole column given its start
(so a prover cannot present a different trace that passes the same steps). From the
first fact, conservation and tamper detection are immediate. Structural over the
integers, no polynomial machinery, in the family of the permutation and lookup
arguments.
-/

namespace Nonos.Stark.RunningSum

/-- The total of a column. -/
def total : List Int → Int
  | [] => 0
  | a :: as => a + total as

/-- The boundary value the accumulator reaches: fold the addends from `start`. -/
def final (start : Int) : List Int → Int
  | [] => start
  | a :: as => final (start + a) as

/-- The accumulator column itself: one row per addend plus the initial row, each
    row the previous plus the next addend. This is the trace the transition
    constraint is imposed on. -/
def acc (start : Int) : List Int → List Int
  | [] => [start]
  | a :: as => start :: acc (start + a) as

theorem acc_length (start : Int) (as : List Int) :
    (acc start as).length = as.length + 1 := by
  induction as generalizing start with
  | nil => rfl
  | cons a as ih => simp [acc, List.length_cons, ih]

/-- The accumulator ends at the running total: the final row equals `start` plus
    the sum of the column. This is the soundness of the boundary check, it reads
    off the whole sum. -/
theorem final_is_total (start : Int) (as : List Int) :
    final start as = start + total as := by
  induction as generalizing start with
  | nil => simp [final, total]
  | cons a as ih => simp only [final, total, ih]; omega

/-- Value conservation: if the inputs total equals the outputs total plus the fee,
    the two accumulators, run from zero, meet at the same boundary offset by the
    fee. This is the gate a shielded transaction passes, no inflation. -/
theorem conservation (inputs outputs : List Int) (fee : Int)
    (h : total inputs = total outputs + fee) :
    final 0 inputs = final 0 outputs + fee := by
  rw [final_is_total, final_is_total]; omega

/-- Tamper detection: any change to the column that moves its total is caught at
    the boundary. A prover cannot alter a value and still land on the committed
    final row. -/
theorem a_changed_total_is_caught (inputs outputs : List Int)
    (h : total inputs ≠ total outputs) :
    final 0 inputs ≠ final 0 outputs := by
  rw [final_is_total, final_is_total]; omega

/-- The column follows the addends from `start`: it begins at `start` and each
    remaining row is the previous plus the next addend. This is the conjunction of
    per-row transition constraints, written structurally. -/
def Follows (start : Int) : List Int → List Int → Prop
  | t, [] => t = [start]
  | t, a :: as => ∃ rest, t = start :: rest ∧ Follows (start + a) rest as

/-- The transition constraints pin the trace: a column that starts at `start` and
    steps by the addends is exactly the accumulator, nothing else passes the same
    steps. This is the soundness of the transition gate, the prover has no freedom
    in the column once the start and addends are fixed. -/
theorem follows_pins_the_trace (start : Int) (as t : List Int) :
    Follows start t as ↔ t = acc start as := by
  induction as generalizing start t with
  | nil => simp [Follows, acc]
  | cons a as ih =>
    simp only [Follows, acc]
    constructor
    · rintro ⟨rest, ht, hf⟩
      rw [ht, (ih (start + a) rest).mp hf]
    · intro ht
      exact ⟨acc (start + a) as, ht, (ih (start + a) (acc (start + a) as)).mpr rfl⟩

/-- A worked instance: two inputs summing to 10, two outputs summing to 8, a fee of
    2. The accumulators meet, so the transaction conserves value. -/
theorem honest_conservation_holds : final 0 [7, 3] = final 0 [6, 2] + 2 := by decide

/-- A tampered instance: the same inputs but outputs summing to 7 with the same fee
    of 2. The accumulators do not meet, so the boundary check rejects it. -/
theorem inflated_outputs_are_rejected : final 0 [7, 3] ≠ final 0 [6, 1] + 2 := by decide

end Nonos.Stark.RunningSum
