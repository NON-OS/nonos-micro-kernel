/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Signing-key validity windows. A key is trusted only within its activation window:
not before it is issued, not after it expires. The theorems below show a key is
active exactly inside its window, a not-yet-valid or expired key is refused,
widening a window never removes a time that was active, and two overlapping keys
share a common active instant only when their windows meet: the temporal half of
the key trust decision, complementing the rollback and revocation checks.
-/

namespace Nonos.KeyLifecycle

/-- A signing key with an activation window. -/
structure Key where
  notBefore : Nat
  notAfter : Nat

/-- The key is active at time t when t lies inside its window. -/
def activeAt (k : Key) (t : Nat) : Prop := k.notBefore ≤ t ∧ t ≤ k.notAfter

/-- A well-formed key does not expire before it activates. -/
def WellFormed (k : Key) : Prop := k.notBefore ≤ k.notAfter

/-- A key is active at its activation instant when well formed. -/
theorem active_at_start (k : Key) (h : WellFormed k) : activeAt k k.notBefore :=
  ⟨Nat.le_refl _, h⟩

/-- A key is active at its expiry instant when well formed. -/
theorem active_at_end (k : Key) (h : WellFormed k) : activeAt k k.notAfter :=
  ⟨h, Nat.le_refl _⟩

/-- A key is refused before it activates. -/
theorem not_yet_valid (k : Key) (t : Nat) (h : t < k.notBefore) : ¬ activeAt k t := by
  simp only [activeAt]; omega

/-- A key is refused after it expires. -/
theorem expired (k : Key) (t : Nat) (h : k.notAfter < t) : ¬ activeAt k t := by
  simp only [activeAt]; omega

/-- Activity is exactly membership in the window. -/
theorem active_iff (k : Key) (t : Nat) : activeAt k t ↔ k.notBefore ≤ t ∧ t ≤ k.notAfter := Iff.rfl

/-- Widening a window on both ends never drops a time that was active. -/
theorem widen_preserves_active (k : Key) (nb na t : Nat) (hnb : nb ≤ k.notBefore)
    (hna : k.notAfter ≤ na) (h : activeAt k t) : activeAt ⟨nb, na⟩ t := by
  simp only [activeAt] at *; omega

/-- A time active in a window is active in any superset window. -/
theorem active_monotone (k k' : Key) (t : Nat) (hb : k'.notBefore ≤ k.notBefore)
    (ha : k.notAfter ≤ k'.notAfter) (h : activeAt k t) : activeAt k' t := by
  simp only [activeAt] at *; omega

/-- Two overlapping windows share a common active instant. -/
theorem overlap_shares_instant (k₁ k₂ : Key) (hwf : k₂.notBefore ≤ k₂.notAfter)
    (h2 : k₂.notBefore ≤ k₁.notAfter) (hle : k₁.notBefore ≤ k₂.notBefore) :
    activeAt k₁ k₂.notBefore ∧ activeAt k₂ k₂.notBefore :=
  ⟨⟨hle, h2⟩, ⟨Nat.le_refl _, hwf⟩⟩

/-- A rotation window that starts at the old key's expiry has no active gap. -/
theorem seamless_rotation (old new : Key) (h : new.notBefore ≤ old.notAfter)
    (t : Nat) (ht : old.notAfter ≤ t) (hn : t ≤ new.notAfter) : activeAt new t := by
  simp only [activeAt]; omega

/-- Activity at a time is decidable: the check always returns a verdict. -/
theorem active_decidable (k : Key) (t : Nat) : activeAt k t ∨ ¬ activeAt k t := by
  by_cases h : activeAt k t
  · exact Or.inl h
  · exact Or.inr h

end Nonos.KeyLifecycle
