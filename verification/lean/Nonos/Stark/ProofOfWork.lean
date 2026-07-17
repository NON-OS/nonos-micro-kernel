/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Grinding proof-of-work. The prover must find a nonce whose transcript hash falls
below a threshold that shrinks by a factor of two per grinding bit, raising the
soundness by that many bits. The theorems below show the threshold is positive, no
grinding admits the full range, more grinding never enlarges the threshold, a
witness valid at a higher grinding count is valid at a lower one, and grinding to
the full width leaves only the zero hash: the exact behaviour the verifier's
threshold check relies on.
-/

namespace Nonos.Stark.ProofOfWork

/-- The grinding threshold: a valid nonce hashes below 2^(bits - grind). -/
def threshold (bits grind : Nat) : Nat := 2 ^ (bits - grind)

/-- A nonce is a valid grinding witness when its hash is below the threshold. -/
def valid (bits grind hash : Nat) : Prop := hash < threshold bits grind

/-- The threshold is always positive: some hash always satisfies it. -/
theorem threshold_pos (bits grind : Nat) : 0 < threshold bits grind := by
  simp only [threshold]; exact Nat.pos_pow_of_pos _ (by decide)

/-- With no grinding, every hash below the full width is valid. -/
theorem no_grind_full_range (bits hash : Nat) (h : hash < 2 ^ bits) : valid bits 0 hash := by
  simp only [valid, threshold, Nat.sub_zero]; exact h

/-- More grinding never enlarges the threshold: grinding only makes proofs harder. -/
theorem more_grind_smaller_threshold (bits g g' : Nat) (h : g ≤ g') :
    threshold bits g' ≤ threshold bits g := by
  simp only [threshold]
  apply Nat.pow_le_pow_right (by decide)
  omega

/-- A witness valid at a higher grinding count is valid at a lower one. -/
theorem valid_downward_grind (bits g g' hash : Nat) (h : g ≤ g') (hv : valid bits g' hash) :
    valid bits g hash := by
  simp only [valid] at *
  exact Nat.lt_of_lt_of_le hv (more_grind_smaller_threshold bits g g' h)

/-- Grinding to the full bit width leaves only the zero hash valid. -/
theorem full_grind_only_zero (bits hash : Nat) (hv : valid bits bits hash) : hash = 0 := by
  simp only [valid, threshold, Nat.sub_self, Nat.pow_zero] at hv
  omega

/-- The zero hash is always a valid witness, whatever the grinding count. -/
theorem zero_always_valid (bits grind : Nat) : valid bits grind 0 := threshold_pos bits grind

/-- A hash at or above the threshold is rejected. -/
theorem over_threshold_rejected (bits grind hash : Nat) (h : threshold bits grind ≤ hash) :
    ¬ valid bits grind hash := by
  simp only [valid]; omega

/-- Grinding by one bit at least halves the admissible range. -/
theorem one_bit_halves (bits grind : Nat) (h : grind < bits) :
    2 * threshold bits (grind + 1) = threshold bits grind := by
  simp only [threshold]
  rw [← Nat.pow_succ']
  congr 1
  omega

end Nonos.Stark.ProofOfWork
