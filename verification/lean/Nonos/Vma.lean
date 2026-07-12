/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Address space regions. A virtual memory area covers a half-open range from a
base for a size. Two areas are disjoint when one ends at or before the other
begins. The theorems below show disjoint areas do not overlap and share no
address, overlap is symmetric, and an address inside an area lies within its
bounds, so a mapping placed in a disjoint hole cannot alias an existing one.
-/

namespace Nonos.Vma

/-- A virtual memory area: a base and a size. -/
structure Region where
  base : Nat
  size : Nat

/-- One past the last address of the area. -/
def hi (r : Region) : Nat := r.base + r.size

/-- An address falls inside the area. -/
def contains (r : Region) (addr : Nat) : Prop := r.base ≤ addr ∧ addr < hi r

/-- Two areas overlap. -/
def overlaps (a b : Region) : Prop := a.base < hi b ∧ b.base < hi a

/-- Two areas are disjoint: one ends at or before the other begins. -/
def disjoint (a b : Region) : Prop := hi a ≤ b.base ∨ hi b ≤ a.base

/-- Disjoint areas do not overlap. -/
theorem disjoint_not_overlaps (a b : Region) (h : disjoint a b) : ¬ overlaps a b := by
  simp only [disjoint, overlaps, hi] at *; omega

/-- Disjoint areas share no address: a mapping in the hole cannot alias. -/
theorem disjoint_no_shared_addr (a b : Region) (h : disjoint a b) (addr : Nat)
    (ha : contains a addr) : ¬ contains b addr := by
  simp only [disjoint, contains, hi] at *
  obtain ⟨ha1, ha2⟩ := ha
  omega

/-- Overlap is symmetric. -/
theorem overlaps_symm (a b : Region) (h : overlaps a b) : overlaps b a := by
  simp only [overlaps] at *; exact ⟨h.2, h.1⟩

/-- An address inside an area lies within its bounds. -/
theorem contains_in_bounds (r : Region) (addr : Nat) (h : contains r addr) : addr < hi r :=
  h.2

end Nonos.Vma
