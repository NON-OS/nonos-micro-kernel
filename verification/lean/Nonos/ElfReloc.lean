/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The relocation write-size gate (`src/elf/reloc/apply/bounds.rs::ensure_target_range`).
Each supported x86_64 relocation type writes a fixed number of bytes (0, 1, 2, 4
or 8); an unknown type is rejected outright, and a write of non-zero size is
admitted only when it lands wholly inside a segment. The theorems below fix that
the write width of any supported relocation is at most eight bytes, that an
unsupported type is always refused, and that an admitted relocation either writes
nothing or writes in-segment, so applying relocations can never poke a byte
outside the image.
-/

namespace Nonos.ElfReloc

/-- The x86_64 relocation types the applier knows, plus a catch-all for any
    other value (`other` in the match). -/
inductive Reloc where
  | none | copy
  | r64 | globDat | jumpSlot | relative | irelative
  | pc32 | got32 | plt32 | gotpcrel | r32 | r32s
  | r16 | pc16
  | r8 | pc8
  | other (v : Nat)
  deriving DecidableEq

/-- The byte width a relocation type writes, or `none` if unsupported. Mirrors
    the `match reloc_type` size table. -/
def writeSize : Reloc → Option Nat
  | .none | .copy => some 0
  | .r64 | .globDat | .jumpSlot | .relative | .irelative => some 8
  | .pc32 | .got32 | .plt32 | .gotpcrel | .r32 | .r32s => some 4
  | .r16 | .pc16 => some 2
  | .r8 | .pc8 => some 1
  | .other _ => none

/-- The outcome of `ensure_target_range`. -/
inductive Verdict where
  | ok
  | unsupported
  | rangeFail
  deriving DecidableEq

/-- `ensure_target_range`, with the in-segment check abstracted to its boolean
    outcome: a zero-size write is always fine, a sized write must be in-segment,
    an unknown type is refused. -/
def ensure (t : Reloc) (inSegment : Bool) : Verdict :=
  match writeSize t with
  | Option.none => .unsupported
  | some s => if s = 0 ∨ inSegment = true then .ok else .rangeFail

/-- Every supported relocation writes at most eight bytes: the write width is
    bounded regardless of the relocation. -/
theorem writeSize_le_8 (t : Reloc) (s : Nat) (h : writeSize t = some s) : s ≤ 8 := by
  cases t <;> simp [writeSize] at h <;> omega

/-- An unsupported relocation type is always refused. -/
theorem other_unsupported (v : Nat) (inSegment : Bool) :
    ensure (.other v) inSegment = .unsupported := by
  simp [ensure, writeSize]

/-- An admitted relocation either writes nothing or writes in-segment: a
    relocation apply never touches a byte outside a segment. -/
theorem admitted_no_oob (t : Reloc) (inSegment : Bool) (h : ensure t inSegment = .ok) :
    writeSize t = some 0 ∨ inSegment = true := by
  by_cases hin : inSegment = true
  · exact Or.inr hin
  · left
    have hinf : inSegment = false := by
      cases hb : inSegment with
      | true => exact absurd hb hin
      | false => rfl
    cases hw : writeSize t with
    | none => simp_all [ensure]
    | some s =>
      by_cases hs : s = 0
      · simp [hw, hs]
      · simp_all [ensure]

end Nonos.ElfReloc
