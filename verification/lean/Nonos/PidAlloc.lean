/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The PID selector (`src/process/core/table/pid_alloc.rs::choose_pid`). Starting
from a counter it probes forward, skipping any PID the activity predicate still
reports in use, and returns the first free PID together with the next counter
value; it gives up after a bounded number of probes. The chosen PID is never 0,
the counter it returns is never 0 (it wraps to 1 at the top of the range instead
of reaching 0), and a PID is handed out only when the predicate reports it free.
The theorems below fix exactly those three: 0 is a reserved non-PID that is never
allocated, an allocated PID is not already live, and the stored counter never
becomes the reserved value.
-/

namespace Nonos.PidAlloc

/-- `u32::MAX`, the top of the counter range. -/
def u32Max : Nat := 2 ^ 32 - 1

/-- The PID a given counter value yields: 0 is remapped to 1, so the result is
    never the reserved 0. Mirrors `if current == 0 { 1 } else { current }`. -/
def pidOf (current : Nat) : Nat := if current = 0 then 1 else current

/-- The next counter value: wraps to 1 at the top instead of overflowing to 0.
    Mirrors `if current >= u32::MAX - 1 { 1 } else { current + 1 }`. -/
def nextOf (current : Nat) : Nat := if current ≥ u32Max - 1 then 1 else current + 1

/-- Probe forward for a free PID, at most `fuel` times. Returns the free PID and
    the next counter, or nothing once the probe budget is spent. -/
def choose (isActive : Nat → Bool) : Nat → Nat → Option (Nat × Nat)
  | 0, _ => none
  | fuel + 1, current =>
    if isActive (pidOf current) then choose isActive fuel (nextOf current)
    else some (pidOf current, nextOf current)

/-- Any counter yields a non-zero PID. -/
theorem pidOf_ne_zero (current : Nat) : pidOf current ≠ 0 := by
  unfold pidOf
  by_cases h : current = 0
  · rw [if_pos h]; omega
  · rw [if_neg h]; exact h

/-- Any counter yields a non-zero next value. -/
theorem nextOf_ne_zero (current : Nat) : nextOf current ≠ 0 := by
  unfold nextOf
  by_cases h : current ≥ u32Max - 1
  · rw [if_pos h]; omega
  · rw [if_neg h]; omega

/-- An allocated PID is never the reserved 0. -/
theorem chosen_pid_ne_zero (isActive : Nat → Bool) (fuel : Nat) :
    ∀ current pid next, choose isActive fuel current = some (pid, next) → pid ≠ 0 := by
  induction fuel with
  | zero => intro current pid next h; simp [choose] at h
  | succ f ih =>
    intro current pid next h
    simp only [choose] at h
    by_cases ha : isActive (pidOf current)
    · rw [if_pos ha] at h; exact ih (nextOf current) pid next h
    · rw [if_neg ha] at h
      injection h with hpair
      injection hpair with hpid hnext
      rw [← hpid]; exact pidOf_ne_zero current

/-- The counter stored back is never the reserved 0, so the allocator never
    parks on a value that would yield a degenerate probe. -/
theorem chosen_next_ne_zero (isActive : Nat → Bool) (fuel : Nat) :
    ∀ current pid next, choose isActive fuel current = some (pid, next) → next ≠ 0 := by
  induction fuel with
  | zero => intro current pid next h; simp [choose] at h
  | succ f ih =>
    intro current pid next h
    simp only [choose] at h
    by_cases ha : isActive (pidOf current)
    · rw [if_pos ha] at h; exact ih (nextOf current) pid next h
    · rw [if_neg ha] at h
      injection h with hpair
      injection hpair with hpid hnext
      rw [← hnext]; exact nextOf_ne_zero current

/-- An allocated PID was reported free: the allocator never hands out a PID that
    is still live, so PIDs are never aliased between two processes. -/
theorem chosen_pid_inactive (isActive : Nat → Bool) (fuel : Nat) :
    ∀ current pid next, choose isActive fuel current = some (pid, next) → isActive pid = false := by
  induction fuel with
  | zero => intro current pid next h; simp [choose] at h
  | succ f ih =>
    intro current pid next h
    simp only [choose] at h
    by_cases ha : isActive (pidOf current)
    · rw [if_pos ha] at h; exact ih (nextOf current) pid next h
    · rw [if_neg ha] at h
      injection h with hpair
      injection hpair with hpid hnext
      rw [← hpid]
      cases hb : isActive (pidOf current) with
      | false => rfl
      | true => exact absurd hb ha

end Nonos.PidAlloc
