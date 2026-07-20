/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The concrete anti-rollback state machine the bootloader runs
(`nonos-bootloader/src/security/anti_rollback/state/{check,update}.rs`). Where
`Nonos.AntiRollback` states the abstract floor property, this models the actual
four-branch `check_kernel_version` with its trust gating: an uninitialized state
with no TPM refuses everything, version zero is refused, an initialized state
refuses any version below its stored floor, and `update_kernel_version` raises
the floor to the greater of the two. The theorems fix those branches and then
connect them to the abstract theorem: once initialized, the concrete check
accepts a version exactly when the abstract `Accepts` holds, so the property
proven in the abstract carries down to the code's real control flow, and the
floor the concrete update keeps is monotone, so a booted version burns every
older one.
-/

import Nonos.AntiRollback

namespace Nonos.AntiRollbackState

/-- The concrete state: whether the store is initialized, whether a TPM backs
    it, and the two monotone counters. Mirrors `AntiRollbackState` plus its inner
    `state`. -/
structure Concrete where
  initialized : Bool
  tpmAvailable : Bool
  minimumKernel : Nat
  kernelVersion : Nat

/-- The verdict of `check_kernel_version`, one constructor per `Result` arm. -/
inductive Verdict where
  | ok
  | errTpm
  | errZero
  | errTooOld
  deriving DecidableEq

/-- `check_kernel_version`, branch for branch. -/
def check (s : Concrete) (v : Nat) : Verdict :=
  if s.initialized = false ∧ s.tpmAvailable = false then .errTpm
  else if v = 0 then .errZero
  else if s.initialized = true ∧ v < s.minimumKernel then .errTooOld
  else .ok

/-- `update_kernel_version`: the floor and the recorded version each rise to the
    greater of their old value and the booted one. -/
def update (s : Concrete) (v : Nat) : Concrete :=
  { s with minimumKernel := max s.minimumKernel v, kernelVersion := max s.kernelVersion v }

/-- The abstract floor this concrete state projects to. -/
def toAbstract (s : Concrete) : AntiRollback.State := { floor := s.minimumKernel }

/-- With neither an initialized store nor a TPM, every version is refused: there
    is no trusted state to check against, so nothing is admitted. -/
theorem untrusted_refused (s : Concrete) (v : Nat)
    (hi : s.initialized = false) (ht : s.tpmAvailable = false) :
    check s v = .errTpm := by
  simp [check, hi, ht]

/-- Version zero is always refused. -/
theorem zero_refused (s : Concrete) (hguard : ¬ (s.initialized = false ∧ s.tpmAvailable = false)) :
    check s 0 = .errZero := by
  simp [check, hguard]

/-- An initialized store refuses any version below its floor: the anti-rollback
    check at the level of the real branches. Refusal is stated as "not ok"
    because a zero version below the floor is caught by the zero branch first;
    either way it is refused. -/
theorem old_refused (s : Concrete) (v : Nat) (hin : s.initialized = true)
    (hlt : v < s.minimumKernel) : check s v ≠ .ok := by
  unfold check
  have hguard : ¬ (s.initialized = false ∧ s.tpmAvailable = false) := by simp [hin]
  rw [if_neg hguard]
  by_cases hz : v = 0
  · rw [if_pos hz]; simp
  · rw [if_neg hz, if_pos ⟨hin, hlt⟩]; simp

/-- An accepted version on an initialized store is at or above the floor. -/
theorem accepted_meets_floor (s : Concrete) (v : Nat) (hin : s.initialized = true)
    (h : check s v = .ok) : s.minimumKernel ≤ v := by
  unfold check at h
  by_cases hg : s.initialized = false ∧ s.tpmAvailable = false
  · rw [if_pos hg] at h; exact absurd h (by simp)
  · rw [if_neg hg] at h
    by_cases hz : v = 0
    · rw [if_pos hz] at h; exact absurd h (by simp)
    · rw [if_neg hz] at h
      by_cases ho : s.initialized = true ∧ v < s.minimumKernel
      · rw [if_pos ho] at h; exact absurd h (by simp)
      · rw [if_neg ho] at h
        have := (not_and.mp ho) hin
        omega

/-- The refinement: on an initialized store the concrete check accepts exactly
    the versions the abstract `Accepts` admits, so every property proven of the
    abstract floor holds of the code's real control flow. -/
theorem refines_abstract (s : Concrete) (v : Nat) (hin : s.initialized = true) :
    check s v = .ok ↔ AntiRollback.Accepts (toAbstract s) v := by
  unfold check AntiRollback.Accepts toAbstract
  have hg : ¬ (s.initialized = false ∧ s.tpmAvailable = false) := by simp [hin]
  rw [if_neg hg]
  by_cases hz : v = 0
  · subst hz; simp
  · rw [if_neg hz]
    by_cases ho : s.initialized = true ∧ v < s.minimumKernel
    · rw [if_pos ho]
      constructor
      · intro hc; exact absurd hc (by simp)
      · intro ha; exact absurd ha.2 (Nat.not_le.mpr ho.2)
    · rw [if_neg ho]
      have hnlt : ¬ v < s.minimumKernel := fun hlt => ho ⟨hin, hlt⟩
      simp only [true_iff]
      exact ⟨hz, by omega⟩

/-- The concrete update never lowers the floor: the stored minimum only ever
    rises, whatever version boots. -/
theorem update_floor_monotone (s : Concrete) (v : Nat) :
    s.minimumKernel ≤ (update s v).minimumKernel := by
  simp [update, Nat.le_max_left]

/-- The recorded kernel version never falls either. -/
theorem update_version_monotone (s : Concrete) (v : Nat) :
    s.kernelVersion ≤ (update s v).kernelVersion := by
  simp [update, Nat.le_max_left]

/-- No rollback after a boot: once a version `v` has been accepted and recorded,
    every strictly older version is refused by the updated state. Raising the
    floor burns the past. -/
theorem no_rollback_after_update (s : Concrete) (v w : Nat)
    (hin : s.initialized = true) (hw : w < v) :
    check (update s v) w ≠ .ok := by
  have hin' : (update s v).initialized = true := hin
  have hfloor : v ≤ (update s v).minimumKernel := by
    simp [update, Nat.le_max_right]
  exact old_refused (update s v) w hin' (by omega)

end Nonos.AntiRollbackState
