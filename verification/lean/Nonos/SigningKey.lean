/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Signing-key trust: the anti-rollback floor, key epochs, and revocation. A key is
accepted only when its rollback index meets the enforced floor; the floor only
rises; a rotation supersedes the prior epoch; and a revoked key is refused
regardless of its index. The theorems below show a downgrade is impossible, the
floor is monotone, acceptance is upward-closed in the index, and revocation
dominates, so an old or pulled key can never sign an image the boot chain trusts.
-/

namespace Nonos.SigningKey

/-- A signing key: its generation epoch and its rollback index. -/
structure Key where
  epoch : Nat
  rollback : Nat

/-- A key is accepted when its rollback index meets the enforced floor. -/
def accepts (floor : Nat) (k : Key) : Prop := floor ≤ k.rollback

/-- A key exactly at the floor is accepted. -/
theorem accepts_at_floor (floor e : Nat) : accepts floor ⟨e, floor⟩ := by
  simp [accepts]

/-- A key above the floor is accepted. -/
theorem accepts_above_floor (floor e r : Nat) (h : floor ≤ r) : accepts floor ⟨e, r⟩ := h

/-- A key below the floor is refused: a downgrade cannot sign. -/
theorem below_floor_refused (floor e r : Nat) (h : r < floor) : ¬ accepts floor ⟨e, r⟩ := by
  simp only [accepts]; omega

/-- Acceptance is upward-closed in the rollback index. -/
theorem accepts_monotone_index (floor e r r' : Nat) (h : accepts floor ⟨e, r⟩) (hr : r ≤ r') :
    accepts floor ⟨e, r'⟩ := by
  simp only [accepts] at *; omega

/-- Raising the floor never newly accepts a key. -/
theorem raising_floor_never_admits (floor floor' : Nat) (k : Key)
    (hf : floor ≤ floor') (h : ¬ accepts floor k) : ¬ accepts floor' k := by
  simp only [accepts] at *; omega

/-- If a key is refused at a floor, it is refused at every higher floor. -/
theorem refusal_upward (floor floor' : Nat) (k : Key) (hf : floor ≤ floor')
    (h : ¬ accepts floor k) : ¬ accepts floor' k :=
  raising_floor_never_admits floor floor' k hf h

/-- Acceptance at a higher floor implies acceptance at a lower one. -/
theorem accepts_downward_floor (floor floor' : Nat) (k : Key) (hf : floor ≤ floor')
    (h : accepts floor' k) : accepts floor k := by
  simp only [accepts] at *; omega

/-- The floor only ever rises across an update. -/
def advance (floor new : Nat) : Nat := max floor new

/-- Advancing never lowers the floor. -/
theorem advance_monotone (floor new : Nat) : floor ≤ advance floor new := by
  simp only [advance]; omega

/-- Advancing takes at least the requested new floor. -/
theorem advance_at_least_new (floor new : Nat) : new ≤ advance floor new := by
  simp only [advance]; omega

/-- Advancing twice is advancing by the larger. -/
theorem advance_advance (floor a b : Nat) : advance (advance floor a) b = advance floor (max a b) := by
  simp [advance, Nat.max_assoc]

/-- Advancing by a floor already met is a no-op. -/
theorem advance_idem (floor new : Nat) (h : new ≤ floor) : advance floor new = floor := by
  simp only [advance]; omega

/-- After advancing to a new floor, a key below it is refused. -/
theorem advance_refuses_stale (floor new e r : Nat) (h : r < new) :
    ¬ accepts (advance floor new) ⟨e, r⟩ := by
  simp only [accepts, advance]; omega

/-- A rotation supersedes the prior epoch. -/
def rotate (k : Key) : Key := ⟨k.epoch + 1, k.rollback⟩

/-- Rotation strictly advances the epoch. -/
theorem rotate_advances_epoch (k : Key) : k.epoch < (rotate k).epoch := by
  simp [rotate]

/-- Rotation preserves the rollback index. -/
theorem rotate_keeps_index (k : Key) : (rotate k).rollback = k.rollback := rfl

/-- Rotation does not change acceptance under a fixed floor. -/
theorem rotate_preserves_acceptance (floor : Nat) (k : Key) :
    accepts floor (rotate k) ↔ accepts floor k := by
  simp [accepts, rotate]

/-- A revocation set: a key is revoked when its epoch is in the set. -/
def revoked (set : List Nat) (k : Key) : Prop := k.epoch ∈ set

/-- Trust requires acceptance and non-revocation. -/
def trusted (floor : Nat) (set : List Nat) (k : Key) : Prop :=
  accepts floor k ∧ ¬ revoked set k

/-- A revoked key is never trusted, whatever its index. -/
theorem revoked_never_trusted (floor : Nat) (set : List Nat) (k : Key)
    (h : revoked set k) : ¬ trusted floor set k := by
  intro ht; exact ht.2 h

/-- Trust implies acceptance. -/
theorem trusted_accepts (floor : Nat) (set : List Nat) (k : Key)
    (h : trusted floor set k) : accepts floor k := h.1

/-- Trust implies non-revocation. -/
theorem trusted_not_revoked (floor : Nat) (set : List Nat) (k : Key)
    (h : trusted floor set k) : ¬ revoked set k := h.2

/-- Adding an epoch to the revocation set revokes that key. -/
theorem adding_revokes (set : List Nat) (k : Key) : revoked (k.epoch :: set) k := by
  simp [revoked]

/-- A key not in the revocation set is not revoked. -/
theorem absent_not_revoked (set : List Nat) (k : Key) (h : k.epoch ∉ set) :
    ¬ revoked set k := h

/-- Revoking one epoch never un-revokes another already revoked. -/
theorem revocation_monotone (set : List Nat) (e : Nat) (k : Key)
    (h : revoked set k) : revoked (e :: set) k := by
  simp only [revoked] at *; exact List.mem_cons_of_mem e h

/-- Below the floor and revoked: refused on both counts. -/
theorem stale_and_revoked (floor : Nat) (set : List Nat) (e r : Nat)
    (_hr : r < floor) (hrev : e ∈ set) : ¬ trusted floor set ⟨e, r⟩ := by
  intro ht; exact ht.2 hrev

/-- Trust at a higher floor implies acceptance at a lower one, revocation unchanged. -/
theorem trusted_downward_floor (floor floor' : Nat) (set : List Nat) (k : Key)
    (hf : floor ≤ floor') (h : trusted floor' set k) : trusted floor set k :=
  ⟨accepts_downward_floor floor floor' k hf h.1, h.2⟩

end Nonos.SigningKey
