/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

MMIO window confinement. A driver is granted a set of memory-mapped register
windows; a register access is permitted only if it lands in a granted window.
The theorems below show a driver with no grant can touch no register, a
permitted access lies inside a granted window, and an access outside every
window is denied, so a driver never pokes a device it was not handed.

The kernel admits a window through `validate_mmio_region`, whose validity
arithmetic lives in `src/drivers/security/mmio_range.rs`. The `mechanism_proofs`
crate includes that file and proves with Kani that an admitted window is
non-empty and does not wrap the address space, so the well-formedness this model
assumes of a window is proven of the code that admits it.
-/

namespace Nonos.Mmio

/-- A memory-mapped register window: base address and byte size. -/
structure Window where
  base : Nat
  size : Nat

/-- Address `addr` lies within window `w`. -/
def contains (w : Window) (addr : Nat) : Prop := w.base ≤ addr ∧ addr < w.base + w.size

/-- A driver's granted MMIO windows. -/
abbrev Grant := List Window

/-- An access is permitted iff some granted window contains it. -/
def permitted (g : Grant) (addr : Nat) : Prop := ∃ w ∈ g, contains w addr

/-- A driver with no grant can touch no register. -/
theorem empty_grant_denies (addr : Nat) : ¬ permitted [] addr := by
  intro h
  obtain ⟨w, hw, _⟩ := h
  exact absurd hw (List.not_mem_nil w)

/-- A permitted access lies inside a granted window. -/
theorem permitted_in_window (g : Grant) (addr : Nat) (h : permitted g addr) :
    ∃ w ∈ g, w.base ≤ addr ∧ addr < w.base + w.size := h

/-- Granting a window never revokes an existing permission. -/
theorem grant_monotone (g : Grant) (w : Window) (addr : Nat) (h : permitted g addr) :
    permitted (w :: g) addr := by
  obtain ⟨v, hv, hc⟩ := h
  exact ⟨v, List.mem_cons_of_mem w hv, hc⟩

/-- An access outside every granted window is denied. -/
theorem outside_denied (g : Grant) (addr : Nat) (h : ∀ w ∈ g, ¬ contains w addr) :
    ¬ permitted g addr := by
  intro hp
  obtain ⟨w, hw, hc⟩ := hp
  exact h w hw hc

/-- A permitted access is below the window's end: it cannot spill past the
    device's register block. -/
theorem access_below_end (w : Window) (addr : Nat) (h : contains w addr) :
    addr < w.base + w.size := by
  unfold contains at h; exact h.2

/-- A zero-size window contains nothing: an empty grant window is inert. -/
theorem zero_size_empty (base addr : Nat) : ¬ contains (Window.mk base 0) addr := by
  simp only [contains]
  omega

end Nonos.Mmio
