/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The file-descriptor allocator (`src/process/process_fd_table.rs::allocate_min`).
Starting at `min_fd` it walks upward over the occupied table and hands back the
first descriptor that is free, or nothing once it reaches `MAX_PROCESS_FDS`. This
is the POSIX guarantee open and fcntl(F_DUPFD) rest on: the descriptor returned
is the lowest free one at or above the floor. The theorems below fix exactly
that: an allocated descriptor sits in `[min, bound)`, it was genuinely free, and
every descriptor from the floor up to it was occupied so nothing lower was
missed; and the allocator declines precisely when the whole window is full.
-/

namespace Nonos.FdAlloc

/-- Occupancy of the descriptor table: `true` when that number is in use. Models
    `table.contains_key(&fd)`. -/
abbrev Occupied := Nat → Bool

/-- Walk upward from `i`, at most `fuel` steps, returning the first free
    descriptor below `bound`. `fuel` is the width of the window `bound - i`, so
    the walk covers exactly `[i, bound)`; this is the `while table.contains_key`
    loop, which advances `fd` and stops at `MAX_PROCESS_FDS`. -/
def scan (occ : Occupied) (bound : Nat) : Nat → Nat → Option Nat
  | 0, _ => none
  | fuel + 1, i =>
    if i < bound then
      match occ i with
      | true => scan occ bound fuel (i + 1)
      | false => some i
    else none

/-- Allocate the lowest free descriptor in `[min, bound)`, where `bound` is
    `MAX_PROCESS_FDS`. The window width is `bound - min`. -/
def allocMin (occ : Occupied) (min bound : Nat) : Option Nat :=
  scan occ bound (bound - min) min

/-- A scanned descriptor is at or above the start. -/
theorem scan_lb (occ : Occupied) (bound fuel : Nat) :
    ∀ i fd, scan occ bound fuel i = some fd → i ≤ fd := by
  induction fuel with
  | zero => intro i fd h; simp [scan] at h
  | succ f ih =>
    intro i fd h
    simp only [scan] at h
    by_cases hb : i < bound
    · rw [if_pos hb] at h
      cases hoc : occ i with
      | true => simp only [hoc] at h; exact Nat.le_of_lt (Nat.lt_of_succ_le (ih (i + 1) fd h))
      | false => simp only [hoc] at h; injection h with h'; omega
    · rw [if_neg hb] at h; simp at h

/-- A scanned descriptor is below the bound. -/
theorem scan_ub (occ : Occupied) (bound fuel : Nat) :
    ∀ i fd, scan occ bound fuel i = some fd → fd < bound := by
  induction fuel with
  | zero => intro i fd h; simp [scan] at h
  | succ f ih =>
    intro i fd h
    simp only [scan] at h
    by_cases hb : i < bound
    · rw [if_pos hb] at h
      cases hoc : occ i with
      | true => simp only [hoc] at h; exact ih (i + 1) fd h
      | false => simp only [hoc] at h; injection h with h'; omega
    · rw [if_neg hb] at h; simp at h

/-- A scanned descriptor was genuinely free. -/
theorem scan_free (occ : Occupied) (bound fuel : Nat) :
    ∀ i fd, scan occ bound fuel i = some fd → occ fd = false := by
  induction fuel with
  | zero => intro i fd h; simp [scan] at h
  | succ f ih =>
    intro i fd h
    simp only [scan] at h
    by_cases hb : i < bound
    · rw [if_pos hb] at h
      cases hoc : occ i with
      | true => simp only [hoc] at h; exact ih (i + 1) fd h
      | false => simp only [hoc] at h; injection h with h'; rw [← h']; exact hoc
    · rw [if_neg hb] at h; simp at h

/-- No lower descriptor was skipped: everything from the start up to the returned
    one was occupied. This is the "lowest free" guarantee. -/
