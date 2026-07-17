/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The policy epoch bound into every attestation context. When the enrolled policy is
rolled forward, its epoch is bumped, and because the epoch is part of the proving
context an attestation issued under an old policy no longer verifies. The theorems
below show the epoch is monotone, a bump strictly advances it, an attestation is
fresh only at the current epoch, and bumping invalidates every proof drawn under a
prior epoch: this is how a policy revocation takes effect without touching keys.
-/

namespace Nonos.Stark.PolicyEpoch

/-- An attestation is fresh when its epoch equals the current policy epoch. -/
def fresh (current attEpoch : Nat) : Prop := attEpoch = current

/-- Bump the policy epoch forward by one. -/
def bump (current : Nat) : Nat := current + 1

/-- A bump strictly advances the epoch. -/
theorem bump_advances (current : Nat) : current < bump current := by simp [bump]

/-- The epoch is monotone under a bump. -/
theorem bump_monotone (current : Nat) : current ≤ bump current := by simp [bump]

/-- An attestation at the current epoch is fresh. -/
theorem current_is_fresh (current : Nat) : fresh current current := rfl

/-- A bump invalidates an attestation issued under the current epoch. -/
theorem bump_invalidates (current att : Nat) (h : att = current) : ¬ fresh (bump current) att := by
  simp only [fresh, bump]; omega

/-- An attestation from any earlier epoch is stale after enough bumps. -/
theorem earlier_epoch_stale (current att : Nat) (h : att < current) : ¬ fresh current att := by
  simp only [fresh]; omega

/-- A future-dated attestation is not fresh at the current epoch. -/
theorem future_epoch_stale (current att : Nat) (h : current < att) : ¬ fresh current att := by
  simp only [fresh]; omega

/-- Freshness pins the epoch exactly: no window, no drift. -/
theorem fresh_pins_epoch (current att : Nat) (h : fresh current att) : att = current := h

/-- Bumping n times, as a closed form. -/
def bumpN (current n : Nat) : Nat := current + n

/-- Bumping zero times is a no-op. -/
theorem bumpN_zero (current : Nat) : bumpN current 0 = current := rfl

/-- Each further bump advances the closed form by one. -/
theorem bumpN_succ (current n : Nat) : bumpN current (n + 1) = bump (bumpN current n) := by
  simp only [bumpN, bump]; omega

/-- Bumping a positive number of times strictly advances the epoch. -/
theorem bumpN_advances (current n : Nat) (h : 0 < n) : current < bumpN current n := by
  simp only [bumpN]; omega

/-- Once bumped past an epoch, no later bump restores freshness for the old proof. -/
theorem no_epoch_replay (old current att : Nat) (hbumped : old < current) (h : att = old) :
    ¬ fresh current att := by
  simp only [fresh]; omega

end Nonos.Stark.PolicyEpoch
