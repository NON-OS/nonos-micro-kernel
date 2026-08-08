/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The station address rule (`userland/nonos_mac/`).

A factory Ethernet address is burned into one piece of silicon and is the first
field every access point on the way records. A system that keeps nothing on
disk still arrives everywhere carrying the same name, so being amnesic is not
the same as being unlinkable.

The rule that fixes it touches two bits of the first octet and nothing else:
clear the group bit so the address names one station, set the locally
administered bit so it does not claim a range the IEEE gave to a manufacturer.
The theorems below are about those two bits and about the forty six that must
survive untouched, because an implementation that quietly normalised more of
the address would shrink the space it is drawn from, and a smaller space is
easier to correlate across networks.
-/

namespace Nonos.StationAddress

/-- Bit 0 of the first octet. Set means the address names a group, so a station
    must never transmit from one. -/
abbrev group : Nat := 0

/-- Bit 1 of the first octet. Set means the address was not assigned by the
    IEEE to a vendor. -/
abbrev localBit : Nat := 1

/-- The first octet after the rule is applied: local set, group cleared.
    Mirrors `mac[0] = (mac[0] ||| 0x02) &&& !0x01` in the encoder. -/
def leadOctet (b : Nat) : Nat := (b ||| 2) &&& (Nat.pow 2 8 - 1 - 1)

/-- Test one bit. -/
def has (w i : Nat) : Bool := w.testBit i

/-! ### The two bits the rule fixes -/

/-- The result never names a group, whatever arrived. A station transmitting
    from a group address is malformed, and the input may well be one: the rule
    is applied to whatever the entropy source produced. -/
theorem never_group (b : Nat) : has (leadOctet b) group = false := by
  simp [has, leadOctet, group, Nat.testBit_and, Nat.testBit_or]

/-- The result is always locally administered, so it never claims a vendor's
    registered range. This is the property the privacy claim rests on. -/
theorem always_local (b : Nat) : has (leadOctet b) localBit = true := by
  simp [has, leadOctet, localBit, Nat.testBit_and, Nat.testBit_or]
  exact ⟨Or.inr (by decide), by decide⟩

/-! ### What the rule must leave alone

    That the other forty six bits survive untouched is checked by the Kani
    harnesses in `userland/nonos_mac/src/proofs.rs`, over every octet of every
    input, against the function the drivers actually call. It is left there
    rather than restated here: it is a statement about masking, and a bounded
    model checker settles those exhaustively where core Lean without Mathlib
    would need a hand-rolled bitvector theory to say the same thing less
    convincingly. The same applies to the rule being idempotent. -/

/-! ### Consequences -/

/-- A factory address has the local bit clear by definition, so no output of
    the rule can equal one. The two ranges are disjoint, which is what makes
    this a replacement rather than a reshuffle. -/
theorem never_a_factory_address (b f : Nat) (hf : has f localBit = false) :
    leadOctet b ≠ f := by
  intro h
  rw [← h, always_local] at hf
  exact Bool.noConfusion hf

/-- Broadcast is every bit set, which includes the group bit, so the result is
    never broadcast. -/
theorem never_broadcast (b : Nat) : leadOctet b ≠ 255 := by
  intro h
  have : has (leadOctet b) group = true := by rw [h]; simp [has, group]
  rw [never_group] at this
  exact Bool.noConfusion this

/-- The result is never zero either, which some drivers read as "no address
    assigned" and then paper over with the factory one. -/
theorem never_zero (b : Nat) : leadOctet b ≠ 0 := by
  intro h
  have : has (leadOctet b) localBit = false := by rw [h]; simp [has, localBit]
  rw [always_local] at this
  exact Bool.noConfusion this

end Nonos.StationAddress
