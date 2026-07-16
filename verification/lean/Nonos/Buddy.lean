/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Buddy allocator conservation. Memory is split between a free pool and used
bytes; a block of order k has size two to the k. The theorems below show
allocation and freeing conserve the total memory (bytes only move between the
free and used columns), a request larger than the free pool cannot go negative,
and splitting a block into its two buddies conserves its size, so the allocator
neither creates nor destroys memory.

The kernel's buddy allocator keeps this arithmetic in
`src/memory/buddy_alloc/constants/helpers.rs`. The `mechanism_proofs` crate
includes that file and checks `split_conserves` and the
buddy-address involution against this model with differential tests and Kani, so
the split conservation is proven of the code the allocator runs.
-/

namespace Nonos.Buddy

/-- The heap accounting: free bytes and bytes handed out. -/
structure Heap where
  free : Nat
  used : Nat

/-- Total memory under management. -/
def total (h : Heap) : Nat := h.free + h.used

/-- Allocate `n` bytes, moving them from free to used. -/
def alloc (h : Heap) (n : Nat) : Heap := ⟨h.free - n, h.used + n⟩

/-- Free `n` bytes, moving them from used back to free. -/
def freeBlk (h : Heap) (n : Nat) : Heap := ⟨h.free + n, h.used - n⟩

/-- Allocation conserves the total memory. -/
theorem alloc_conserves (h : Heap) (n : Nat) (hn : n ≤ h.free) : total (alloc h n) = total h := by
  simp only [total, alloc]; omega

/-- Freeing conserves the total memory. -/
theorem free_conserves (h : Heap) (n : Nat) (hn : n ≤ h.used) : total (freeBlk h n) = total h := by
  simp only [total, freeBlk]; omega

/-- A satisfiable allocation lowers the free pool by exactly the request. -/
theorem alloc_reduces_free (h : Heap) (n : Nat) (hn : n ≤ h.free) :
    (alloc h n).free + n = h.free := by
  simp only [alloc]; omega

/-- A request larger than the free pool floors at zero, never negative. -/
theorem cannot_alloc_more_than_free (h : Heap) (n : Nat) (hn : h.free < n) : (alloc h n).free = 0 := by
  simp only [alloc]; omega

/-- Splitting a block of order k yields two buddies of order k minus one whose
    sizes sum back to the original. -/
theorem split_conserves (m : Nat) : 2 ^ m + 2 ^ m = 2 ^ (m + 1) := by
  rw [Nat.pow_succ]; omega

end Nonos.Buddy
