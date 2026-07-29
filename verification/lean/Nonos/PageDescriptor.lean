/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The aarch64 page-table leaf encoder (`src/arch/paging/descriptor/aarch64/`).
The shared paging manager states what a mapping should be in a neutral flag
vocabulary and each architecture turns that into its own descriptor bits. On
aarch64 three of those rules run opposite to the x86_64 ones: write permission
is the absence of `AP[2]`, a block is the absence of bit 1, and execute
permission is denied by setting a bit rather than granted by clearing one. Each
is silent when got wrong, so the theorems below fix them.

The two that matter most are `kernel_leaf_never_reaches_el0` and
`user_leaf_never_executes_at_el1`. The first is the property whose polarity was
inverted in an earlier revision of this encoder, which published every kernel
page to userspace; the second is the aarch64 equivalent of SMEP. Both are
stated here for every address and every flag word, and checked against the
running code by the Kani harnesses in `userland/arch_paging_proofs`.
-/

namespace Nonos.PageDescriptor

/-- A single bit, `1 <<< i` in the code. -/
def bit (i : Nat) : Nat := 2 ^ i

/-- Test one bit of a word. -/
def has (w i : Nat) : Bool := w.testBit i

/-! ### The neutral flag vocabulary the manager passes down -/

def fPRESENT : Nat := 0
def fWRITABLE : Nat := 1
def fUSER : Nat := 2
def fNOCACHE : Nat := 4
def fHUGE : Nat := 7
def fGLOBAL : Nat := 8
def fNOEXEC : Nat := 63

/-! ### aarch64 stage 1 descriptor bit positions -/

abbrev dVALID : Nat := 0
abbrev dTABLE : Nat := 1
abbrev dAP_EL0 : Nat := 6
abbrev dAP_RO : Nat := 7
abbrev dAF : Nat := 10
abbrev dNOT_GLOBAL : Nat := 11
abbrev dPXN : Nat := 53
abbrev dUXN : Nat := 54

/-- The attribute, shareability and access-flag bits the encoder also sets.
    They occupy positions 2 to 4 and 8 to 11, none of which any theorem here
    mentions, so they are carried as an opaque word constrained to stay clear
    of the permission bits. -/
structure Attrs where
  word : Nat
  clear_valid : has word dVALID = false
  clear_table : has word dTABLE = false
  clear_el0 : has word dAP_EL0 = false
  clear_ro : has word dAP_RO = false
  clear_pxn : has word dPXN = false
  clear_uxn : has word dUXN = false

/-- Execute-never bits, exactly as `execute_never` in the encoder computes
    them. EL1 never runs a user page and EL0 never runs a kernel page, whatever
    the caller asked for; within a page's own level, `executable` decides. -/
def xn (user executable : Bool) : Nat :=
  match user, executable with
  | true, true => bit dPXN
  | true, false => bit dPXN ||| bit dUXN
  | false, true => bit dUXN
  | false, false => bit dPXN ||| bit dUXN

/-- The leaf encoder. `a` stands for the memory-type, shareability and
    not-global bits, which the permission theorems are independent of. -/
def leaf (a : Attrs) (f : Nat) : Nat :=
  let user := has f fUSER
  let wa := a.word ||| bit dAF
  let w0 := if has f fPRESENT then wa ||| bit dVALID else wa
  let w1 := if has f fHUGE then w0 else w0 ||| bit dTABLE
  let w2 := if user then w1 ||| bit dAP_EL0 else w1
  let w3 := if has f fWRITABLE then w2 else w2 ||| bit dAP_RO
  w3 ||| xn user (has f fNOEXEC = false)

/-- A table descriptor: valid, and bit 1 set so it is not read as a block. -/
def table (a : Attrs) : Nat := a.word ||| bit dVALID ||| bit dTABLE

/-! ### Decoders -/

def isPresent (w : Nat) : Bool := has w dVALID
def isBlock (w : Nat) : Bool := has w dVALID && !(has w dTABLE)
def isUser (w : Nat) : Bool := has w dAP_EL0
def isWritable (w : Nat) : Bool := !(has w dAP_RO)
def execNeverEl1 (w : Nat) : Bool := has w dPXN
def execNeverEl0 (w : Nat) : Bool := has w dUXN

/-! ### Bit lemmas -/

theorem has_or (u v i : Nat) : has (u ||| v) i = (has u i || has v i) := by
  simp [has, Nat.testBit_or]

theorem has_bit_self (i : Nat) : has (bit i) i = true := by
  simp [has, bit, Nat.testBit_two_pow_self]

theorem has_bit_other {i j : Nat} (h : j ≠ i) : has (bit j) i = false := by
  simp [has, bit, Nat.testBit_two_pow, h]

