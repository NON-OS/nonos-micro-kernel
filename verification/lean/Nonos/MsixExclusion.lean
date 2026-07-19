/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The MSI-X exclusion clamp (`src/hardware/broker/mmio/msix_exclusion.rs`). A
capsule may map a device BAR, but never the MSI-X table or PBA that live inside
it: those are programmed only through the broker's checked bind path, so exposing
them via a raw mapping would let a capsule steer interrupts around the allowlist.
Rather than reject a mapping that reaches into a protected region, `safe_length`
clamps its end down to the page boundary below the region start; a mapping that
begins inside a region clamps to zero. The theorem below fixes the guarantee the
clamp exists for: no address the clamped mapping covers ever falls inside the
protected region, whether the request straddled it or started within it.
-/

namespace Nonos.MsixExclusion

/-- The MMIO page size. -/
def page : Nat := 4096

/-- Round an address down to its page boundary, `addr & !(PAGE_SIZE - 1)`. -/
def pageFloor (x : Nat) : Nat := x - x % page

/-- The clamped end of a mapping `[offset, offset + length)` against a single
    protected region `[rStart, rEnd)`: if the request overlaps the region the end
    is pulled down to the page boundary below the region start, else it is left
    at `offset + length`. Mirrors the `end.min(region.0 & !(PAGE-1))` step. -/
def clampEnd (offset length rStart rEnd : Nat) : Nat :=
  if offset < rEnd ∧ rStart < offset + length then min (offset + length) (pageFloor rStart)
  else offset + length

/-- The safe mapping length: the clamped end back to the offset, saturating to
    zero. Mirrors `end.saturating_sub(offset)`. -/
def safeLen (offset length rStart rEnd : Nat) : Nat := clampEnd offset length rStart rEnd - offset

/-- Rounding down never increases an address. -/
theorem pageFloor_le (x : Nat) : pageFloor x ≤ x := by unfold pageFloor; omega

/-- The clamp never lengthens a request: the safe length is at most the
    requested length, so the clamp only ever shrinks a mapping. -/
theorem safeLen_le_length (offset length rStart rEnd : Nat) :
    safeLen offset length rStart rEnd ≤ length := by
  unfold safeLen clampEnd
  by_cases h : offset < rEnd ∧ rStart < offset + length
  · rw [if_pos h]
    have : min (offset + length) (pageFloor rStart) ≤ offset + length := Nat.min_le_left _ _
    omega
  · rw [if_neg h]; omega

/-- The exclusion guarantee: no address the clamped mapping covers falls inside
    the protected region. A capsule that maps a BAR can never reach a byte of the
    MSI-X table or PBA, whether its request straddled the region or began inside
    it (in which case the safe length is zero and it maps nothing). -/
theorem no_protected_byte_mapped (offset length rStart rEnd addr : Nat)
    (hlo : offset ≤ addr) (hhi : addr < offset + safeLen offset length rStart rEnd) :
    ¬ (rStart ≤ addr ∧ addr < rEnd) := by
  rintro ⟨hr1, hr2⟩
  have hpf : pageFloor rStart ≤ rStart := pageFloor_le rStart
  have key : addr < clampEnd offset length rStart rEnd := by
    unfold safeLen at hhi; omega
  unfold clampEnd at key
  by_cases hov : offset < rEnd ∧ rStart < offset + length
  · rw [if_pos hov] at key
    have hm : min (offset + length) (pageFloor rStart) ≤ pageFloor rStart := Nat.min_le_right _ _
    omega
  · rw [if_neg hov] at key
    by_cases ho : offset < rEnd
    · have : ¬ rStart < offset + length := not_and.mp hov ho
      omega
    · omega

end Nonos.MsixExclusion
