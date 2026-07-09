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

/-- The ring identity the factor step needs, with the coefficients abstracted so
    it is pure integer algebra. Proven by hand since core Lean has no `ring`. -/
private theorem factor_identity (c a b x r : Int) :
    c + x * (a + (x - r) * b) = c + r * a + (x - r) * (a + x * b) := by
  have e1 : x * (a + (x - r) * b) = x * a + x * ((x - r) * b) := Int.mul_add x a ((x - r) * b)
  have e2 : (x - r) * (a + x * b) = (x - r) * a + (x - r) * (x * b) := Int.mul_add (x - r) a (x * b)
  have e3 : (x - r) * a = x * a - r * a := Int.sub_mul x r a
  have e4 : x * ((x - r) * b) = (x - r) * (x * b) := by
    rw [← Int.mul_assoc, Int.mul_comm x (x - r), Int.mul_assoc]
  rw [e1, e2, e3, e4]
  omega

/-- The factor theorem: any polynomial splits as its value at `r` plus `(X - r)`
    times a quotient. The quotient is `eval tail r :: quotient tail`, built by the
    same recursion synthetic division uses. This is the algebraic heart of
    root counting. -/
theorem factor (p : Poly) (r : Int) :
    ∃ q, ∀ x, eval p x = eval p r + (x - r) * eval q x := by
  induction p with
  | nil => exact ⟨[], fun x => by simp [eval]⟩
  | cons c p ih =>
    obtain ⟨q, hq⟩ := ih
    refine ⟨eval p r :: q, fun x => ?_⟩
    simp only [eval]
    rw [hq x]
    exact factor_identity c (eval p r) (eval q x) x r

/-- A root lets you divide it out: if `p` vanishes at `r`, then `p = (X - r) q`
    for a quotient `q`. This is the step that turns each root into a linear
    factor, the basis of the bound that a polynomial has no more roots than its
    degree. -/
theorem root_divides (p : Poly) (r : Int) (h : eval p r = 0) :
    ∃ q, ∀ x, eval p x = (x - r) * eval q x := by
  obtain ⟨q, hq⟩ := factor p r
  exact ⟨q, fun x => by rw [hq x, h]; omega⟩

/-- The root structure theorem: if a polynomial vanishes at every point of a list
    of distinct points, the zerofier of those points divides it. Proved by
    dividing out one linear factor per root; a value present in the roots but not
    yet divided cannot be a root of the linear factor, so the domain forces the
    quotient to vanish there too. This is the degree-free form of the bound that
    a polynomial has no more roots than its degree: more distinct roots than the
    degree would demand a zerofier of higher degree than the polynomial, so the
    polynomial is zero. -/
