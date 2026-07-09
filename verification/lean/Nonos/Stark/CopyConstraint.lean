/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the copy constraint, the Plonk sigma-wiring gadget the mega-AIR
uses to force cells apart in the trace to carry the same value. A wiring is a
permutation sigma over cell positions; the constraint holds when every cell
equals the cell sigma sends it to. This module proves the soundness: when the
wiring holds, every two cells in the same sigma orbit carry equal values, so the
gadget binds a value across regions (a transcript challenge to where a fold
consumes it, an opening to a fold input); and a single broken wire fails the
constraint. The permutation argument that enforces the wiring at the polynomial
level is `Nonos.Stark.Permutation`; this is the equality it delivers.

`userland/stark_proofs` discharges the gadget on the real `copy_constraint.rs`
and `wired.rs` code in `air_tests.rs`: `a_value_is_bound_across_two_fused_regions`
and the wiring tests around it.
-/

namespace Nonos.Stark.CopyConstraint

variable {α : Type}

/-- Follow the wiring `sigma` from a cell `k` steps. -/
def iterate (sigma : Nat → Nat) : Nat → Nat → Nat
  | 0, i => i
  | k + 1, i => iterate sigma k (sigma i)

/-- The wiring holds: every cell equals the cell sigma links it to. -/
def Wired (cells : Nat → α) (sigma : Nat → Nat) : Prop :=
  ∀ i, cells i = cells (sigma i)

/-- The wiring forces equality along the whole orbit: a cell equals every cell
    reachable from it under sigma. This is how a copy constraint binds one value
    across arbitrarily distant positions in the trace. -/
theorem wiring_forces_equality (cells : Nat → α) (sigma : Nat → Nat)
    (h : Wired cells sigma) (i k : Nat) :
    cells i = cells (iterate sigma k i) := by
  induction k generalizing i with
  | zero => rfl
  | succ k ih =>
    rw [iterate, h i]
    exact ih (sigma i)

/-- Two directly wired cells carry the same value: the one-step case, the atom
    the whole gadget is built from. -/
theorem wired_cells_are_equal (cells : Nat → α) (sigma : Nat → Nat)
    (h : Wired cells sigma) (i : Nat) : cells i = cells (sigma i) :=
  h i

/-- A single broken wire fails the constraint: if two cells the wiring links
    disagree, the constraint does not hold, so the gadget rejects it. -/
theorem a_broken_wire_is_rejected (cells : Nat → α) (sigma : Nat → Nat)
    (i : Nat) (hne : cells i ≠ cells (sigma i)) : ¬ Wired cells sigma :=
  fun h => hne (h i)

end Nonos.Stark.CopyConstraint
