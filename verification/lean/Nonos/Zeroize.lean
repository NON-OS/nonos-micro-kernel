/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Zeroization. When a secret buffer is wiped, every byte in the wiped region
reads zero. The theorems below show zeroization clears exactly its range,
leaves the rest untouched, is idempotent, and that a wider wipe subsumes a
narrower one, so a freed key page carries no residual secret.
-/

namespace Nonos.Zeroize

/-- A buffer as a byte-valued function over indices. -/
def Buf := Nat → Nat

/-- Wipe the first `n` bytes to zero. -/
def wipe (b : Buf) (n : Nat) : Buf := fun i => if i < n then 0 else b i

/-- Every byte inside the wiped region reads zero: no residual secret. -/
theorem wiped_is_zero (b : Buf) (n i : Nat) (h : i < n) : wipe b n i = 0 := by
  simp only [wipe]; rw [if_pos h]

/-- Bytes past the wiped region are untouched. -/
theorem past_wipe_untouched (b : Buf) (n i : Nat) (h : n ≤ i) : wipe b n i = b i := by
  simp only [wipe]; rw [if_neg (show ¬ i < n by omega)]

/-- Wiping is idempotent. -/
theorem wipe_idempotent (b : Buf) (n i : Nat) : wipe (wipe b n) n i = wipe b n i := by
  simp only [wipe]
  by_cases hi : i < n <;> simp [hi]

/-- A wider wipe clears everything a narrower wipe would. -/
theorem wider_wipe_clears (b : Buf) (m n i : Nat) (hmn : m ≤ n) (hi : i < m) :
    wipe b n i = 0 := by
  simp only [wipe]; rw [if_pos (show i < n by omega)]

/-- Wiping zero bytes changes nothing. -/
theorem wipe_zero (b : Buf) (i : Nat) : wipe b 0 i = b i := by
  simp only [wipe]; rw [if_neg (by omega)]

end Nonos.Zeroize
