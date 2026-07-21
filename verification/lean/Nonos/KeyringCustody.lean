/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the wallet's custody invariant: a private key is
released only to the process that owns it. This models the keyring's
ownership-gated access (`capsule_keyring` `store.eth_secret(id, caller_pid)`
with anti-spoof `resolve_caller`). The theorems establish that no caller other
than the owner ever obtains a secret, whatever the store holds, so the UI
capsule and every other process is confined to a key handle it can never
dereference.
-/

namespace Nonos.KeyringCustody

/-- The custody store: which process owns each wallet id (`none` = no such key). -/
structure Store where
  owner : Nat → Option Nat

/-- The access decision. A caller receives the key (`some ()`) only when it is
    the recorded owner of an existing wallet; otherwise access is denied. -/
def access (st : Store) (id caller : Nat) : Option Unit :=
  match st.owner id with
  | some o => if o = caller then some () else none
  | none => none

/-- No non-owner is ever granted a key: if the wallet is owned by `o` and the
    caller is not `o`, access is denied. -/
theorem non_owner_denied (st : Store) (id caller o : Nat)
    (hown : st.owner id = some o) (hne : caller ≠ o) :
    access st id caller = none := by
  unfold access
  rw [hown]
  simp [Ne.symm hne]

/-- A key that no one owns is released to no one. -/
theorem unowned_denied (st : Store) (id caller : Nat) (hnone : st.owner id = none) :
    access st id caller = none := by
  unfold access; rw [hnone]

/-- The owner is granted its own key. -/
theorem owner_granted (st : Store) (id o : Nat) (hown : st.owner id = some o) :
    access st id o = some () := by
  unfold access; rw [hown]; simp

/-- The custody theorem: access implies ownership. Anyone the store ever hands a
    key to is exactly the owner of that wallet, so possession of a key handle by
    a non-owner can never be turned into the key itself. -/
theorem access_implies_owner (st : Store) (id caller : Nat)
    (h : access st id caller = some ()) : st.owner id = some caller := by
  unfold access at h
  split at h
  · next o heq =>
      split at h
      · next he => rw [heq, he]
      · next _ => simp at h
  · next _ => simp at h

/-- Non-interference: two callers who are both non-owners get the identical
    (denied) result, so a non-owner learns nothing about the key from the access
    interface, whatever the owner set it to. -/
theorem non_owner_indistinguishable (st : Store) (id c1 c2 o : Nat)
    (hown : st.owner id = some o) (h1 : c1 ≠ o) (h2 : c2 ≠ o) :
    access st id c1 = access st id c2 := by
  rw [non_owner_denied st id c1 o hown h1, non_owner_denied st id c2 o hown h2]

end Nonos.KeyringCustody
