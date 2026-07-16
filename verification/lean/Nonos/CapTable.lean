/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Per-capsule capability table. Each capsule holds a set of capabilities; the
kernel grants and revokes individual capabilities. The theorems below show a
granted capability is held, a revoked capability is not held (revoke takes
effect), and grant/revoke of one capability on one capsule disturbs no other
capability and no other capsule, so authority changes are exact and never
leak across capsules.
-/

namespace Nonos.CapTable

/-- A capsule's capability set, as a bit-membership function. -/
def Caps := Nat → Bool

/-- The table: each capsule id maps to its capability set. -/
def Table := Nat → Caps

/-- Capsule `owner` holds capability `cap`. -/
def holds (t : Table) (owner cap : Nat) : Prop := t owner cap = true

/-- Grant `cap` to `owner`. -/
def grant (t : Table) (owner cap : Nat) : Table :=
  fun o => if o = owner then (fun c => if c = cap then true else t o c) else t o

/-- Revoke `cap` from `owner`. -/
def revoke (t : Table) (owner cap : Nat) : Table :=
  fun o => if o = owner then (fun c => if c = cap then false else t o c) else t o

/-- A granted capability is held. -/
theorem grant_holds (t : Table) (o c : Nat) : holds (grant t o c) o c := by
  simp only [holds, grant]; simp

/-- A revoked capability is not held: revoke takes effect. -/
theorem revoke_not_holds (t : Table) (o c : Nat) : ¬ holds (revoke t o c) o c := by
  simp only [holds, revoke]; simp

/-- Granting one capability disturbs no other capability of the same capsule. -/
theorem grant_other_cap (t : Table) (o c d : Nat) (hne : d ≠ c) :
    holds (grant t o c) o d ↔ holds t o d := by
  simp only [holds, grant]; simp [hne]

/-- Granting to one capsule disturbs no other capsule. -/
theorem grant_other_owner (t : Table) (o p c d : Nat) (hne : p ≠ o) :
    holds (grant t o c) p d ↔ holds t p d := by
  simp only [holds, grant]; simp [hne]

/-- Revoking one capability disturbs no other capability of the same capsule. -/
theorem revoke_other_cap (t : Table) (o c d : Nat) (hne : d ≠ c) :
    holds (revoke t o c) o d ↔ holds t o d := by
  simp only [holds, revoke]; simp [hne]

/-- Revoking from one capsule disturbs no other capsule. -/
theorem revoke_other_owner (t : Table) (o p c d : Nat) (hne : p ≠ o) :
    holds (revoke t o c) p d ↔ holds t p d := by
  simp only [holds, revoke]; simp [hne]

/-- Granting then revoking the same capability leaves it revoked: revoke wins,
    so a re-grant cannot be smuggled in by a prior grant. -/
theorem grant_then_revoke (t : Table) (o c : Nat) :
    ¬ holds (revoke (grant t o c) o c) o c := revoke_not_holds (grant t o c) o c

end Nonos.CapTable
