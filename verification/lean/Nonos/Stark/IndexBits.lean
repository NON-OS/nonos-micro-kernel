/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Index bit decomposition for the DEEP query point. The running product that places a
query on the evaluation domain reads the index one bit at a time, so each fold step
consumes the low bit and shifts the rest down. The theorems below show the low bit
is zero or one, the index reconstructs from its low bit and remainder, the parity
matches the low bit, and shifting halves the index: the arithmetic the DEEP point
derivation walks over.
-/

namespace Nonos.Stark.IndexBits

/-- The low bit of an index. -/
def lowBit (idx : Nat) : Nat := idx % 2

/-- The index with its low bit shifted off. -/
def shift (idx : Nat) : Nat := idx / 2

/-- The low bit is zero or one. -/
theorem lowBit_le_one (idx : Nat) : lowBit idx ≤ 1 := by simp only [lowBit]; omega

/-- The index reconstructs from its remainder and low bit. -/
theorem reconstruct (idx : Nat) : 2 * shift idx + lowBit idx = idx := by
  simp only [shift, lowBit]; omega

/-- An even index has low bit zero. -/
theorem even_lowBit_zero (idx : Nat) (h : idx % 2 = 0) : lowBit idx = 0 := h

/-- An odd index has low bit one. -/
theorem odd_lowBit_one (idx : Nat) (h : idx % 2 = 1) : lowBit idx = 1 := h

/-- Shifting halves the index (rounding down). -/
theorem shift_halves (idx : Nat) : shift idx = idx / 2 := rfl

/-- Shifting strictly decreases a nonzero index. -/
theorem shift_decreases (idx : Nat) (h : 0 < idx) : shift idx < idx := by
  simp only [shift]; omega

/-- Doubling then setting a bit is the inverse of decomposition. -/
theorem compose_decompose (hi b : Nat) (hb : b ≤ 1) : lowBit (2 * hi + b) = b := by
  simp only [lowBit]; omega

/-- The shifted part of a composed index recovers the high part. -/
theorem shift_compose (hi b : Nat) (hb : b ≤ 1) : shift (2 * hi + b) = hi := by
  simp only [shift]; omega

/-- Two indices with equal low bits and equal shifts are equal. -/
theorem bit_extensional (a b : Nat) (hl : lowBit a = lowBit b) (hs : shift a = shift b) : a = b := by
  have ra := reconstruct a
  have rb := reconstruct b
  simp only [hl, hs] at ra
  omega

/-- The low bit and shift together are injective, so the decomposition loses nothing. -/
theorem decomposition_injective (a b : Nat) (h : (lowBit a, shift a) = (lowBit b, shift b)) :
    a = b := by
  simp only [Prod.mk.injEq] at h
  exact bit_extensional a b h.1 h.2

end Nonos.Stark.IndexBits
