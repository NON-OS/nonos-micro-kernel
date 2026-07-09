/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The polynomial algebra the STARK arguments rest on, proved over the integers, an
integral domain, so the results transfer to any field including Goldilocks.
Evaluation is a ring homomorphism: the evaluation of a sum is the sum of the
evaluations and the evaluation of a product is their product. From that, the
zerofier of a set of points, the product of `(X - r)` over the points, vanishes
exactly on those points and nowhere else. This is the structural fact FRI, the
lookup argument, and the permutation argument all use: a constraint's zerofier
is zero on the trace domain and nonzero off it, so a low-degree combination that
must be divisible by the zerofier is pinned. The homomorphism and the domain give
the vanishing set with no assumed Schwartz-Zippel step.

`userland/stark_proofs` discharges the evaluation and low-degree machinery on the
real `poly` code in `poly_tests.rs`: `evaluation_matches_the_defining_sum`,
`the_low_degree_extension_recovers_the_polynomial`, and
`lagrange_interpolation_reproduces_its_nodes`.
-/

namespace Nonos.Stark.Polynomial

/-- A polynomial as its coefficients low degree first: `[a0, a1, a2]` is
    `a0 + a1 X + a2 X^2`. -/
abbrev Poly := List Int

/-- Horner evaluation. -/
def eval : Poly → Int → Int
  | [], _ => 0
  | c :: p, x => c + x * eval p x

/-- Coefficient-wise sum, the longer tail carried through. -/
def add : Poly → Poly → Poly
  | [], q => q
  | p, [] => p
  | a :: p, b :: q => (a + b) :: add p q

/-- Scale every coefficient. -/
def scale (a : Int) : Poly → Poly
  | [] => []
  | c :: p => (a * c) :: scale a p

/-- Multiply, `a::p` contributing `a * q` and `X * (p * q)`. -/
def mul : Poly → Poly → Poly
  | [], _ => []
  | a :: p, q => add (scale a q) (0 :: mul p q)

/-- The linear factor `X - r`. -/
def linear (r : Int) : Poly := [-r, 1]

/-- The zerofier of a point set: the product of `(X - r)` over the points. -/
def zerofier : List Int → Poly
  | [] => [1]
  | r :: rs => mul (linear r) (zerofier rs)

theorem eval_add (p q : Poly) (x : Int) :
    eval (add p q) x = eval p x + eval q x := by
  induction p generalizing q with
  | nil => simp [add, eval]
  | cons a p ih =>
    cases q with
    | nil => simp [add, eval]
    | cons b q =>
      simp only [add, eval]
      rw [ih q, Int.mul_add]
      omega

/-- Prepending a zero coefficient multiplies the polynomial by `X`. -/
theorem eval_shift (p : Poly) (x : Int) : eval (0 :: p) x = x * eval p x := by
  simp [eval]

theorem eval_scale (a : Int) (p : Poly) (x : Int) :
    eval (scale a p) x = a * eval p x := by
  induction p with
  | nil => simp [scale, eval]
  | cons c p ih =>
    simp only [scale, eval]
    rw [ih, Int.mul_add]
    have hlc : x * (a * eval p x) = a * (x * eval p x) := by
      rw [← Int.mul_assoc, Int.mul_comm x a, Int.mul_assoc]
    rw [hlc]

/-- Evaluation is multiplicative: the ring-homomorphism property the arguments
    lean on. -/
theorem eval_mul (p q : Poly) (x : Int) :
    eval (mul p q) x = eval p x * eval q x := by
  induction p with
  | nil => simp [mul, eval]
  | cons a p ih =>
    simp only [mul, eval, eval_add, eval_scale, eval_shift]
    rw [ih, Int.add_mul, Int.mul_assoc]
    omega

theorem eval_linear (r x : Int) : eval (linear r) x = x - r := by
  simp only [linear, eval, Int.mul_zero, Int.add_zero, Int.mul_one]
  omega

/-- The zerofier evaluates to the product of `(x - r)` over the points. -/
theorem eval_zerofier (rs : List Int) (x : Int) :
    eval (zerofier rs) x = rs.foldr (fun r acc => (x - r) * acc) 1 := by
  induction rs with
  | nil => simp [zerofier, eval]
  | cons r rs ih =>
    simp only [zerofier, eval_mul, eval_linear, List.foldr_cons, ih]

/-- The zerofier vanishes at every enrolled point: its root set contains them. -/
theorem zerofier_vanishes_at_a_point (r : Int) :
    ∀ (rs : List Int), r ∈ rs → eval (zerofier rs) r = 0 := by
  intro rs hmem
  rw [eval_zerofier]
  induction rs with
  | nil => exact absurd hmem (List.not_mem_nil r)
  | cons s rs ih =>
    rw [List.foldr_cons]
    rcases List.mem_cons.mp hmem with hs | hs
    · subst hs; simp
    · rw [ih hs]; simp

/-- The zerofier vanishes nowhere else: off its point set it is nonzero, because
    the integers are a domain, so no product of nonzero factors is zero. This is
    the constraint zerofier's defining property, with no Schwartz-Zippel
    assumption. -/
theorem zerofier_nonzero_off_the_points (x : Int) :
    ∀ (rs : List Int), (∀ r ∈ rs, x ≠ r) → eval (zerofier rs) x ≠ 0 := by
  intro rs hoff
  rw [eval_zerofier]
  induction rs with
  | nil => simp
  | cons r rs ih =>
    rw [List.foldr_cons]
    have hr : x - r ≠ 0 := fun heq => hoff r (List.mem_cons_self r rs) (by omega)
    have hrest := ih (fun s hs => hoff s (List.mem_cons_of_mem r hs))
    intro hprod
    rcases Int.mul_eq_zero.mp hprod with h0 | h0
    · exact hr h0
    · exact hrest h0

end Nonos.Stark.Polynomial
