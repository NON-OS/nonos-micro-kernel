/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's ABI calldata builder
(`nox/calldata_addr.rs`). A read call for `f(address)` is the 4-byte selector,
twelve zero bytes of left padding, then the twenty address bytes. These
theorems fix that layout so a call can never send a selector into the address
word or leak stray bytes into the padding an on-chain contract reads as the
high bits of an argument.
-/

namespace Nonos.WalletNoxCalldata

/-- The byte at offset `i` of `calldata_addr selector addr`. -/
def byteAt (selector addr : Nat → UInt8) (i : Nat) : UInt8 :=
  if i < 4 then selector i
  else if i < 16 then 0
  else addr (i - 16)

/-- Total calldata length: selector (4) + padded address word (32). -/
def length : Nat := 36

/-- The first four bytes are exactly the function selector. -/
theorem selector_placed (sel addr : Nat → UInt8) (i : Nat) (h : i < 4) :
    byteAt sel addr i = sel i := by
  unfold byteAt; simp [h]

/-- The twelve bytes between the selector and the address are all zero: the
    address argument is left-padded, never carrying stray high bits. -/
theorem padding_is_zero (sel addr : Nat → UInt8) (i : Nat) (h1 : 4 ≤ i) (h2 : i < 16) :
    byteAt sel addr i = 0 := by
  unfold byteAt
  have : ¬ i < 4 := by omega
  simp [this, h2]

/-- The final twenty bytes are the address, right-aligned in the 32-byte word. -/
theorem address_placed (sel addr : Nat → UInt8) (i : Nat) (h1 : 16 ≤ i) (h2 : i < 36) :
    byteAt sel addr i = addr (i - 16) := by
  unfold byteAt
  have h3 : ¬ i < 4 := by omega
  have h4 : ¬ i < 16 := by omega
  simp [h3, h4]

/-- The address occupies the low twenty bytes of the argument word, so exactly
    twelve pad bytes precede it: `36 = 4 + 12 + 20`. -/
theorem layout_accounts_for_every_byte : length = 4 + 12 + 20 := rfl

end Nonos.WalletNoxCalldata
