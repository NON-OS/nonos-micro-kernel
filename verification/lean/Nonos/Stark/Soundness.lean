/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The money-grade soundness budget, in bits. Each FRI query contributes a fixed
per-query strength, a blown-up rate raises that strength, and proof-of-work
grinding adds a flat number of bits on top. The theorems below account the total
exactly and show it is monotone in every knob, that dropping a query or the
grinding never strengthens the proof, and that once the accounted total meets the
128-bit target the configuration is money-grade: this is the arithmetic the
verifier constants are chosen against.
-/

namespace Nonos.Stark.Soundness

/-- Per-query soundness bits: the base folding strength plus the blown-up rate. -/
def perQueryBits (baseBits extraBlowupBits : Nat) : Nat := baseBits + extraBlowupBits

/-- Soundness from the queries alone. -/
def queryBits (baseBits extraBlowupBits nQueries : Nat) : Nat :=
  perQueryBits baseBits extraBlowupBits * nQueries

/-- Total accounted soundness: query bits plus grinding bits. -/
def securityBits (baseBits extraBlowupBits nQueries grindBits : Nat) : Nat :=
  queryBits baseBits extraBlowupBits nQueries + grindBits

/-- The money-grade target. -/
def target : Nat := 128

/-- A configuration is money-grade when its accounted soundness meets the target. -/
def moneyGrade (baseBits extraBlowupBits nQueries grindBits : Nat) : Prop :=
  target ≤ securityBits baseBits extraBlowupBits nQueries grindBits

/-- Grinding contributes exactly its bits. -/
theorem grinding_adds_exactly (b e nq g : Nat) :
    securityBits b e nq g = queryBits b e nq + g := rfl

/-- With no grinding, the total is the query bits. -/
theorem no_grind_is_query_bits (b e nq : Nat) : securityBits b e nq 0 = queryBits b e nq := by
  simp [securityBits]

/-- A blown-up rate never weakens a query. -/
theorem blowup_never_weaker (baseBits e e' : Nat) (h : e ≤ e') :
    perQueryBits baseBits e ≤ perQueryBits baseBits e' := by
  simp only [perQueryBits]; omega

/-- Total soundness is monotone in the grinding bits. -/
theorem monotone_grind (b e nq g g' : Nat) (h : g ≤ g') :
    securityBits b e nq g ≤ securityBits b e nq g' := by
  simp only [securityBits]; omega

/-- Total soundness is monotone in the number of queries. -/
theorem monotone_queries (b e nq nq' g : Nat) (h : nq ≤ nq') :
    securityBits b e nq g ≤ securityBits b e nq' g := by
  simp only [securityBits, queryBits]
  have := Nat.mul_le_mul_left (perQueryBits b e) h
  omega

/-- Total soundness is monotone in the blown-up rate. -/
theorem monotone_blowup (b e e' nq g : Nat) (h : e ≤ e') :
    securityBits b e nq g ≤ securityBits b e' nq g := by
  simp only [securityBits, queryBits]
  have : perQueryBits b e ≤ perQueryBits b e' := blowup_never_weaker b e e' h
  have := Nat.mul_le_mul_right nq this
  omega

/-- Dropping a query never strengthens the proof. -/
theorem dropping_query_not_stronger (b e nq g : Nat) :
    securityBits b e nq g ≤ securityBits b e (nq + 1) g :=
  monotone_queries b e nq (nq + 1) g (Nat.le_succ nq)

/-- Adding one query adds exactly the per-query bits. -/
theorem one_more_query (b e nq g : Nat) :
    securityBits b e (nq + 1) g = securityBits b e nq g + perQueryBits b e := by
  simp only [securityBits, queryBits, Nat.mul_succ]; omega

/-- Removing the grinding never strengthens the proof. -/
theorem dropping_grind_not_stronger (b e nq g : Nat) :
    securityBits b e nq 0 ≤ securityBits b e nq g :=
  monotone_grind b e nq 0 g (Nat.zero_le g)

/-- Meeting the target is preserved when any knob is raised: more queries. -/
theorem money_grade_stable_queries (b e nq nq' g : Nat) (h : nq ≤ nq')
    (hm : moneyGrade b e nq g) : moneyGrade b e nq' g :=
  Nat.le_trans hm (monotone_queries b e nq nq' g h)

/-- Meeting the target is preserved under more grinding. -/
theorem money_grade_stable_grind (b e nq g g' : Nat) (h : g ≤ g')
    (hm : moneyGrade b e nq g) : moneyGrade b e nq g' :=
  Nat.le_trans hm (monotone_grind b e nq g g' h)

/-- A concrete money-grade witness: 4-bit base folding, blown-up by 3, 32 queries. -/
theorem concrete_money_grade : moneyGrade 4 3 32 0 := by
  unfold moneyGrade securityBits queryBits perQueryBits target; decide

/-- The Fp2 challenge doubles the effective base folding strength. -/
def fp2Bits (baseBits : Nat) : Nat := 2 * baseBits

/-- Moving to an Fp2 challenge never weakens a query. -/
theorem fp2_never_weaker (baseBits : Nat) : baseBits ≤ fp2Bits baseBits := by
  simp only [fp2Bits]; omega

/-- With grinding, the total strictly exceeds the query bits when grinding is nonzero. -/
theorem grind_strictly_adds (b e nq g : Nat) (h : 0 < g) :
    queryBits b e nq < securityBits b e nq g := by
  simp only [securityBits]; omega

end Nonos.Stark.Soundness
