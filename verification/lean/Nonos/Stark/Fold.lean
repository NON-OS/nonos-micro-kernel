/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The algebraic heart of the FRI fold. Split a polynomial into its even and odd
coefficients; then `f(x) = f_even(x^2) + x * f_odd(x^2)`. This is why folding
under a challenge, `f_even + beta * f_odd`, produces a polynomial in `x^2` of
half the degree: the split has at most half the coefficients each. Iterated, the
degree halves every round until a constant, which is the low-degree conclusion
the final-layer check enforces (Nonos.Stark.Fri). This module proves the split
identity and the coefficient-count halving over the integers.

`userland/stark_proofs` discharges the fold on the real FRI code in `fri_tests.rs`:
`an_honest_low_degree_codeword_verifies`, `honest_proofs_verify_across_sizes`, and
`a_high_degree_codeword_is_rejected`.
-/

import Nonos.Stark.Polynomial

namespace Nonos.Stark.Fold

open Nonos.Stark.Polynomial

/-- The even-index coefficients: `a0, a2, a4, ...`. -/
def evens : Poly → Poly
  | [] => []
  | [a] => [a]
  | a :: _ :: p => a :: evens p

/-- The odd-index coefficients: `a1, a3, a5, ...`. -/
def odds : Poly → Poly
  | [] => []
  | [_] => []
  | _ :: b :: p => b :: odds p

/-- The FRI split identity: a polynomial is its even part at `x^2` plus `x` times
    its odd part at `x^2`. The two evaluations at `x^2` are what the fold then
    combines under the challenge. -/
theorem eval_split (p : Poly) (x : Int) :
    eval p x = eval (evens p) (x * x) + x * eval (odds p) (x * x) := by
  induction p using evens.induct with
  | case1 => simp [evens, odds, eval]
  | case2 a => simp [evens, odds, eval]
  | case3 a b p ih =>
    simp only [evens, odds, eval]
    rw [ih]
    simp only [Int.mul_add, Int.mul_assoc]
    omega

/-- The even part has at most half the coefficients, rounded up. -/
theorem evens_length (p : Poly) : (evens p).length ≤ (p.length + 1) / 2 := by
  induction p using evens.induct with
  | case1 => simp [evens]
  | case2 a => simp [evens]
  | case3 a b p ih => simp only [evens, List.length_cons]; omega

/-- The odd part has at most half the coefficients, rounded down: the fold
    strictly shrinks a polynomial of degree above zero. -/
theorem odds_length (p : Poly) : (odds p).length ≤ p.length / 2 := by
  induction p using evens.induct with
  | case1 => simp [odds]
  | case2 a => simp [odds]
  | case3 a b p ih => simp only [odds, List.length_cons]; omega

end Nonos.Stark.Fold
