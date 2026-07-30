/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

The flags a saved context is allowed to resume with.

A context is restored from memory the process it belongs to could have
influenced, so its RFLAGS is input, not state. `sanitize` in
`src/arch/x86_64/context/rflags.rs` clears the bits ring 0 owns and restores
the reserved one; `sanitize_user` is the same with IF set, for the resume that
lands in user mode. Both restore paths call it.

IOPL is why this matters. A context resuming with IOPL 3 has unrestricted port
I/O: it can reach every device on the machine, and nothing downstream observes
that it happened.

The claim splits in two, and this file carries the half Lean is good at. Here:
the mask constant is exactly the bit positions it is meant to stand for, which
is what a typo in the hex literal breaks. In `userland/mechanism_proofs`: the
real Rust, included by path, agrees with a spec restated from these same
positions, checked by Kani over every 64-bit input and by `cargo test` over
the single and paired bit patterns. Neither half is interesting alone. The
mask being right does not say the function applies it, and the function being
checked against a restatement of the positions does not say the positions are
the privileged ones.
-/

namespace Nonos.Rflags

/-- RFLAGS_PRIVILEGED_MASK as `src/arch/x86_64/context/rflags.rs` writes it. -/
def privilegedMask : Nat := 0x1F7500

/-- RFLAGS_RESERVED_SET: bit 1 reads as one on every x86-64 part. -/
def reservedSet : Nat := 0x2

/-- RFLAGS_IF: bit 9, set only on the resume that lands in user mode. -/
def interruptFlag : Nat := 0x200

/-- Bit `i` of `n`. -/
def bit (n i : Nat) : Bool := n / 2 ^ i % 2 == 1

/-- The positions the mask is meant to cover: TF (8), DF (10), IOPL (12, 13),
    NT (14), RF (16), VM (17), AC (18), VIF (19), VIP (20). Written as
    positions so that this and `privilegedMask` are two independent statements
    of one fact, and disagreeing is an error rather than a silent drift. -/
def privilegedBits : List Nat := [8, 10, 12, 13, 14, 16, 17, 18, 19, 20]

/-! ### The mask is the bits it stands for -/

/-- Every position in the mask is a privileged one and every privileged one is
    in the mask, across all 64 bits. Changing a digit of the hex literal, or
    the list, makes this false. -/
theorem mask_is_exactly_the_privileged_bits :
    (List.range 64).all (fun i => bit privilegedMask i == privilegedBits.contains i) = true := by
  decide

/-- IOPL is both of its bits, and both are masked. Called out separately
    because it is the one that grants hardware access rather than a nuisance. -/
theorem iopl_is_masked :
    bit privilegedMask 12 = true ∧ bit privilegedMask 13 = true := by
  decide

/-- The reserved bit is not something the mask clears; the sanitizer puts it
    back, so the two constants must not overlap. -/
theorem reserved_bit_is_not_masked : bit privilegedMask 1 = false := by
  decide

/-- Nor is IF. A CPL=0 continuation saved with interrupts off has to resume
    with them off, so the mask must leave bit 9 alone and let the caller
    decide. -/
theorem interrupt_flag_is_not_masked : bit privilegedMask 9 = false := by
  decide

/-- The reserved bit and IF are the single bits they claim to be. -/
theorem the_single_bit_constants_are_single_bits :
    (List.range 64).all (fun i => bit reservedSet i == (i == 1)) = true ∧
      (List.range 64).all (fun i => bit interruptFlag i == (i == 9)) = true := by
  decide

end Nonos.Rflags
