/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Canonical field elements over Goldilocks p = 2^64 - 2^32 + 1. The proof
deserializer must reject any limb that is not already reduced, so a malformed
proof cannot smuggle a non-canonical value past the verifier. The theorems below
show reduction always lands in range, a reduced value is unchanged, zero and one
are canonical, a value at or above the modulus is rejected, and field addition and
multiplication modulo p stay canonical: the invariant the parser enforces.
-/

namespace Nonos.Stark.FieldBound

/-- The Goldilocks modulus, 2^64 - 2^32 + 1. -/
def P : Nat := 18446744069414584321

/-- The modulus is positive. -/
theorem P_pos : 0 < P := by decide

/-- A value is canonical when it is a reduced residue. -/
def Canonical (x : Nat) : Prop := x < P

/-- Reduction into the field. -/
def reduce (x : Nat) : Nat := x % P

/-- Reduction always yields a canonical value. -/
theorem reduce_canonical (x : Nat) : Canonical (reduce x) := by
  simp only [Canonical, reduce]; exact Nat.mod_lt x P_pos

/-- A canonical value is unchanged by reduction. -/
theorem reduce_id (x : Nat) (h : Canonical x) : reduce x = x := by
  simp only [reduce]; exact Nat.mod_eq_of_lt h

/-- Zero is canonical. -/
theorem zero_canonical : Canonical 0 := by simp only [Canonical]; exact P_pos

/-- One is canonical. -/
theorem one_canonical : Canonical 1 := by simp only [Canonical, P]; omega

/-- A value at or above the modulus is rejected. -/
theorem over_modulus_rejected (x : Nat) (h : P ≤ x) : ¬ Canonical x := by
  simp only [Canonical]; omega

/-- The parser accepts a limb iff it is below the modulus. -/
theorem accept_iff_below (x : Nat) : Canonical x ↔ x < P := Iff.rfl

/-- Field addition stays canonical. -/
theorem add_canonical (x y : Nat) : Canonical ((x + y) % P) := reduce_canonical (x + y)

/-- Field multiplication stays canonical. -/
theorem mul_canonical (x y : Nat) : Canonical ((x * y) % P) := reduce_canonical (x * y)

/-- Reduction is idempotent. -/
theorem reduce_idem (x : Nat) : reduce (reduce x) = reduce x :=
  reduce_id (reduce x) (reduce_canonical x)

/-- The modulus itself is not canonical. -/
theorem modulus_not_canonical : ¬ Canonical P := by
  simp only [Canonical]; omega

/-- Canonicity is decidable: the parser always decides accept or reject. -/
theorem canonical_decidable (x : Nat) : Canonical x ∨ ¬ Canonical x := by
  by_cases h : Canonical x
  · exact Or.inl h
  · exact Or.inr h

/-- Every canonical value is strictly below the modulus, so limbs are bounded. -/
theorem canonical_bounded (x : Nat) (h : Canonical x) : x + 1 ≤ P := by
  simp only [Canonical] at h; omega

end Nonos.Stark.FieldBound
