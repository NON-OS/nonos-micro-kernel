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

end Nonos.Paging
