/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Consistency of the committed Merkle root. Two openings that agree with the same
enrolled root must have compressed the same node, so a verifier that has pinned a
root cannot be shown two different subtrees under it. The theorems below show
compression is a function of its inputs and, under an injective compression, equal
roots force equal children and distinct children reach distinct roots: the
collision-freedom the whole membership argument rests on.
-/

namespace Nonos.Stark.RootConsistency

/-- One two-to-one compression. -/
def compress (comb : Nat → Nat → Nat) (l r : Nat) : Nat := comb l r

/-- Injectivity of the compression in both children. -/
def CombInjective (comb : Nat → Nat → Nat) : Prop :=
  ∀ a b c d, comb a b = comb c d → a = c ∧ b = d

/-- Compression is a function of its children: same inputs, same root. -/
theorem compress_deterministic (comb : Nat → Nat → Nat) (l r x y : Nat)
    (h₁ : compress comb l r = x) (h₂ : compress comb l r = y) : x = y := by
  rw [← h₁, ← h₂]

/-- Equal roots force equal left children under injectivity. -/
theorem equal_root_equal_left (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (l r l' r' : Nat) (h : compress comb l r = compress comb l' r') : l = l' :=
  (hc _ _ _ _ h).1

/-- Equal roots force equal right children under injectivity. -/
theorem equal_root_equal_right (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (l r l' r' : Nat) (h : compress comb l r = compress comb l' r') : r = r' :=
  (hc _ _ _ _ h).2

/-- A different left child reaches a different root. -/
theorem distinct_left_distinct_root (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (l l' r : Nat) (hne : l ≠ l') : compress comb l r ≠ compress comb l' r := by
  intro h; exact hne (equal_root_equal_left comb hc l r l' r h)

/-- A different right child reaches a different root. -/
theorem distinct_right_distinct_root (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (l r r' : Nat) (hne : r ≠ r') : compress comb l r ≠ compress comb l r' := by
  intro h; exact hne (equal_root_equal_right comb hc l r l r' h)

/-- A verifier pinned to a root sees at most one child pair under injectivity. -/
theorem root_pins_children (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (l r l' r' root : Nat) (h₁ : compress comb l r = root) (h₂ : compress comb l' r' = root) :
    l = l' ∧ r = r' := by
  apply hc
  simp only [compress] at h₁ h₂
  rw [h₁, h₂]

/-- No second preimage: a distinct child pair cannot share the root. -/
theorem no_second_preimage (comb : Nat → Nat → Nat) (hc : CombInjective comb)
    (l r l' r' : Nat) (hne : l ≠ l' ∨ r ≠ r') :
    compress comb l r ≠ compress comb l' r' := by
  intro h
  obtain ⟨hl, hr⟩ := hc _ _ _ _ h
  cases hne with
  | inl hnl => exact hnl hl
  | inr hnr => exact hnr hr

end Nonos.Stark.RootConsistency
