/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

TLB coherence. The TLB caches which virtual pages are mapped. The theorems
below show a full flush leaves nothing cached, invalidating a page evicts
exactly it and no other, and inserting a page caches it, so a stale
translation never survives a flush or a targeted invalidation, the property
that makes an unmap actually take effect.
-/

namespace Nonos.Tlb

/-- The TLB as a validity predicate over virtual page numbers. -/
def Tlb := Nat → Bool

/-- A page's translation is cached. -/
def cached (t : Tlb) (vpn : Nat) : Prop := t vpn = true

/-- A full flush: nothing cached. -/
def flush : Tlb := fun _ => false

/-- Evict a single page. -/
def invalidate (t : Tlb) (vpn : Nat) : Tlb := fun v => if v = vpn then false else t v

/-- Cache a page's translation. -/
def insert (t : Tlb) (vpn : Nat) : Tlb := fun v => if v = vpn then true else t v

/-- A flush leaves nothing cached: no stale translation survives. -/
theorem flush_empties (vpn : Nat) : ¬ cached flush vpn := by
  simp only [cached, flush]; simp

/-- Invalidating a page evicts it. -/
theorem invalidate_evicts (t : Tlb) (vpn : Nat) : ¬ cached (invalidate t vpn) vpn := by
  simp only [cached, invalidate]; simp

/-- Invalidating one page disturbs no other. -/
theorem invalidate_preserves (t : Tlb) (vpn wpn : Nat) (hne : wpn ≠ vpn) :
    cached (invalidate t vpn) wpn ↔ cached t wpn := by
  simp only [cached, invalidate]; simp [hne]

/-- Inserting a page caches its translation. -/
theorem insert_caches (t : Tlb) (vpn : Nat) : cached (insert t vpn) vpn := by
  simp only [cached, insert]; simp

/-- Inserting one page disturbs no other. -/
theorem insert_preserves (t : Tlb) (vpn wpn : Nat) (hne : wpn ≠ vpn) :
    cached (insert t vpn) wpn ↔ cached t wpn := by
  simp only [cached, insert]; simp [hne]

/-- Inserting then invalidating the same page leaves it evicted: an unmap after
    a fill still takes effect. -/
theorem insert_then_invalidate (t : Tlb) (vpn : Nat) :
    ¬ cached (invalidate (insert t vpn) vpn) vpn := invalidate_evicts (insert t vpn) vpn

end Nonos.Tlb
