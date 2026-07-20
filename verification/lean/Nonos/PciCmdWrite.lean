/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The PCI Command-register write allowlist
(`src/hardware/broker/pci/allowlist.rs::validate_command`). A driver capsule may
flip only the Bus Master and Memory Space command bits; every other bit (I/O
space, interrupt disable, SERR, and the rest) is off limits. A write whose new
value touches only writable bits is merged into the current register; any write
is admitted only if the result differs from the current register on no
non-writable bit. The theorems below fix the confinement: an admitted write
changes only writable bits, the merge branch is always confined and never clears
a bit it does not touch, and a raw write that would flip a protected bit is
refused. This is what keeps a capsule from reprogramming a device's decode or
interrupt behaviour behind the broker's back.
-/

namespace Nonos.PciCmdWrite

/-- A command register as its set bits: `bit b` is set when the function is
    true. The Command register is 16 bits in hardware; the model is bit-indexed
    so the writable-bit reasoning is exact. -/
abbrev Bits := Nat → Bool

/-- One register is within another set of bits when it sets none the other
    leaves clear: `value & !mask == 0`. -/
def Sub (x mask : Bits) : Prop := ∀ b, x b = true → mask b = true

/-- Bitwise or, `current | new`. -/
def union (x y : Bits) : Bits := fun b => x b || y b

/-- The value the validator intends to write: if the new value only sets
    writable bits it is merged into the current register, else it is taken as is.
    Mirrors `if new & !WRITABLE == 0 { current | new } else { new }`. -/
def desired (new current : Bits) (newSub : Bool) : Bits :=
  fun b => if newSub = true then current b || new b else new b

/-- The admission test: the intended value differs from the current register on
    no non-writable bit. Mirrors `(desired ^ current) & !WRITABLE == 0`. -/
def AgreesOutside (x current writable : Bits) : Prop :=
  ∀ b, writable b = false → x b = current b

/-- The confinement guarantee: if a write is admitted, it changes only writable
    bits. A driver capsule can never flip a command bit outside the allowlist. -/
theorem admitted_changes_only_writable (new current writable : Bits) (newSub : Bool)
    (h : AgreesOutside (desired new current newSub) current writable)
    (b : Nat) (hchg : desired new current newSub b ≠ current b) : writable b = true := by
  by_cases hw : writable b = true
  · exact hw
  · have hwf : writable b = false := by
      cases hb : writable b with
      | true => exact absurd hb hw
      | false => rfl
    exact absurd (h b hwf) hchg

/-- The merge branch is always confined: when the new value sets only writable
    bits, merging it into the current register changes nothing outside the
    writable set, so a legitimate write is always admitted. -/
theorem merge_branch_confined (new current writable : Bits)
    (hsub : Sub new writable) :
    AgreesOutside (desired new current true) current writable := by
  intro b hwf
  have hd : desired new current true b = (current b || new b) := by simp [desired]
  rw [hd]
  cases hn : new b with
  | true => exact absurd (hsub b hn) (by rw [hwf]; simp)
  | false => rw [Bool.or_false]

/-- The merge never clears a bit that was set: bus-master and memory-space can be
    asserted, but no currently-set bit is dropped by a merge. -/
theorem merge_never_clears (new current : Bits) (b : Nat) (h : current b = true) :
    desired new current true b = true := by
  simp [desired, h]

/-- A raw write (not a submask merge) that would flip a protected bit is refused:
    it fails the admission test, so it never reaches the bus. -/
theorem raw_protected_write_refused (new current writable : Bits)
    (b : Nat) (hwf : writable b = false) (hdiff : new b ≠ current b) :
    ¬ AgreesOutside (desired new current false) current writable := by
  intro h
  have hval : desired new current false b = current b := h b hwf
  simp [desired] at hval
  exact hdiff hval

end Nonos.PciCmdWrite
