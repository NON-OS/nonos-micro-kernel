/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the anti-rollback security property. This is the
abstract theorem; the real bootloader code that implements it is proven to match
by the Rust/Kani harnesses in `nonos-bootloader/boot_proofs` (accept iff
non-zero and at or above the floor; the floor never lowers; no rollback after a
successful boot). Lean proves the property; the code proofs discharge it on the
implementation, so the model is connected to what actually runs.
-/

namespace Nonos.AntiRollback

/-- Abstract anti-rollback state: a monotone minimum-version floor. -/
structure State where
  floor : Nat

/-- A version is acceptable iff it is non-zero and at least the stored floor. -/
def Accepts (s : State) (v : Nat) : Prop := v ≠ 0 ∧ s.floor ≤ v

instance (s : State) (v : Nat) : Decidable (Accepts s v) := by
  unfold Accepts; exact inferInstance

/-- Booting an accepted version raises the floor to the greater of the two. -/
def update (s : State) (v : Nat) : State :=
  if Accepts s v then { floor := max s.floor v } else s

/-- Version 0 is never acceptable, whatever the floor. -/
theorem zero_never_accepted (s : State) : ¬ Accepts s 0 := by
  unfold Accepts; omega

/-- An update never lowers the floor. -/
theorem update_never_lowers_floor (s : State) (v : Nat) :
    s.floor ≤ (update s v).floor := by
  unfold update
  split
  · exact Nat.le_max_left _ _
  · exact Nat.le_refl _

/-- The floor is a lower bound on every accepted version. -/
theorem accepted_at_least_floor (s : State) (v : Nat) (h : Accepts s v) :
    s.floor ≤ v := h.2

/-- No rollback: after a version `v` boots, every strictly older version is
    rejected forever. -/
theorem no_rollback_after_boot (s : State) (v w : Nat)
    (hv : Accepts s v) (hw : w < v) : ¬ Accepts (update s v) w := by
  have hfloor : s.floor ≤ v := hv.2
  unfold update
  rw [if_pos hv]
  unfold Accepts
  -- Reduce the structure projection so `omega` sees the new floor explicitly.
  show ¬ (w ≠ 0 ∧ max s.floor v ≤ w)
  -- The new floor is `max s.floor v ≥ v > w`, so `w` cannot meet it.
  omega

/-- On an accepted boot the floor rises to exactly the greater of the old floor
    and the booted version: a complete characterization of the update. -/
theorem update_raises_to_max (s : State) (v : Nat) (h : Accepts s v) :
    (update s v).floor = max s.floor v := by
  unfold update
  rw [if_pos h]

/-- Re-booting the same version is stable: the floor does not change on a second
    identical update. -/
theorem update_stable (s : State) (v : Nat) (h : Accepts s v) :
    (update (update s v) v).floor = (update s v).floor := by
  have hfloor : s.floor ≤ v := h.2
  have hus : update s v = { floor := max s.floor v } := by
    unfold update; rw [if_pos h]
  rw [hus]
  have h2 : Accepts { floor := max s.floor v } v := by
    refine ⟨h.1, ?_⟩
    show max s.floor v ≤ v
    omega
  rw [update_raises_to_max { floor := max s.floor v } v h2]
  show max (max s.floor v) v = max s.floor v
  omega

/-- One boot attempt: an accepted version commits, a rejected one changes
    nothing. This is the bootloader's whole per-boot effect on the floor. -/
def step (s : State) (v : Nat) : State :=
  if Accepts s v then update s v else s

/-- A whole execution: any sequence of boot attempts, hostile or not. -/
def run (s : State) : List Nat → State
  | [] => s
  | v :: vs => run (step s v) vs

/-- One step never lowers the floor. -/
theorem step_never_lowers_floor (s : State) (v : Nat) :
    s.floor ≤ (step s v).floor := by
  unfold step
  split
  · exact update_never_lowers_floor s v
  · exact Nat.le_refl _

/-- The floor never decreases across any execution: every trace of boot
    attempts, of any length and content, leaves the floor at least where it
    started. This lifts the single-step theorem to whole histories. -/
theorem floor_never_decreases_over_any_trace (s : State) (vs : List Nat) :
    s.floor ≤ (run s vs).floor := by
  induction vs generalizing s with
  | nil => exact Nat.le_refl _
  | cons v vs ih =>
    exact Nat.le_trans (step_never_lowers_floor s v) (ih (step s v))

/-- No rollback over any execution: once version `v` boots, no strictly older
    version is accepted at any later point of any trace whatsoever. The
    attacker chooses every subsequent boot attempt and still cannot get one
    accepted. -/
theorem no_rollback_over_any_trace (s : State) (v w : Nat) (vs : List Nat)
    (hv : Accepts s v) (hw : w < v) : ¬ Accepts (run (update s v) vs) w := by
  intro hacc
  have hstart : v ≤ (update s v).floor := by
    rw [update_raises_to_max s v hv]
    omega
  have hrun : (update s v).floor ≤ (run (update s v) vs).floor :=
    floor_never_decreases_over_any_trace (update s v) vs
  have : (run (update s v) vs).floor ≤ w := hacc.2
  omega

end Nonos.AntiRollback
