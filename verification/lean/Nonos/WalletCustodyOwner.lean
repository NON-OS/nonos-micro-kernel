/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the keyring's key-access gate
(`capsule_keyring/store/eth_secret.rs`). A stored key is usable only by the pid
that owns it, only while unlocked, and only before it expires against the
keyring's own clock. These theorems fix that no signing, export or retrieval
crosses an ownership boundary and that an expired key is dead at the moment of
use regardless of what time the caller claims.
-/

namespace Nonos.WalletCustodyOwner

/-- A stored key entry, reduced to the fields the access gate reads. -/
structure Entry where
  owner : Nat
  expiresAt : Nat
  locked : Bool

/-- The gate in `eth_secret`: the caller must be the recorded owner, the entry
    unlocked, and (when an expiry is set) the current time not past it. Expiry
    0 means no expiry. -/
def access (e : Entry) (caller now : Nat) : Bool :=
  decide (e.owner = caller) && (!e.locked) &&
    (decide (e.expiresAt = 0) || decide (now ≤ e.expiresAt))

/-- A caller who is not the owner is always denied, whatever the clock says. -/
theorem non_owner_denied (e : Entry) (caller now : Nat) (h : e.owner ≠ caller) :
    access e caller now = false := by
  unfold access; simp [h]

/-- A locked entry is denied even to its owner. -/
theorem locked_denied (e : Entry) (caller now : Nat) (h : e.locked = true) :
    access e caller now = false := by
  unfold access; simp [h]

/-- An entry with a set expiry is denied once the clock is past it, even to its
    owner: expiry is enforced at use. -/
theorem expired_denied (e : Entry) (caller now : Nat)
    (hexp : e.expiresAt ≠ 0) (hpast : e.expiresAt < now) :
    access e caller now = false := by
  unfold access
  have h1 : decide (e.expiresAt = 0) = false := by simp [hexp]
  have h2 : decide (now ≤ e.expiresAt) = false := by simp; omega
  simp [h1, h2]

/-- The owner may use an unlocked, unexpired key. -/
theorem owner_live_allowed (e : Entry) (caller now : Nat)
    (howner : e.owner = caller) (hlock : e.locked = false)
    (hlive : now ≤ e.expiresAt) :
    access e caller now = true := by
  unfold access; simp [howner, hlock, hlive]

/-- Access implies the caller is the owner: the gate never grants a non-owner. -/
theorem access_implies_owner (e : Entry) (caller now : Nat)
    (h : access e caller now = true) : e.owner = caller := by
  unfold access at h
  simp at h
  exact h.1.1

end Nonos.WalletCustodyOwner
