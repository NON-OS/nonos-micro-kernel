/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Memory-region interval algebra. A region is a half-open interval [lo, hi) of
addresses. Containment, membership, disjointness and size are the primitives
every memory-safety argument in the kernel rests on, paging, the IOMMU, the
heap, capsule isolation. The theorems below are the full algebra: containment
is a partial order, disjoint regions share no address, a subregion never
exceeds its parent, and adjacency composes. All machine-checked.

The kernel's `MemRegion` keeps this range algebra in
`src/memory/region/overlap.rs`, which its `overlaps`, `contains` and
`contains_range` methods delegate to. The `mechanism_proofs` crate includes that
file and checks, with differential tests and Kani, that overlap is symmetric and
is exactly the negation of disjointness, so the core of this algebra is proven
of the code the memory manager runs.
-/

namespace Nonos.Interval

/-- A memory region as a half-open interval [lo, hi). -/
structure Iv where
  lo : Nat
  hi : Nat

/-- A region is valid when its low bound does not exceed its high bound. -/
def valid (a : Iv) : Prop := a.lo ≤ a.hi

/-- Address `x` lies in region `a`. -/
def mem (a : Iv) (x : Nat) : Prop := a.lo ≤ x ∧ x < a.hi

/-- `a` is contained in `b`. -/
def Subset (a b : Iv) : Prop := b.lo ≤ a.lo ∧ a.hi ≤ b.hi

/-- `a` and `b` share no address. -/
def Disjoint (a b : Iv) : Prop := a.hi ≤ b.lo ∨ b.hi ≤ a.lo

/-- The number of addresses in a region. -/
def size (a : Iv) : Nat := a.hi - a.lo

/-- The empty region at a point. -/
def empty (p : Nat) : Iv := ⟨p, p⟩

/-- Two regions are adjacent when the first ends where the second begins. -/
def Adjacent (a b : Iv) : Prop := a.hi = b.lo

/-- The union of two adjacent regions. -/
def join (a b : Iv) : Iv := ⟨a.lo, b.hi⟩

/-! ### Membership -/

theorem mem_iff (a : Iv) (x : Nat) : mem a x ↔ a.lo ≤ x ∧ x < a.hi := Iff.rfl

theorem mem_lo_le (a : Iv) (x : Nat) (h : mem a x) : a.lo ≤ x := h.1

theorem mem_lt_hi (a : Iv) (x : Nat) (h : mem a x) : x < a.hi := h.2

theorem not_mem_below (a : Iv) (x : Nat) (h : x < a.lo) : ¬ mem a x := by
  simp only [mem]; omega

theorem not_mem_above (a : Iv) (x : Nat) (h : a.hi ≤ x) : ¬ mem a x := by
  simp only [mem]; omega

theorem empty_no_mem (p x : Nat) : ¬ mem (empty p) x := by
  simp only [mem, empty]; omega

theorem valid_of_mem (a : Iv) (x : Nat) (h : mem a x) : valid a := by
  simp only [valid]; simp only [mem] at h; omega

/-! ### Size -/

theorem size_empty (p : Nat) : size (empty p) = 0 := by
  simp only [size, empty]; omega

theorem size_zero_iff (a : Iv) (h : valid a) : size a = 0 ↔ a.lo = a.hi := by
  simp only [size, valid] at *; constructor <;> (intro hh; omega)

theorem mem_of_size_pos (a : Iv) (h : 0 < size a) : mem a a.lo := by
  simp only [size] at h; simp only [mem]; omega

theorem size_pos_of_mem (a : Iv) (x : Nat) (h : mem a x) : 0 < size a := by
  simp only [mem] at h; simp only [size]; omega

/-! ### Containment is a partial order -/

theorem subset_refl (a : Iv) : Subset a a := by simp only [Subset]; omega

theorem subset_trans (a b c : Iv) (hab : Subset a b) (hbc : Subset b c) :
    Subset a c := by simp only [Subset] at *; omega

theorem subset_antisymm (a b : Iv) (hab : Subset a b) (hba : Subset b a) :
    a.lo = b.lo ∧ a.hi = b.hi := by simp only [Subset] at *; omega