theorem roots_divide :
    ∀ (rs : List Int), rs.Nodup → ∀ (p : Poly), (∀ r ∈ rs, eval p r = 0) →
      ∃ s, ∀ x, eval p x = eval (zerofier rs) x * eval s x := by
  intro rs
  induction rs with
  | nil =>
    intro _ p _
    exact ⟨p, fun x => by simp [zerofier, eval]⟩
  | cons r rs ih =>
    intro hnd p hvan
    obtain ⟨hrnotin, hrsnd⟩ := List.nodup_cons.mp hnd
    obtain ⟨q, hq⟩ := root_divides p r (hvan r (List.mem_cons_self r rs))
    have hqvan : ∀ r' ∈ rs, eval q r' = 0 := by
      intro r' hr'
      have hz : (r' - r) * eval q r' = 0 := by
        rw [← hq r']; exact hvan r' (List.mem_cons_of_mem r hr')
      rcases Int.mul_eq_zero.mp hz with h0 | h0
      · have hre : r' = r := by omega
        rw [hre] at hr'
        exact absurd hr' hrnotin
      · exact h0
    obtain ⟨s, hs⟩ := ih hrsnd q hqvan
    refine ⟨s, fun x => ?_⟩
    rw [hq x, hs x]
    simp only [zerofier, eval_mul, eval_linear]
    rw [Int.mul_assoc]

/-- A concrete quotient by `(X - r)`, the coefficients synthetic division
    produces. Independent of the head coefficient, which the linear factor
    absorbs into the remainder. -/
def quo (r : Int) : Poly → Poly
  | [] => []
  | _ :: p => eval p r :: quo r p

theorem eval_quo (p : Poly) (r x : Int) :
    eval p x = eval p r + (x - r) * eval (quo r p) x := by
  induction p with
  | nil => simp [quo, eval]
  | cons c p ih =>
    simp only [quo, eval]
    rw [ih]
    exact factor_identity c (eval p r) (eval (quo r p) x) x r

/-- Appending a zero coefficient does not change the value. -/
theorem eval_snoc_zero (p : Poly) (x : Int) : eval (p ++ [0]) x = eval p x := by
  induction p with
  | nil => simp [eval]
  | cons c p ih =>
    have hc : (c :: p) ++ [0] = c :: (p ++ [0]) := List.cons_append c p [0]
    rw [hc]
    simp only [eval]
    rw [ih]

/-- The concrete quotient always ends in a zero, so it can be trimmed to a
    strictly shorter polynomial with the same value: `(X - r)` lowers the degree
    by exactly one. This is what makes the root count strict. -/
theorem quo_snoc_zero (p : Poly) (r : Int) (hp : p ≠ []) :
    ∃ q0, quo r p = q0 ++ [0] ∧ q0.length + 1 = p.length := by
  induction p with
  | nil => exact absurd rfl hp
  | cons c p ih =>
    by_cases hpe : p = []
    · subst hpe
      exact ⟨[], by simp [quo, eval], by simp⟩
    · obtain ⟨q0, hq0, hlen⟩ := ih hpe
      refine ⟨eval p r :: q0, ?_, ?_⟩
      · simp [quo, hq0]
      · simp only [List.length_cons] at hlen ⊢; omega

/-- A root divides out with a strictly shorter quotient. -/
theorem root_divides_shorter (p : Poly) (r : Int) (hp : p ≠ []) (h : eval p r = 0) :
    ∃ q, (∀ x, eval p x = (x - r) * eval q x) ∧ q.length + 1 = p.length := by
  obtain ⟨q0, hq0, hlen⟩ := quo_snoc_zero p r hp
  refine ⟨q0, fun x => ?_, hlen⟩
  rw [eval_quo p r x, h, hq0, eval_snoc_zero]
  omega

private theorem root_bound_aux :
    ∀ (n : Nat) (p : Poly), p.length ≤ n → ∀ (rs : List Int), rs.Nodup →
      (∀ r ∈ rs, eval p r = 0) → p.length ≤ rs.length → ∀ x, eval p x = 0 := by
  intro n
  induction n with
  | zero =>
    intro p hpl _ _ _ _ x
    cases p with
    | nil => simp [eval]
    | cons c p => simp only [List.length_cons] at hpl; omega
  | succ n ih =>
    intro p hpl rs hnd hvan hlen x
    by_cases hpe : p = []
    · subst hpe; simp [eval]
    · cases rs with
      | nil =>
        exfalso
        rw [List.length_nil] at hlen
        cases p with
        | nil => exact hpe rfl
        | cons _ _ => simp only [List.length_cons] at hlen; omega
      | cons r0 rs' =>
        obtain ⟨hr0, hrs'⟩ := List.nodup_cons.mp hnd
        obtain ⟨q, hq, hqlen⟩ :=
          root_divides_shorter p r0 hpe (hvan r0 (List.mem_cons_self r0 rs'))
        have hqvan : ∀ r' ∈ rs', eval q r' = 0 := by
          intro r' hr'
          have hz : (r' - r0) * eval q r' = 0 := by
            rw [← hq r']; exact hvan r' (List.mem_cons_of_mem r0 hr')
          rcases Int.mul_eq_zero.mp hz with h0 | h0
          · have : r' = r0 := by omega
            rw [this] at hr'; exact absurd hr' hr0
          · exact h0
        have hqn : q.length ≤ n := by omega
        have hqrs : q.length ≤ rs'.length := by
          simp only [List.length_cons] at hlen; omega
        rw [hq x, ih q hqn rs' hrs' hqvan hqrs x, Int.mul_zero]

/-- The numeric root bound: a polynomial that vanishes at as many distinct points
    as its length is functionally zero. Equivalently, a polynomial that is not
    identically zero has strictly fewer roots than its length, so no more than
    its degree. This is the Schwartz-Zippel core, proved outright: divide out one
    linear factor per root, each strictly lowering the length, until the length
    runs out and forces the zero polynomial. -/
theorem root_bound (p : Poly) (rs : List Int) (hnd : rs.Nodup)
    (hvan : ∀ r ∈ rs, eval p r = 0) (hlen : p.length ≤ rs.length) :
    ∀ x, eval p x = 0 :=
  root_bound_aux p.length p (Nat.le_refl _) rs hnd hvan hlen

/-- Polynomial subtraction. -/
def sub (p q : Poly) : Poly := add p (scale (-1) q)

theorem eval_sub (p q : Poly) (x : Int) :
    eval (sub p q) x = eval p x - eval q x := by
  simp only [sub, eval_add, eval_scale]
  omega

/-- The Reed-Solomon agreement structure FRI rests on: two polynomials that
    agree at every point of a set of distinct points differ by a multiple of the
    zerofier of those points. A codeword close to a low-degree polynomial, agree
    at more positions than the degree, is that polynomial; the proximity test
    turns disagreement into a zerofier that cannot divide a lower-degree word. -/
theorem agreement_divides_by_zerofier (p q : Poly) (rs : List Int)
    (hnd : rs.Nodup) (hagree : ∀ r ∈ rs, eval p r = eval q r) :
    ∃ t, ∀ x, eval p x - eval q x = eval (zerofier rs) x * eval t x := by
  obtain ⟨t, ht⟩ := roots_divide rs hnd (sub p q) (by
    intro r hr
    rw [eval_sub]
    have := hagree r hr
    omega)
  exact ⟨t, fun x => by rw [← eval_sub]; exact ht x⟩

end Nonos.Stark.Polynomial
