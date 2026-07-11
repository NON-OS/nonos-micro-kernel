/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Allocation bitmap. Each index is a bit: set means allocated, clear means free.
The theorems below show setting a bit allocates its index, clearing frees it,
set-then-clear frees the index, and setting or clearing one index leaves every
other unchanged, so bit operations affect exactly their own index and no
neighbour.
-/

namespace Nonos.Bitmap

/-- The bitmap as a predicate over indices: true means allocated. -/
def Bits := Nat → Bool

/-- Mark an index allocated. -/
def set (b : Bits) (i : Nat) : Bits := fun x => if x = i then true else b x

/-- Mark an index free. -/
def clear (b : Bits) (i : Nat) : Bits := fun x => if x = i then false else b x

/-- An index is allocated. -/
def allocated (b : Bits) (i : Nat) : Prop := b i = true

/-- An index is free. -/
def free (b : Bits) (i : Nat) : Prop := b i = false

/-- Setting a bit allocates its index. -/
theorem set_allocates (b : Bits) (i : Nat) : allocated (set b i) i := by
  simp only [allocated, set]; simp

/-- Clearing a bit frees its index. -/
theorem clear_frees (b : Bits) (i : Nat) : free (clear b i) i := by
  simp only [free, clear]; simp

/-- Setting then clearing an index leaves it free. -/
theorem set_then_clear_frees (b : Bits) (i : Nat) : free (clear (set b i) i) i :=
  clear_frees (set b i) i

/-- Setting one index leaves every other unchanged. -/
theorem set_other (b : Bits) (i j : Nat) (h : j ≠ i) : (set b i) j = b j := by
  simp only [set]; simp [h]

/-- Clearing one index leaves every other unchanged. -/
theorem clear_other (b : Bits) (i j : Nat) (h : j ≠ i) : (clear b i) j = b j := by
  simp only [clear]; simp [h]

/-- Re-setting an already allocated index keeps it allocated. -/
theorem double_set_still_allocated (b : Bits) (i : Nat) : allocated (set (set b i) i) i := by
  simp only [allocated, set]; simp

end Nonos.Bitmap
