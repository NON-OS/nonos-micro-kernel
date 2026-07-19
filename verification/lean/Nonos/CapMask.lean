/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The capability bitmask operations (`src/capabilities/bits.rs`). A capability is
a single bit; `has_capability` tests it, `add_capability` sets it with a bitwise
or, and a mask is a subset of another when it sets no bit the other leaves
clear. The theorems below fix the algebra the whole capability system leans on:
adding a capability grants exactly that one and disturbs no other, adding is
monotone so a grant never revokes, and the delegation-safety property, that a
subset mask never carries a capability its parent lacks. This is what makes a
delegated token strictly weaker than the authority it came from.
-/

namespace Nonos.CapMask

/-- A capability's bit, `1 << index` in the code. -/
def bit (c : Nat) : Nat := 2 ^ c

/-- Test a capability, `bits & cap.bit() != 0`. -/
def has (bits c : Nat) : Bool := bits.testBit c

/-- Grant a capability, `bits | cap.bit()`. -/
def add (bits c : Nat) : Nat := bits ||| bit c

/-- One mask is a subset of another when every capability it holds the other
    holds too: the delegation admissibility relation. -/
def Subset (child parent : Nat) : Prop := ∀ c, has child c = true → has parent c = true

/-- Adding a capability grants exactly that capability. -/
theorem has_add_self (bits c : Nat) : has (add bits c) c = true := by
  simp [has, add, bit, Nat.testBit_or, Nat.testBit_two_pow_self]

/-- Adding one capability never changes whether another is held: no aliasing
    between distinct capability bits. -/
theorem has_add_other (bits c d : Nat) (h : d ≠ c) : has (add bits d) c = has bits c := by
  simp only [has, add, bit, Nat.testBit_or]
  have : (2 ^ d).testBit c = false := by
    simp [Nat.testBit_two_pow, h]
  rw [this, Bool.or_false]

/-- Granting is monotone: a capability already held stays held after any grant,
    so a grant never revokes. -/
theorem add_monotone (bits c d : Nat) (h : has bits d = true) : has (add bits c) d = true := by
  simp only [has, add, bit, Nat.testBit_or]
  rw [show bits.testBit d = true from h, Bool.true_or]

/-- Every mask is a subset of itself. -/
theorem subset_refl (bits : Nat) : Subset bits bits := fun _ h => h

/-- The delegation-safety property: a subset mask holds no capability its parent
    lacks. If the parent does not grant a capability, no child delegated from it
    can either. -/
theorem subset_no_extra (child parent c : Nat) (hs : Subset child parent)
    (h : has parent c = false) : has child c = false := by
  cases hcc : has child c with
  | false => rfl
  | true =>
    rw [hs c hcc] at h
    exact Bool.noConfusion h

/-- Subset is transitive: a delegation chain stays within the original
    authority, so authority only ever narrows down a chain of grants. -/
theorem subset_trans (a b c : Nat) (hab : Subset a b) (hbc : Subset b c) : Subset a c :=
  fun x hx => hbc x (hab x hx)

/-- Granting the parent a capability keeps the subset relation: widening the
    parent never breaks a child that was already within it. -/
theorem subset_add_parent (child parent c : Nat) (hs : Subset child parent) :
    Subset child (add parent c) := by
  intro d hd
  exact add_monotone parent c d (hs d hd)

end Nonos.CapMask
