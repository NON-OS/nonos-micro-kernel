/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Process reaping. A process table maps each pid to a state: free, running, or
zombie. Exiting turns a running slot into a zombie; reaping a zombie returns
the slot to free. The theorems below show reaping frees the slot and leaves no
zombie behind, reaping one pid disturbs no other, and the exit-then-reap
lifecycle ends in a free slot, so a reaped process leaks no table entry.
-/

namespace Nonos.Reaper

/-- The state of a process table slot. -/
inductive St where
  | free
  | running
  | zombie

/-- A process table. -/
def Table := Nat → St

/-- Start a process. -/
def spawn (t : Table) (pid : Nat) : Table := fun p => if p = pid then St.running else t p

/-- A process exits, becoming a zombie awaiting reaping. -/
def exit (t : Table) (pid : Nat) : Table := fun p => if p = pid then St.zombie else t p

/-- Reap a zombie, freeing its slot. -/
def reap (t : Table) (pid : Nat) : Table := fun p => if p = pid then St.free else t p

/-- Whether a slot is a zombie. -/
def isZombie : St → Bool
  | St.zombie => true
  | _ => false

/-- Spawning marks the slot running. -/
theorem spawn_running (t : Table) (pid : Nat) : (spawn t pid) pid = St.running := by
  simp [spawn]

/-- Exiting marks the slot a zombie. -/
theorem exit_zombie (t : Table) (pid : Nat) : (exit t pid) pid = St.zombie := by
  simp [exit]

/-- Reaping frees the slot. -/
theorem reap_frees (t : Table) (pid : Nat) : (reap t pid) pid = St.free := by
  simp [reap]

/-- After reaping, the slot is not a zombie: no leaked table entry. -/
theorem reaped_not_zombie (t : Table) (pid : Nat) : isZombie ((reap t pid) pid) = false := by
  simp [reap_frees, isZombie]

/-- Reaping one pid disturbs no other slot. -/
theorem reap_other (t : Table) (pid other : Nat) (h : other ≠ pid) :
    (reap t pid) other = t other := by
  simp [reap, h]

/-- The exit-then-reap lifecycle ends in a free slot. -/
theorem exit_then_reap_frees (t : Table) (pid : Nat) :
    (reap (exit t pid) pid) pid = St.free := by
  simp [reap]

end Nonos.Reaper
