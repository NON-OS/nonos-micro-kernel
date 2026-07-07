/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Word-level refinement of the capability model. `Capability.lean` proves the
lattice properties of an abstract capability set; the kernel stores a token as
a 64-bit word and implements the operations as `bits & mask`, `bits | bit`,
and `bits & !bit` in `src/capabilities/bits.rs`. This module closes the gap
between the two by proof rather than by naming: `capsOf` maps a word to the
capability set it denotes, and each theorem shows a kernel bit operation
denotes exactly the corresponding abstract operation, for every word and every
capability id. The Verus theorems in `verification/verus/src/capabilities.rs`
check the same operations on the `u64` type the kernel executes, so the chain
is: abstract lattice (Lean), word-level denotation (this file), machine word
(Verus and the kernel_proofs differential harnesses).

Axioms: these theorems depend on propext, Quot.sound, and, through the core
library's own proof of `Nat.testBit_two_pow_sub_one`, Classical.choice. All
three are Lean's standard axioms; there is no sorry anywhere.
-/

import Nonos.Capability

namespace Nonos.CapabilityBits

open Nonos.Capability

/-- The capability set a word denotes: bit `i` grants capability `i`. -/
def capsOf (w : Nat) : Caps := fun i => w.testBit i

/-- The kernel's 64-bit all-ones word, `!0u64`. -/
def mask64 : Nat := 2 ^ 64 - 1

/-- The kernel's `!bit` on a 64-bit word: all-ones XOR the single bit. -/
def not64 (w : Nat) : Nat := mask64 ^^^ w

/-- A single capability bit, `1 << b`. -/
def bitOf (b : Nat) : Nat := 1 <<< b

private theorem testBit_bitOf (b i : Nat) : (bitOf b).testBit i = (i == b) := by
  unfold bitOf
  rw [Nat.shiftLeft_eq, Nat.one_mul]
  by_cases h : i = b
  · subst h
    simp [Nat.testBit_two_pow_self]
  · simp [Nat.testBit_two_pow_of_ne (fun hb => h hb.symm), h]

private theorem testBit_mask64 (i : Nat) (h : i < 64) : mask64.testBit i = true := by
  unfold mask64
  rw [Nat.testBit_two_pow_sub_one]
  simp [h]

/-- `bits & mask` denotes exactly the abstract attenuation: the meet proven to
    be the greatest lower bound in `Capability.lean`. -/
theorem land_denotes_attenuate (a m x : Nat) :
    Grants (capsOf (a &&& m)) x ↔ Grants (attenuate (capsOf a) (capsOf m)) x := by
  unfold Grants capsOf attenuate
  simp [Nat.testBit_and]

/-- `bits | (1 << b)` denotes exactly the abstract grant: the join proven to be
    the least upper bound in `Capability.lean`. -/
theorem lor_denotes_grant (a b x : Nat) :
    Grants (capsOf (a ||| bitOf b)) x ↔ Grants (grant (capsOf a) b) x := by
  unfold Grants capsOf grant
  simp [Nat.testBit_or, testBit_bitOf]

/-- `bits & !(1 << b)` denotes exactly the abstract revocation, for every
    64-bit word and capability id in range. The range hypotheses are the
    kernel's own: a token is a `u64` and a capability id indexes one of its
    bits. -/
theorem land_not_denotes_revoke (a b x : Nat)
    (ha : a < 2 ^ 64) (_hb : b < 64) :
    Grants (capsOf (a &&& not64 (bitOf b))) x ↔ Grants (revoke (capsOf a) b) x := by
  unfold Grants capsOf revoke not64
  by_cases hx : x < 64
  · -- In range, the complement flips exactly the named bit.
    have hm : mask64.testBit x = true := testBit_mask64 x hx
    simp [Nat.testBit_and, Nat.testBit_xor, hm, testBit_bitOf, bne]
  · -- Above the word width both sides are false: a u64 has no such bit.
    have hge : 64 ≤ x := Nat.le_of_not_lt hx
    have hlt : a < 2 ^ x := Nat.lt_of_lt_of_le ha (Nat.pow_le_pow_right (by omega) hge)
    have hax : a.testBit x = false := Nat.testBit_lt_two_pow hlt
    simp [Nat.testBit_and, hax]

/-- Transfer: the word-level attenuation confines, directly from the
    denotation and the abstract theorem. A `bits & mask` token can never grant
    a capability the original word lacked. -/
theorem word_attenuation_confines (a m x : Nat)
    (h : Grants (capsOf (a &&& m)) x) : Grants (capsOf a) x :=
  attenuate_confines (capsOf a) (capsOf m) x ((land_denotes_attenuate a m x).mp h)

/-- Transfer: a chain of word-level attenuations, of any length, never widens
    authority. This is `chain_never_widens` landed on the kernel's operation. -/
theorem word_chain_never_widens (a : Nat) (ms : List Nat) (x : Nat)
    (h : Grants (capsOf (ms.foldl (· &&& ·) a)) x) : Grants (capsOf a) x := by
  induction ms generalizing a with
  | nil => exact h
  | cons m ms ih => exact word_attenuation_confines a m x (ih (a &&& m) h)

end Nonos.CapabilityBits
