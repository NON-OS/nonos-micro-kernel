/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the page-permission attenuation lattice. The real
kernel PTE flags and the Verus theorems in
`verification/verus/src/page_permissions.rs` implement it. Lean proves the
lattice property; the Verus proof discharges it on the u64 bit operations.
-/

namespace Nonos.Paging

/-- The permission-relevant flags of a page-table entry. -/
structure Flags where
  writable : Bool
  user : Bool
  executable : Bool

/-- `child` is confined by `parent` when it grants no permission `parent` lacks:
    a page mapping may only be narrowed, never widened, under refinement. -/
def Subset (child parent : Flags) : Prop :=
  (child.writable = true → parent.writable = true) ∧
  (child.user = true → parent.user = true) ∧
  (child.executable = true → parent.executable = true)

/-- Every flag set confines itself. -/
theorem subset_refl (f : Flags) : Subset f f :=
  ⟨fun h => h, fun h => h, fun h => h⟩

/-- Confinement is transitive, so a chain of narrowing steps never widens. -/
theorem subset_trans (a b c : Flags)
    (hab : Subset a b) (hbc : Subset b c) : Subset a c := by
  obtain ⟨w1, u1, x1⟩ := hab
  obtain ⟨w2, u2, x2⟩ := hbc
  exact ⟨fun h => w2 (w1 h), fun h => u2 (u1 h), fun h => x2 (x1 h)⟩

/-- A confined mapping that is not writable-and-executable stays non-W+X: it
    cannot gain both rights through refinement. -/
theorem confined_preserves_no_wx (child parent : Flags)
    (hsub : Subset child parent)
    (hparent : ¬ (parent.writable = true ∧ parent.executable = true)) :
    ¬ (child.writable = true ∧ child.executable = true) := by
  obtain ⟨w, _, x⟩ := hsub
  intro ⟨hcw, hcx⟩
  exact hparent ⟨w hcw, x hcx⟩

/-- Two Booleans that imply each other are equal. -/
private theorem bool_eq_of_imp {a b : Bool}
    (h1 : a = true → b = true) (h2 : b = true → a = true) : a = b := by
  cases a <;> cases b <;> simp_all

/-- Confinement is antisymmetric: two mappings that confine each other carry
    identical permissions. With `subset_refl` and `subset_trans` this makes the
    subset relation a genuine partial order, so "narrower than" is
    unambiguous. -/
theorem subset_antisymm (a b : Flags) (hab : Subset a b) (hba : Subset b a) :
    a.writable = b.writable ∧ a.user = b.user ∧ a.executable = b.executable := by
  obtain ⟨w1, u1, x1⟩ := hab
  obtain ⟨w2, u2, x2⟩ := hba
  exact ⟨bool_eq_of_imp w1 w2, bool_eq_of_imp u1 u2, bool_eq_of_imp x1 x2⟩

/-- The meet of two flag sets: a permission survives only if both hold it. -/
def meet (a b : Flags) : Flags :=
  ⟨a.writable && b.writable, a.user && b.user, a.executable && b.executable⟩

theorem meet_subset_left (a b : Flags) : Subset (meet a b) a := by
  refine ⟨?_, ?_, ?_⟩ <;> intro h <;> simp only [meet, Bool.and_eq_true] at h <;> exact h.1

theorem meet_subset_right (a b : Flags) : Subset (meet a b) b := by
  refine ⟨?_, ?_, ?_⟩ <;> intro h <;> simp only [meet, Bool.and_eq_true] at h <;> exact h.2

/-- The meet is the greatest lower bound: anything confined by both mappings is
    confined by their meet. Intersecting two mappings' permissions is exactly
    the narrowest view both agree on. -/
theorem meet_is_glb (a b c : Flags) (hca : Subset c a) (hcb : Subset c b) :
    Subset c (meet a b) := by
  obtain ⟨wa, ua, xa⟩ := hca
  obtain ⟨wb, ub, xb⟩ := hcb
  refine ⟨?_, ?_, ?_⟩ <;> intro h <;> simp only [meet, Bool.and_eq_true]
  · exact ⟨wa h, wb h⟩
  · exact ⟨ua h, ub h⟩
  · exact ⟨xa h, xb h⟩

/-- Meeting with any non-W+X mapping is non-W+X: intersecting permissions can
    never manufacture a writable-executable page. -/
theorem meet_no_wx_left (a b : Flags)
    (ha : ¬ (a.writable = true ∧ a.executable = true)) :
    ¬ ((meet a b).writable = true ∧ (meet a b).executable = true) := by
  intro ⟨hw, hx⟩
  simp only [meet, Bool.and_eq_true] at hw hx
  exact ha ⟨hw.1, hx.1⟩

end Nonos.Paging