/-- Containment transports membership: every address of a subregion is an
    address of its parent. -/
theorem mem_of_subset (a b : Iv) (x : Nat) (hsub : Subset a b) (hx : mem a x) :
    mem b x := by simp only [Subset] at hsub; simp only [mem] at *; omega

/-- A subregion never has more addresses than its parent. -/
theorem size_le_of_subset (a b : Iv) (ha : valid a) (hsub : Subset a b) :
    size a ≤ size b := by
  simp only [valid] at ha; simp only [Subset] at hsub; simp only [size]; omega

theorem empty_subset (p : Nat) (b : Iv) (hlo : b.lo ≤ p) (hhi : p ≤ b.hi) :
    Subset (empty p) b := by simp only [Subset, empty]; omega

/-! ### Disjointness -/

theorem disjoint_symm (a b : Iv) (h : Disjoint a b) : Disjoint b a := by
  simp only [Disjoint] at *; omega

/-- Disjoint regions share no address: the core memory-isolation fact. -/
theorem disjoint_not_mem (a b : Iv) (x : Nat) (hd : Disjoint a b)
    (ha : mem a x) : ¬ mem b x := by
  simp only [Disjoint] at hd; simp only [mem] at *; omega

/-- Contrapositive: a shared address means the regions are not disjoint. -/
theorem mem_both_not_disjoint (a b : Iv) (x : Nat)
    (ha : mem a x) (hb : mem b x) : ¬ Disjoint a b := by
  simp only [Disjoint]; simp only [mem] at *; omega

/-- An empty region below another is disjoint from it. -/
theorem empty_disjoint_below (p : Nat) (b : Iv) (h : p ≤ b.lo) :
    Disjoint (empty p) b := by simp only [Disjoint, empty]; omega

/-- A subregion of one of two disjoint regions is disjoint from the other. -/
theorem subset_of_disjoint (a b c : Iv) (hd : Disjoint a b) (hsub : Subset c a) :
    Disjoint c b := by
  simp only [Disjoint] at *; simp only [Subset] at hsub; omega

/-! ### Adjacency and composition -/

/-- Adjacent regions are disjoint. -/
theorem adjacent_disjoint (a b : Iv) (h : Adjacent a b) : Disjoint a b := by
  simp only [Adjacent] at h; simp only [Disjoint]; omega

/-- The join of adjacent valid regions is valid. -/
theorem join_valid (a b : Iv) (ha : valid a) (hb : valid b) (hadj : Adjacent a b) :
    valid (join a b) := by
  simp only [valid] at *; simp only [Adjacent] at hadj; simp only [join]; omega

/-- The join contains its left half. -/
theorem subset_join_left (a b : Iv) (hb : valid b) (hadj : Adjacent a b) :
    Subset a (join a b) := by
  simp only [valid] at hb; simp only [Adjacent] at hadj; simp only [Subset, join]; omega

/-- The join contains its right half. -/
theorem subset_join_right (a b : Iv) (ha : valid a) (hadj : Adjacent a b) :
    Subset b (join a b) := by
  simp only [valid] at ha; simp only [Adjacent] at hadj; simp only [Subset, join]; omega

/-- The join's size is the sum of the parts' sizes. -/
theorem size_join (a b : Iv) (ha : valid a) (hb : valid b) (hadj : Adjacent a b) :
    size (join a b) = size a + size b := by
  simp only [valid] at *; simp only [Adjacent] at hadj; simp only [size, join]; omega

/-- Every address of the left half is an address of the join. -/
theorem mem_join_of_mem_left (a b : Iv) (x : Nat) (hb : valid b) (hadj : Adjacent a b)
    (hx : mem a x) : mem (join a b) x := by
  simp only [valid] at hb; simp only [Adjacent] at hadj
  simp only [mem] at *; simp only [join]; omega

/-- Every address of the right half is an address of the join. -/
theorem mem_join_of_mem_right (a b : Iv) (x : Nat) (ha : valid a) (hadj : Adjacent a b)
    (hx : mem b x) : mem (join a b) x := by
  simp only [valid] at ha; simp only [Adjacent] at hadj
  simp only [mem] at *; simp only [join]; omega

end Nonos.Interval
