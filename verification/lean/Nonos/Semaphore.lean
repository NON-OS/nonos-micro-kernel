/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Counting semaphore. The count tracks available permits, bounded above by a
capacity. The theorems below show an acquire needs a positive count and lowers
it by one, a release never pushes the count past the capacity, and the
capacity bound is an invariant of both operations, so a semaphore never hands
out more permits than it was configured for and never underflows.

The kernel implements this as a lock-free counting semaphore in
`src/sys/sync/semaphore`. The permit arithmetic in that module's `pure.rs` is
the same computation these theorems describe, and the `sync_proofs` crate runs
it against this model with differential tests and Kani, so the property is
checked of the code the kernel executes, not only of the model.
-/

namespace Nonos.Semaphore

/-- A counting semaphore: available permits and the configured maximum. -/
structure Sem where
  count : Nat
  cap : Nat

/-- The invariant: never more permits available than the capacity. -/
def valid (s : Sem) : Prop := s.count ≤ s.cap

/-- A caller may acquire only when a permit is available. -/
def canAcquire (s : Sem) : Prop := 0 < s.count

/-- Take one permit. -/
def acquire (s : Sem) : Sem := ⟨s.count - 1, s.cap⟩

/-- Return one permit, saturating at the capacity. -/
def release (s : Sem) : Sem := ⟨min s.cap (s.count + 1), s.cap⟩

/-- Acquire lowers the count by exactly one when a permit was available. -/
theorem acquire_dec (s : Sem) (h : canAcquire s) : (acquire s).count + 1 = s.count := by
  simp only [acquire, canAcquire] at *; omega

/-- Acquire strictly reduces the available permits. -/
theorem acquire_strict (s : Sem) (h : canAcquire s) : (acquire s).count < s.count := by
  simp only [acquire, canAcquire] at *; omega

/-- Release never pushes the count past the capacity. -/
theorem release_le_cap (s : Sem) : (release s).count ≤ s.cap := by
  simp only [release]; omega

/-- The capacity bound survives an acquire. -/
theorem acquire_valid (s : Sem) (h : valid s) : valid (acquire s) := by
  simp only [valid, acquire] at *; omega

/-- The capacity bound survives a release. -/
theorem release_valid (s : Sem) : valid (release s) := by
  simp only [valid, release]; omega

/-- An acquire on a full-below-cap semaphore round-trips: release restores the
    prior count, so permits are conserved across a matched pair. -/
theorem acquire_release_roundtrip (s : Sem) (h : canAcquire s) (hv : valid s) :
    (release (acquire s)).count = s.count := by
  simp only [release, acquire, canAcquire, valid] at *; omega

end Nonos.Semaphore
