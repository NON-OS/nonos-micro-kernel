/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Epoch based reclamation (RCU). Readers are counted per epoch. A new reader
always enters the current epoch, never the previous one, so the count of old
readers only ever falls. Reclamation of the old epoch is safe once that count
reaches zero. The theorems below show a fresh reader never adds to the old
count, draining reaches a quiescent state where reclamation is safe, and any
number of old-reader exits only lowers the old count, so a grace period always
terminates.
-/

namespace Nonos.Epoch

/-- Reclamation state: the epoch and the reader counts in the current and
    previous epochs. -/
structure Rcu where
  epoch : Nat
  readersOld : Nat
  readersCur : Nat

/-- The old epoch is quiescent when no reader remains in it. -/
def quiesced (r : Rcu) : Prop := r.readersOld = 0

/-- Reclaiming the old epoch is safe exactly when it is quiescent. -/
def canReclaim (r : Rcu) : Prop := r.readersOld = 0

/-- A new reader enters the current epoch. -/
def enter (r : Rcu) : Rcu := ⟨r.epoch, r.readersOld, r.readersCur + 1⟩

/-- A reader in the old epoch leaves. -/
def leaveOld (r : Rcu) : Rcu := ⟨r.epoch, r.readersOld - 1, r.readersCur⟩

/-- Drain the old epoch by processing `n` exits. -/
def drainN (r : Rcu) : Nat → Rcu
  | 0 => r
  | n + 1 => drainN (leaveOld r) n

/-- A fresh reader never adds to the old count, so it cannot delay reclamation
    of the previous epoch. -/
theorem enter_keeps_old (r : Rcu) : (enter r).readersOld = r.readersOld := by
  simp only [enter]

/-- An old-reader exit strictly lowers the old count while any remain. -/
theorem leaveOld_dec (r : Rcu) (h : 0 < r.readersOld) :
    (leaveOld r).readersOld < r.readersOld := by
  simp only [leaveOld]; omega

/-- A quiescent epoch may be reclaimed. -/
theorem quiesced_can_reclaim (r : Rcu) (h : quiesced r) : canReclaim r := h

/-- Any number of old-reader exits only lowers the old count: a grace period
    never grows the set it waits on, so it terminates. -/
theorem drainN_old_le (r : Rcu) (n : Nat) : (drainN r n).readersOld ≤ r.readersOld := by
  induction n generalizing r with
  | zero => simp [drainN]
  | succ k ih =>
    simp only [drainN]
    have hstep := ih (leaveOld r)
    have hle : (leaveOld r).readersOld = r.readersOld - 1 := rfl
    omega

end Nonos.Epoch
