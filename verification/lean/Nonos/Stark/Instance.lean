/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

A worked AIR instance, so the soundness theorem is not vacuously true over an
empty model. The bit gate is the most common real constraint: a trace column
that must hold a bit in every row, encoded as `C(x) = W(x) * (W(x) - 1)`, which
is zero exactly when `W(x)` is 0 or 1. It is the shape range decomposition and
selector columns are built from. Here it is instantiated with concrete traces
and evaluated with real integers: an honest bit column is accepted (a quotient
exists), and a tampered column with a non-bit row is rejected (no quotient can
exist), both discharged through the abstract Constraint soundness rather than
re-argued. The bridge is `bit_constraint_eval`, which reduces the composed
constraint to the pointwise bit test via the evaluation ring homomorphism.
-/

import Nonos.Stark.Constraint

namespace Nonos.Stark.Instance

open Nonos.Stark.Polynomial Nonos.Stark.Constraint

/-- The bit gate over a trace column `W`: `C = W * (W - 1)`. -/
def bitConstraint (w : Poly) : Poly := mul w (add w [-1])

/-- The gate is exactly the pointwise bit test: at every point it evaluates to
    `W(x) * (W(x) - 1)`, zero iff `W(x)` is a bit. This is where the evaluation
    ring homomorphism turns the polynomial composition into arithmetic. -/
theorem bit_constraint_eval (w : Poly) (x : Int) :
    eval (bitConstraint w) x = eval w x * (eval w x - 1) := by
  have h : eval (add w [-1]) x = eval w x - 1 := by
    rw [eval_add]; simp only [eval, Int.mul_zero, Int.add_zero]; omega
  rw [bitConstraint, eval_mul, h]

/-- The two-point evaluation domain `{0, 1}`. -/
def domain : List Int := [0, 1]

theorem domain_nodup : domain.Nodup := by decide

/-- An honest column carrying a bit in each row: `W(0) = 1`, `W(1) = 0`.
    Interpolated over the domain this is `W(x) = 1 - x`. -/
def honestTrace : Poly := [1, -1]

/-- Every row holds a bit, so the gate vanishes on the whole domain: a checked
    instance of AIR acceptance with real arithmetic. -/
theorem honest_satisfies : ∀ r ∈ domain, eval (bitConstraint honestTrace) r = 0 := by
  intro r hr
  simp only [domain] at hr
  rcases List.mem_cons.mp hr with rfl | hr
  · decide
  · rcases List.mem_cons.mp hr with rfl | hr
    · decide
    · simp at hr

/-- The gate is not the zero polynomial: it is nonzero off the domain, so the
    quotient it divides out is real content, not a triviality. -/
theorem the_gate_is_not_vacuous : eval (bitConstraint honestTrace) 2 ≠ 0 := by decide

/-- End to end: the honest run admits a low-degree quotient `C = Z_domain * q`,
    obtained straight from the abstract soundness, no separate argument. -/
theorem honest_run_has_a_quotient :
    ∃ q, ∀ x, eval (bitConstraint honestTrace) x = eval (zerofier domain) x * eval q x :=
  (constraint_holds_iff_quotient_exists (bitConstraint honestTrace) domain domain_nodup).mp
    honest_satisfies

/-- A tampered column whose second row holds 2, not a bit: `W(0) = 1`, `W(1) = 2`,
    interpolated `W(x) = 1 + x`. -/
def tamperedTrace : Poly := [1, 1]

/-- The gate fails at the tampered row. -/
theorem tampered_breaks_at_row_one : eval (bitConstraint tamperedTrace) 1 ≠ 0 := by decide

/-- End to end: the tampered run has no quotient. A prover that cheats on even one
    row cannot produce the low-degree witness the FRI test then checks, so the
    forgery is caught by the mechanism, not by inspection. -/
theorem tampered_run_has_no_quotient :
    ¬ ∃ q, ∀ x, eval (bitConstraint tamperedTrace) x = eval (zerofier domain) x * eval q x :=
  a_broken_constraint_has_no_quotient (bitConstraint tamperedTrace) domain domain_nodup 1
    (by decide) tampered_breaks_at_row_one

end Nonos.Stark.Instance
