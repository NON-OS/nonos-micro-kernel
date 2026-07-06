/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the capability security model. The real kernel bit
operations that implement it (`has = bits & bit ≠ 0`, `grant = bits | bit`,
`revoke = bits & !bit`, attenuation) are proven to match by the Verus theorems
in `verification/verus/src/capabilities.rs`. Lean proves the abstract lattice
property; the Verus proof discharges it on the bit representation.
-/

namespace Nonos.Capability

/-- A capability set as its membership function over capability ids. -/
def Caps := Nat → Bool

/-- Holding capability `b`. -/
def Grants (c : Caps) (b : Nat) : Prop := c b = true

def empty : Caps := fun _ => false
def grant (c : Caps) (b : Nat) : Caps := fun x => c x || x == b
def revoke (c : Caps) (b : Nat) : Caps := fun x => c x && x != b
def attenuate (c mask : Caps) : Caps := fun x => c x && mask x

/-- The empty token grants nothing. -/
theorem empty_grants_nothing (b : Nat) : ¬ Grants empty b := by
  unfold Grants empty; simp

/-- Granting adds exactly the requested capability. -/
theorem grant_adds (c : Caps) (b : Nat) : Grants (grant c b) b := by
  unfold Grants grant; simp

/-- Granting never removes a capability already held (monotone). -/
theorem grant_never_removes (c : Caps) (b x : Nat) (h : Grants c x) :
    Grants (grant c b) x := by
  unfold Grants grant at *; simp [h]

/-- Revoking drops exactly the named capability. -/
theorem revoke_drops (c : Caps) (b : Nat) : ¬ Grants (revoke c b) b := by
  unfold Grants revoke; simp

/-- Revoking leaves every other capability untouched. -/
theorem revoke_preserves_others (c : Caps) (b x : Nat) (h : x ≠ b) :
    Grants (revoke c b) x ↔ Grants c x := by
  unfold Grants revoke; simp [h]

/-- Attenuation confines: an attenuated token never grants a capability the
    original token did not hold. -/
theorem attenuate_confines (c mask : Caps) (x : Nat)
    (h : Grants (attenuate c mask) x) : Grants c x := by
  unfold Grants attenuate at *; simp at h; exact h.1

end Nonos.Capability
