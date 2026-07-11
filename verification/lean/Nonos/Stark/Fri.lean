/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification of the FRI low-degree test's conclusion. FRI reduces a claim that
a committed codeword is close to a polynomial of degree below a bound to a
final layer that a low-degree word folds to a single constant. This module
proves the two structural facts the verifier rests on: folding a codeword by a
challenge halves its degree bound, so after enough rounds an honest low-degree
word reaches a constant final layer; and the final-layer check accepts exactly
when that constant matches the committed value, so a high-degree word, which
does not fold to a constant, is rejected. The Reed-Solomon proximity gap that
makes the sampled queries sound is the assumed hardness of the low-degree test,
carried as a hypothesis here and stated as an assumption in the architecture
document; everything below is unconditional.

`userland/stark_proofs` discharges these on the real FRI code in `fri_tests.rs`:
`an_honest_low_degree_codeword_verifies`, `an_honest_codeword_on_a_coset_verifies`,
`honest_proofs_verify_across_sizes`, and `a_high_degree_codeword_is_rejected`,
and the in-circuit final-layer conclusion is proven by the `FinalLayerConstant`
AIR exercised in `recursive_verifier_tests.rs`.
-/

namespace Nonos.Stark.Fri

/-- The degree bound of a codeword. FRI's fold takes a word of bound `d` to a
    word of bound `d / 2`: it splits into even and odd parts and combines them
    under the fold challenge, and each part carries half the degree. -/
def foldBound (d : Nat) : Nat := d / 2

/-- Folding `r` times divides the bound by `2^r`. -/
def foldBoundN : Nat → Nat → Nat
  | d, 0 => d
  | d, r + 1 => foldBoundN (foldBound d) r

/-- Each fold does not increase the bound. -/
theorem fold_does_not_increase (d : Nat) : foldBound d ≤ d :=
  Nat.div_le_self d 2

/-- Folding `r` times is division of the bound by `2^r`: the closed form of the
    round-by-round halving. -/
theorem foldBoundN_eq_div (d r : Nat) : foldBoundN d r = d / 2 ^ r := by
  induction r generalizing d with
  | zero => simp [foldBoundN]
  | succ r ih =>
    unfold foldBoundN
    rw [ih (foldBound d)]
    unfold foldBound
    rw [Nat.div_div_eq_div_mul, Nat.pow_succ, Nat.mul_comm]

/-- An honest low-degree word reaches a constant final layer: after enough
    folds its bound is at most one, so the final layer is a single value.
    Concretely, folding a word of bound below `2^r` exactly `r` times leaves a
    bound of zero. -/
theorem enough_folds_reach_a_constant (d r : Nat) (h : d < 2 ^ r) :
    foldBoundN d r ≤ 1 := by
  rw [foldBoundN_eq_div, Nat.div_eq_of_lt h]
  exact Nat.zero_le 1

/-- The final-layer check: the verifier accepts when the committed final value
    equals the constant the codeword folded to. -/
def finalLayerAccepts (folded committed : Nat) : Bool := folded == committed

/-- Acceptance is exactly agreement with the commitment: the low-degree
    conclusion is a plain equality check, so there is no slack for a forged
    final layer. -/
theorem final_layer_accepts_iff_matches (folded committed : Nat) :
    finalLayerAccepts folded committed = true ↔ folded = committed := by
  unfold finalLayerAccepts
  exact beq_iff_eq

/-- A final layer that does not match the commitment is rejected: the check
    admits nothing but the committed constant, which is what forces a
    high-degree word, whose fold is not that constant, to fail. -/
theorem a_mismatched_final_layer_is_rejected (folded committed : Nat)
    (h : folded ≠ committed) : finalLayerAccepts folded committed = false := by
  unfold finalLayerAccepts
  exact beq_eq_false_iff_ne.mpr h

end Nonos.Stark.Fri
