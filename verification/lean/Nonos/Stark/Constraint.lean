/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The soundness of the AIR mechanism, grounded in the proven polynomial algebra
rather than assumed. A STARK encodes "the computation is correct" as "the
constraint polynomial vanishes at every point of the trace domain", and the
prover commits to a low-degree quotient `q` with `C = Z_D * q`, where `Z_D` is
the domain's zerofier. This module proves the two directions coincide: the
quotient exists exactly when the constraints hold. The forward direction is the
root structure (a polynomial vanishing on distinct points is divisible by their
zerofier); the reverse is the zerofier vanishing on its own points. So a false
execution, whose constraint fails somewhere on the domain, has no such quotient,
and no low-degree commitment can fake one. This is what the FRI low-degree test
then enforces: it checks that the committed quotient really is low degree.

`userland/stark_proofs` discharges the composition and quotient machinery on the
real prover and verifier in `air_tests.rs`: the honest executions verify and the
tampered ones (`a_tampered_running_sum_is_rejected` and the wrong-output cases)
are rejected, which is the operational form of "no quotient without the
constraints".
-/

import Nonos.Stark.Polynomial

namespace Nonos.Stark.Constraint

open Nonos.Stark.Polynomial

/-- The AIR soundness: a constraint polynomial vanishes at every point of the
    trace domain if and only if the domain's zerofier divides it, i.e. a
    low-degree quotient exists. Left to right is the root structure; right to
    left is the zerofier vanishing on its points. The prover can produce the
    quotient exactly when the execution is correct. -/
theorem constraint_holds_iff_quotient_exists (c : Poly) (d : List Int)
    (hnd : d.Nodup) :
    (∀ r ∈ d, eval c r = 0) ↔ ∃ q, ∀ x, eval c x = eval (zerofier d) x * eval q x := by
  constructor
  · intro h
    exact roots_divide d hnd c h
  · rintro ⟨q, hq⟩ r hr
    rw [hq r, zerofier_vanishes_at_a_point r d hr, Int.zero_mul]

/-- A false execution has no quotient: if the constraint fails at even one point
    of the trace domain, no polynomial `q` makes `C = Z_D * q`. This is the
    soundness the FRI test protects, stated in the contrapositive the prover
    faces. -/
theorem a_broken_constraint_has_no_quotient (c : Poly) (d : List Int)
    (hnd : d.Nodup) (r : Int) (hr : r ∈ d) (hbroken : eval c r ≠ 0) :
    ¬ ∃ q, ∀ x, eval c x = eval (zerofier d) x * eval q x := by
  intro hq
  exact hbroken ((constraint_holds_iff_quotient_exists c d hnd).mpr hq r hr)

/-- The quotient is pinned off the domain: any two low-degree quotients for the
    same constraint agree at every point outside the trace domain, since the
    zerofier is nonzero there and cancels. So the witness the FRI test then checks
    for low degree is a determined object, the prover has no freedom to wiggle it
    once the constraint and domain are fixed. -/
theorem the_quotient_is_pinned (c : Poly) (d : List Int) (q1 q2 : Poly)
    (h1 : ∀ x, eval c x = eval (zerofier d) x * eval q1 x)
    (h2 : ∀ x, eval c x = eval (zerofier d) x * eval q2 x)
    (x : Int) (hx : ∀ r ∈ d, x ≠ r) : eval q1 x = eval q2 x := by
  have hz : eval (zerofier d) x ≠ 0 := zerofier_nonzero_off_the_points x d hx
  have heq : eval (zerofier d) x * eval q1 x = eval (zerofier d) x * eval q2 x := by
    rw [← h1 x, ← h2 x]
  have hzero : eval (zerofier d) x * (eval q1 x - eval q2 x) = 0 := by
    rw [Int.mul_sub]; omega
  rcases Int.mul_eq_zero.mp hzero with h0 | h0
  · exact absurd h0 hz
  · omega

end Nonos.Stark.Constraint
