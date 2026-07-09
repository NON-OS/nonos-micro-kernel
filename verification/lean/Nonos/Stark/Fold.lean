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

/-- The split at the negated point: the `x` term flips sign, since `(-x)^2 = x^2`.
    Together with `eval_split`, `f(x)` and `f(-x)` are a two-point view of the
    even and odd parts. -/
theorem eval_split_neg (p : Poly) (x : Int) :
    eval p (-x) = eval (evens p) (x * x) - x * eval (odds p) (x * x) := by
  have h := eval_split p (-x)
  rw [Int.neg_mul_neg, Int.neg_mul] at h
  omega

/-- The even part is recovered from the two points: `f(x) + f(-x) = 2 f_even(x^2)`.
    The FRI verifier reads both and reconstructs the fold. -/
theorem fold_sum (p : Poly) (x : Int) :
    eval p x + eval p (-x) = 2 * eval (evens p) (x * x) := by
  rw [eval_split p x, eval_split_neg p x]; omega

/-- The odd part is recovered likewise: `f(x) - f(-x) = 2 x f_odd(x^2)`. -/
theorem fold_diff (p : Poly) (x : Int) :
    eval p x - eval p (-x) = 2 * (x * eval (odds p) (x * x)) := by
  rw [eval_split p x, eval_split_neg p x]; omega

/-- The folded polynomial under a challenge: `f_even + beta * f_odd`. -/
def fold (beta : Int) (p : Poly) : Poly := add (evens p) (scale beta (odds p))

theorem eval_fold (beta : Int) (p : Poly) (y : Int) :
    eval (fold beta p) y = eval (evens p) y + beta * eval (odds p) y := by
  simp only [fold, eval_add, eval_scale]

theorem scale_length (a : Int) (p : Poly) : (scale a p).length = p.length := by
  induction p with
  | nil => rfl
  | cons c p ih => simp [scale, ih]

theorem add_length (p q : Poly) : (add p q).length = max p.length q.length := by
  induction p generalizing q with
  | nil => simp [add]
  | cons a p ih =>
    cases q with
    | nil => simp [add]
    | cons b q => simp only [add, List.length_cons, ih]; omega

/-- The fold shrinks the degree: the folded polynomial has at most half the
    coefficients, so after enough rounds it is a constant, the low-degree
    conclusion the final-layer check enforces. -/
theorem fold_length (beta : Int) (p : Poly) :
    (fold beta p).length ≤ (p.length + 1) / 2 := by
  rw [fold, add_length, scale_length]
  have he := evens_length p
  have ho := odds_length p
  omega

end Nonos.Stark.Fold
