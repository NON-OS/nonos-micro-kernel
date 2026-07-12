/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

IOMMU DMA confinement. A device is granted a set of DMA windows; an access is
permitted only if some granted window covers it. The theorems below show a
device with no grant can touch nothing, a permitted access always lies inside a
granted window (DMA never escapes the grant), and an access outside every
window is denied.
-/

namespace Nonos.Iommu

/-- A DMA window: a base page and a length in pages. -/
structure Window where
  base : Nat
  len : Nat

/-- Address `a` lies within window `w`. -/
def Window.covers (w : Window) (a : Nat) : Prop := w.base ≤ a ∧ a < w.base + w.len

instance (w : Window) (a : Nat) : Decidable (w.covers a) := by
  unfold Window.covers; exact inferInstance

/-- A device's granted DMA windows. -/
abbrev Grant := List Window

/-- An access to `a` is permitted iff some granted window covers it. -/
def permitted (g : Grant) (a : Nat) : Prop := ∃ w ∈ g, w.covers a

/-- With no granted windows, nothing is permitted: an ungranted device cannot
    touch memory by DMA. -/
theorem empty_grant_denies (a : Nat) : ¬ permitted [] a := by
  intro h
  obtain ⟨w, hw, _⟩ := h
  exact absurd hw (List.not_mem_nil w)

/-- A permitted access lies inside a granted window: DMA never escapes the
    grant. -/
theorem permitted_within_grant (g : Grant) (a : Nat) (h : permitted g a) :
    ∃ w ∈ g, w.base ≤ a ∧ a < w.base + w.len := h

/-- Granting an additional window never revokes an existing permission. -/
theorem grant_monotone (g : Grant) (w : Window) (a : Nat) (h : permitted g a) :
    permitted (w :: g) a := by
  obtain ⟨v, hv, hcov⟩ := h
  exact ⟨v, List.mem_cons_of_mem w hv, hcov⟩

/-- An access outside every granted window is denied. -/
theorem outside_all_denied (g : Grant) (a : Nat)
    (h : ∀ w ∈ g, ¬ w.covers a) : ¬ permitted g a := by
  intro hp
  obtain ⟨w, hw, hcov⟩ := hp
  exact h w hw hcov

/-- A zero-length window covers nothing: an empty grant window is inert. -/
theorem zero_len_covers_nothing (base a : Nat) :
    ¬ (Window.mk base 0).covers a := by
  intro h
  simp only [Window.covers] at h
  obtain ⟨h1, h2⟩ := h
  omega

end Nonos.Iommu