/-- Setting a bit never disturbs a different one. -/
theorem has_or_bit_other {i j : Nat} (w : Nat) (h : j ≠ i) :
    has (w ||| bit j) i = has w i := by
  rw [has_or, has_bit_other h, Bool.or_false]

/-! ### Theorems -/

/-- Presence is the caller's to decide and the encoder never forces it. An
    encoder that did would turn a deliberately absent leaf, the kind carrying
    swap metadata, into a live mapping. -/
theorem leaf_present_iff_requested (a : Attrs) (f : Nat) :
    isPresent (leaf a f) = has f fPRESENT := by
  simp only [isPresent, leaf]
  cases hp : has f fPRESENT <;> cases has f fHUGE <;> cases has f fUSER <;>
    cases has f fWRITABLE <;> cases has f fNOEXEC <;>
    simp [xn, has_or, has_bit_self, has_bit_other, a.clear_valid, dVALID, dTABLE, dAP_EL0,
      dAP_RO, dAF, dPXN, dUXN]

/-- A mapping the manager did not mark `USER` is unreachable from EL0. No other
    flag can open it. This is the property whose inversion published the whole
    kernel to userspace. -/
theorem kernel_leaf_never_reaches_el0 (a : Attrs) (f : Nat) (h : has f fUSER = false) :
    isUser (leaf a f) = false := by
  simp only [isUser, leaf, h]
  cases has f fPRESENT <;> cases has f fHUGE <;> cases has f fWRITABLE <;> cases has f fNOEXEC <;>
    simp [xn, has_or, has_bit_other, a.clear_el0, dVALID, dTABLE, dAP_EL0, dAP_RO, dAF,
      dPXN, dUXN]

/-- A mapping the manager marked `USER` is always reachable from EL0. The
    inverse failure locks userspace out of its own memory. -/
theorem user_leaf_reaches_el0 (a : Attrs) (f : Nat) (h : has f fUSER = true) :
    isUser (leaf a f) = true := by
  simp only [isUser, leaf, h]
  cases has f fPRESENT <;> cases has f fHUGE <;> cases has f fWRITABLE <;> cases has f fNOEXEC <;>
    simp [xn, has_or, has_bit_self, has_bit_other, dVALID, dTABLE, dAP_EL0, dAP_RO, dAF,
      dPXN, dUXN]

/-- A mapping the manager did not mark `WRITABLE` never is. W^X for code pages
    rests on this holding whatever else was asked for. -/
theorem read_only_leaf_never_writable (a : Attrs) (f : Nat) (h : has f fWRITABLE = false) :
    isWritable (leaf a f) = false := by
  simp only [isWritable, leaf, h]
  cases has f fPRESENT <;> cases has f fHUGE <;> cases has f fUSER <;> cases has f fNOEXEC <;>
    simp [xn, has_or, has_bit_self, has_bit_other, dVALID, dTABLE, dAP_EL0, dAP_RO, dAF,
      dPXN, dUXN]

/-- EL1 never executes a user page: the aarch64 equivalent of SMEP, and true
    for every flag word including one that asks for an executable user page. -/
theorem user_leaf_never_executes_at_el1 (a : Attrs) (f : Nat) (h : has f fUSER = true) :
    execNeverEl1 (leaf a f) = true := by
  simp only [execNeverEl1, leaf, h]
  cases has f fPRESENT <;> cases has f fHUGE <;> cases has f fWRITABLE <;> cases has f fNOEXEC <;>
    simp [xn, has_or, has_bit_self, has_bit_other, dVALID, dTABLE, dAP_EL0, dAP_RO, dAF,
      dPXN, dUXN]

/-- EL0 never executes a kernel page. -/
theorem kernel_leaf_never_executes_at_el0 (a : Attrs) (f : Nat) (h : has f fUSER = false) :
    execNeverEl0 (leaf a f) = true := by
  simp only [execNeverEl0, leaf, h]
  cases has f fPRESENT <;> cases has f fHUGE <;> cases has f fWRITABLE <;> cases has f fNOEXEC <;>
    simp [xn, has_or, has_bit_self, has_bit_other, dVALID, dTABLE, dAP_EL0, dAP_RO, dAF,
      dPXN, dUXN]

/-- A table descriptor is present and never reads as a block. The walk decides
    whether to descend on exactly this question, so a table that reads as a
    block ends the walk at a page table and returns its bytes as data. -/
theorem table_is_present_and_not_a_block (a : Attrs) :
    isPresent (table a) = true ∧ isBlock (table a) = false := by
  constructor <;>
    simp [isPresent, isBlock, table, has_or, has_bit_self, has_bit_other, a.clear_valid,
      a.clear_table, dVALID, dTABLE]

end Nonos.PageDescriptor
