/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The attestation acceptance predicate, composing the three guarantees the gate
enforces at once: the opening reaches the enrolled policy root, the transcript is
bound to the expected identity, and the measured image is in the policy. The
theorems below show acceptance implies all three, and, dually, a forged image not
in the policy, a proof drawn under the wrong identity, or an opening to the wrong
root is refused. This is the end-to-end soundness statement the capsule and kernel
gates rely on: nothing outside the policy attests.
-/

namespace Nonos.Stark.AttestSoundness

/-- What a verifier reads off a candidate attestation. -/
structure Attestation where
  measurement : Nat
  root : Nat
  context : Nat

/-- The gate accepts when the opening reaches the policy root, the context is the
    expected one, and the measurement is enrolled: all three, together. -/
def accept (policyRoot boundContext : Nat) (policy : List Nat) (a : Attestation) : Prop :=
  a.root = policyRoot ∧ a.context = boundContext ∧ a.measurement ∈ policy

/-- Acceptance implies the opening reached the enrolled policy root. -/
theorem accept_root (pr bc : Nat) (pol : List Nat) (a : Attestation)
    (h : accept pr bc pol a) : a.root = pr := h.1

/-- Acceptance implies the transcript was bound to the expected identity. -/
theorem accept_context (pr bc : Nat) (pol : List Nat) (a : Attestation)
    (h : accept pr bc pol a) : a.context = bc := h.2.1

/-- Acceptance implies the measured image is enrolled. -/
theorem accept_enrolled (pr bc : Nat) (pol : List Nat) (a : Attestation)
    (h : accept pr bc pol a) : a.measurement ∈ pol := h.2.2

/-- A forged image absent from the policy is refused, however it opens. -/
theorem forged_refused (pr bc : Nat) (pol : List Nat) (a : Attestation)
    (h : a.measurement ∉ pol) : ¬ accept pr bc pol a := by
  intro ha; exact h ha.2.2

/-- A proof drawn under the wrong identity is refused: no replay. -/
theorem wrong_context_refused (pr bc : Nat) (pol : List Nat) (a : Attestation)
    (h : a.context ≠ bc) : ¬ accept pr bc pol a := by
  intro ha; exact h ha.2.1

/-- An opening to the wrong root is refused: no rebinding to another policy. -/
theorem wrong_root_refused (pr bc : Nat) (pol : List Nat) (a : Attestation)
    (h : a.root ≠ pr) : ¬ accept pr bc pol a := by
  intro ha; exact h ha.1

/-- An empty policy accepts nothing: the gate is closed before enrollment. -/
theorem empty_policy_refuses (pr bc : Nat) (a : Attestation) : ¬ accept pr bc [] a := by
  intro ha; simpa using ha.2.2

/-- Acceptance is exactly the conjunction of the three checks. -/
theorem accept_iff (pr bc : Nat) (pol : List Nat) (a : Attestation) :
    accept pr bc pol a ↔ a.root = pr ∧ a.context = bc ∧ a.measurement ∈ pol := Iff.rfl

/-- An honest attestation of an enrolled image under the right identity is accepted. -/
theorem honest_accepted (pr bc m : Nat) (pol : List Nat) (h : m ∈ pol) :
    accept pr bc pol ⟨m, pr, bc⟩ := ⟨rfl, rfl, h⟩

/-- Acceptance is decidable: the gate always returns a verdict. -/
theorem accept_decidable (pr bc : Nat) (pol : List Nat) (a : Attestation) :
    accept pr bc pol a ∨ ¬ accept pr bc pol a := by
  by_cases h : accept pr bc pol a
  · exact Or.inl h
  · exact Or.inr h

/-- Two accepted attestations share the same root and context. -/
theorem accepted_agree (pr bc : Nat) (pol : List Nat) (a b : Attestation)
    (ha : accept pr bc pol a) (hb : accept pr bc pol b) :
    a.root = b.root ∧ a.context = b.context := by
  refine ⟨?_, ?_⟩
  · rw [ha.1, hb.1]
  · rw [ha.2.1, hb.2.1]

end Nonos.Stark.AttestSoundness
