/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Mutual exclusion. A mutex carries at most one owner. The theorems below show a
locked mutex has a unique owner, a try-lock fails on a held mutex and succeeds
on a free one, and unlocking leaves the mutex free, so two holders can never
coexist.
-/

namespace Nonos.Mutex

/-- A mutex holds an optional owner id. -/
structure Mutex where
  owner : Option Nat

/-- The mutex is held. -/
def held (m : Mutex) : Prop := m.owner.isSome = true

/-- Acquire the mutex for a pid. -/
def lock (pid : Nat) : Mutex := ⟨some pid⟩

/-- Release the mutex. -/
def unlock : Mutex := ⟨none⟩

/-- Attempt to take the mutex: succeeds only when it is free. -/
def tryLock (m : Mutex) (pid : Nat) : Option Mutex :=
  match m.owner with
  | none => some (lock pid)
  | some _ => none

/-- A locked mutex is held. -/
theorem lock_held (pid : Nat) : held (lock pid) := by
  simp [held, lock]

/-- An unlocked mutex is free. -/
theorem unlock_free : ¬ held unlock := by
  simp only [held, unlock]; simp

/-- At most one owner: any two owners of the same mutex are equal. -/
theorem owner_unique (m : Mutex) (pid qid : Nat)
    (hp : m.owner = some pid) (hq : m.owner = some qid) : pid = qid := by
  rw [hp] at hq; exact Option.some.inj hq

/-- Try-lock fails when the mutex is already held. -/
theorem trylock_fails_when_held (m : Mutex) (pid qid : Nat) (h : m.owner = some qid) :
    tryLock m pid = none := by
  simp only [tryLock, h]

/-- Try-lock succeeds when the mutex is free. -/
theorem trylock_succeeds_when_free (m : Mutex) (pid : Nat) (h : m.owner = none) :
    (tryLock m pid).isSome = true := by
  simp [tryLock, h]

/-- Locking hands the mutex to exactly the requesting pid. -/
theorem lock_owner (pid : Nat) : (lock pid).owner = some pid := rfl

end Nonos.Mutex
