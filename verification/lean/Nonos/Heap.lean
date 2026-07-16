/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Heap allocation safety. The heap tracks which addresses are live. The theorems
below show alloc makes an address live and disturbs no other, free makes an
address dead, a freed address is not live (no use-after-free), and freeing an
already-dead address keeps it dead (no double-free hazard), the core spatial
safety facts the allocator rests on.
-/

namespace Nonos.Heap

/-- The heap as a liveness predicate: which addresses are currently allocated. -/
def Heap := Nat → Bool

/-- An address is allocated when it is live. -/
def allocated (h : Heap) (a : Nat) : Prop := h a = true

/-- Allocate an address. -/
def alloc (h : Heap) (a : Nat) : Heap := fun x => if x = a then true else h x

/-- Free an address. -/
def free (h : Heap) (a : Nat) : Heap := fun x => if x = a then false else h x

/-- An allocated address is live. -/
theorem alloc_allocated (h : Heap) (a : Nat) : allocated (alloc h a) a := by
  simp only [allocated, alloc]; simp

/-- Allocation disturbs no other address. -/
theorem alloc_preserves (h : Heap) (a b : Nat) (hb : allocated h b) :
    allocated (alloc h a) b := by
  simp only [allocated, alloc]
  by_cases hx : b = a
  · simp [hx]
  · simp only [if_neg hx]; exact hb

/-- Allocation never frees an address that was live. -/
theorem alloc_keeps_live (h : Heap) (a b : Nat) (hb : allocated h b) :
    allocated (alloc h a) b := alloc_preserves h a b hb

/-- A freed address is not live: no use-after-free. -/
theorem free_not_allocated (h : Heap) (a : Nat) : ¬ allocated (free h a) a := by
  simp only [allocated, free]; simp

/-- Freeing one address disturbs no other. -/
theorem free_preserves (h : Heap) (a b : Nat) (hb : allocated h b) (hne : b ≠ a) :
    allocated (free h a) b := by
  simp only [allocated, free]
  simp only [if_neg hne]; exact hb

/-- Freeing an already-freed address keeps it dead: no double-free hazard. -/
theorem double_free_safe (h : Heap) (a : Nat) : ¬ allocated (free (free h a) a) a := by
  simp only [allocated, free]; simp

/-- Alloc then free at a different address round-trips. -/
theorem alloc_free_other (h : Heap) (a b : Nat) (hne : b ≠ a) :
    (free (alloc h a) a) b = h b := by
  simp only [free, alloc]
  simp only [if_neg hne]

/-- After allocating then freeing the same address, it is dead again. -/
theorem alloc_free_id (h : Heap) (a : Nat) : ¬ allocated (free (alloc h a) a) a :=
  free_not_allocated (alloc h a) a

end Nonos.Heap
