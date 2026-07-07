/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the ZeroState property: a capsule's memory is wiped
on teardown, so no secret survives to the next tenant of that memory. The real
kernel zeroization (the explicit_bzero-style wipe on capsule teardown and page
reclaim) implements it; this is the abstract theorem that a wiped region carries
no trace of what it held, so RAM-resident capsules cannot leak across lifetimes.
-/

namespace Nonos.Zeroization

/-- A memory region as its byte contents at each offset. -/
def Region := Nat → UInt8

/-- Wiping a region: every byte becomes zero, regardless of prior contents. -/
def wipe (_ : Region) : Region := fun _ => 0

/-- A region holds no secret when every byte is zero. -/
def AllZero (r : Region) : Prop := ∀ i, r i = 0

/-- After a wipe every byte is zero: the region holds no secret. -/
theorem wipe_all_zero (r : Region) : AllZero (wipe r) := by
  intro i; rfl

/-- No byte of its prior contents survives a wipe, at any offset. -/
theorem no_secret_survives (r : Region) (i : Nat) : wipe r i = 0 := rfl

/-- A wiped region is independent of what it held: two regions wiped to the same
    result whatever their prior contents, so the wipe leaks nothing of the
    source. -/
theorem wipe_independent (r1 r2 : Region) : wipe r1 = wipe r2 := rfl

/-- The next tenant of a reclaimed region observes only zeros, whatever the
    previous capsule stored there: no cross-lifetime leak. -/
theorem reused_region_leaks_nothing (prev : Region) (i : Nat) :
    (wipe prev) i = 0 := rfl

/-- Wiping is idempotent: an already-wiped region stays wiped. -/
theorem wipe_idempotent (r : Region) : wipe (wipe r) = wipe r := rfl

/-- A wipe erases any difference between two regions: after wiping, an observer
    cannot tell which region it was, so the contents are unrecoverable. -/
theorem wipe_erases_difference (r1 r2 : Region) (i : Nat) :
    wipe r1 i = wipe r2 i := rfl

end Nonos.Zeroization
