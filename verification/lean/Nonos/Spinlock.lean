/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Spinlock exclusion. A spinlock is a single held bit taken by a compare-and-set.
The theorems below show a try-acquire fails on a held lock and succeeds on a
free one, and a second acquire against an already held lock always fails, so
two holders can never coexist.
-/

namespace Nonos.Spinlock

/-- A spinlock: held or free. -/
structure Lock where
  held : Bool

/-- The lock is held. -/
def isHeld (l : Lock) : Prop := l.held = true

/-- The taken lock. -/
def acquire : Lock := ⟨true⟩

/-- The released lock. -/
def release : Lock := ⟨false⟩

/-- A compare-and-set acquire: takes the lock only when it is free. -/
def tryAcquire (l : Lock) : Option Lock :=
  if l.held then none else some acquire

/-- Acquiring yields a held lock. -/
theorem acquire_held : isHeld acquire := rfl

/-- Releasing yields a free lock. -/
theorem release_free : ¬ isHeld release := by
  simp [isHeld, release]

/-- A try-acquire fails when the lock is held. -/
theorem try_fails_when_held (l : Lock) (h : l.held = true) : tryAcquire l = none := by
  simp [tryAcquire, h]

/-- A try-acquire succeeds when the lock is free. -/
theorem try_succeeds_when_free (l : Lock) (h : l.held = false) : tryAcquire l = some acquire := by
  simp [tryAcquire, h]

/-- A second acquire against a held lock always fails: no double acquire. -/
theorem no_double_acquire : tryAcquire acquire = none := by
  simp [tryAcquire, acquire]

end Nonos.Spinlock
