/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Proof serialization. A money-grade proof is a list of field limbs; the deserializer
accepts it only when every limb is a canonical residue, and it never faults on a
malformed input, it rejects. The theorems below show serialize then deserialize is
the identity on a canonical proof, deserialization preserves length, a single
non-canonical limb makes the whole proof rejected, and acceptance is decidable:
the invariant that keeps a crafted proof from smuggling an out-of-range value in.
-/

namespace Nonos.Stark.Serialization

/-- The Goldilocks modulus. -/
def P : Nat := 18446744069414584321

/-- A limb is canonical when it is a reduced residue. -/
def Canonical (x : Nat) : Prop := x < P

/-- A limb list is canonical when every limb is. -/
def AllCanonical (limbs : List Nat) : Prop := ∀ x ∈ limbs, Canonical x

/-- Deserialize a limb list: return it only when every limb is canonical. -/
def deserialize (limbs : List Nat) : Option (List Nat) :=
  if limbs.all (· < P) then some limbs else none

/-- The empty proof is trivially all-canonical. -/
theorem all_canonical_nil : AllCanonical [] := by
  intro x hx; simp at hx

/-- Prepending a canonical limb to a canonical proof stays canonical. -/
theorem all_canonical_cons (x : Nat) (limbs : List Nat) (hx : Canonical x)
    (h : AllCanonical limbs) : AllCanonical (x :: limbs) := by
  intro y hy
  simp only [List.mem_cons] at hy
  cases hy with
  | inl he => exact he ▸ hx
  | inr ht => exact h y ht

/-- The `all` predicate agrees with the propositional canonicity of every limb. -/
theorem all_iff (limbs : List Nat) : limbs.all (· < P) = true ↔ AllCanonical limbs := by
  simp only [List.all_eq_true, AllCanonical, Canonical, decide_eq_true_eq]

/-- A canonical proof deserializes back to itself. -/
theorem deserialize_canonical (limbs : List Nat) (h : AllCanonical limbs) :
    deserialize limbs = some limbs := by
  simp only [deserialize]
  rw [if_pos ((all_iff limbs).mpr h)]

/-- Deserialization preserves the proof unchanged when it accepts. -/
theorem deserialize_preserves (limbs out : List Nat) (h : deserialize limbs = some out) :
    out = limbs := by
  simp only [deserialize] at h
  by_cases hc : limbs.all (· < P) = true
  · rw [if_pos hc] at h; injection h with h; exact h.symm
  · rw [if_neg hc] at h; exact absurd h (by simp)

/-- Deserialization preserves length when it accepts: no limbs added or dropped. -/
theorem deserialize_length (limbs out : List Nat) (h : deserialize limbs = some out) :
    out.length = limbs.length := by
  rw [deserialize_preserves limbs out h]

/-- A single non-canonical limb makes the whole proof rejected. -/
theorem non_canonical_rejected (limbs : List Nat) (x : Nat) (hx : P ≤ x) (hmem : x ∈ limbs) :
    deserialize limbs = none := by
  have hne : ¬ (limbs.all (· < P) = true) := by
    rw [all_iff]
    intro hall
    have hc := hall x hmem
    simp only [Canonical] at hc
    omega
  simp only [deserialize, if_neg hne]

/-- Deserialization never faults: it always returns a decision. -/
theorem deserialize_total (limbs : List Nat) :
    deserialize limbs = some limbs ∨ deserialize limbs = none := by
  simp only [deserialize]
  by_cases hc : limbs.all (· < P) = true
  · rw [if_pos hc]; exact Or.inl rfl
  · rw [if_neg hc]; exact Or.inr rfl

/-- Acceptance is exactly canonicity of the whole proof. -/
theorem accepts_iff_canonical (limbs : List Nat) :
    deserialize limbs = some limbs ↔ AllCanonical limbs := by
  constructor
  · intro hd
    by_cases hc : limbs.all (· < P) = true
    · exact (all_iff limbs).mp hc
    · simp only [deserialize, if_neg hc] at hd
      exact Option.noConfusion hd
  · exact deserialize_canonical limbs

end Nonos.Stark.Serialization