theorem scan_lowest (occ : Occupied) (bound fuel : Nat) :
    ∀ i fd, scan occ bound fuel i = some fd → ∀ j, i ≤ j → j < fd → occ j = true := by
  induction fuel with
  | zero => intro i fd h; simp [scan] at h
  | succ f ih =>
    intro i fd h j hij hjf
    simp only [scan] at h
    by_cases hb : i < bound
    · rw [if_pos hb] at h
      cases hoc : occ i with
      | true =>
        simp only [hoc] at h
        rcases Nat.eq_or_lt_of_le hij with he | hlt
        · rw [← he]; exact hoc
        · exact ih (i + 1) fd h j hlt hjf
      | false =>
        simp only [hoc] at h
        injection h with h'
        rw [← h'] at hjf
        exact absurd hjf (Nat.not_lt.mpr hij)
    · rw [if_neg hb] at h; simp at h

/-- If the scan declines, every descriptor in the window it covered was in use. -/
theorem scan_none (occ : Occupied) (bound fuel : Nat) :
    ∀ i, scan occ bound fuel i = none →
      ∀ j, i ≤ j → j < i + fuel → j < bound → occ j = true := by
  induction fuel with
  | zero => intro i _ j hij hjf; omega
  | succ f ih =>
    intro i h j hij hjf hjb
    simp only [scan] at h
    by_cases hb : i < bound
    · rw [if_pos hb] at h
      cases hoc : occ i with
      | true =>
        rcases Nat.eq_or_lt_of_le hij with he | hlt
        · rw [← he]; exact hoc
        · simp only [hoc] at h
          exact ih (i + 1) h j hlt (by omega) hjb
      | false => simp only [hoc] at h; simp at h
    · omega

/-- If every descriptor in the window is occupied, the scan declines. -/
theorem scan_full (occ : Occupied) (bound : Nat) (fuel : Nat) :
    ∀ i, (∀ j, i ≤ j → j < i + fuel → j < bound → occ j = true) →
      scan occ bound fuel i = none := by
  induction fuel with
  | zero => intro i _; rfl
  | succ f ih =>
    intro i hocc
    simp only [scan]
    by_cases hb : i < bound
    · rw [if_pos hb]
      have hi : occ i = true := hocc i (Nat.le_refl i) (by omega) hb
      simp only [hi]
      exact ih (i + 1) (fun j hj1 hj2 hj3 => hocc j (by omega) (by omega) hj3)
    · rw [if_neg hb]

/-- An allocated descriptor sits inside the window `[min, bound)`. -/
theorem alloc_in_range (occ : Occupied) (min bound fd : Nat)
    (h : allocMin occ min bound = some fd) : min ≤ fd ∧ fd < bound :=
  ⟨scan_lb occ bound (bound - min) min fd h, scan_ub occ bound (bound - min) min fd h⟩

/-- An allocated descriptor was free. -/
theorem alloc_free (occ : Occupied) (min bound fd : Nat)
    (h : allocMin occ min bound = some fd) : occ fd = false :=
  scan_free occ bound (bound - min) min fd h

/-- The lowest-free guarantee: nothing from the floor up to the returned
    descriptor was available. -/
theorem alloc_lowest (occ : Occupied) (min bound fd : Nat)
    (h : allocMin occ min bound = some fd) :
    ∀ j, min ≤ j → j < fd → occ j = true :=
  scan_lowest occ bound (bound - min) min fd h

/-- The allocator declines exactly when the whole window is full. -/
theorem alloc_none_full (occ : Occupied) (min bound : Nat) :
    allocMin occ min bound = none ↔ ∀ j, min ≤ j → j < bound → occ j = true := by
  constructor
  · intro h j hmj hjb
    exact scan_none occ bound (bound - min) min h j hmj (by omega) hjb
  · intro hfull
    exact scan_full occ bound (bound - min) min (fun j hj1 _ hj3 => hfull j hj1 hj3)

end Nonos.FdAlloc
