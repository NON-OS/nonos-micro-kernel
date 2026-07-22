/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof for the wallet's 256-bit quantity reducer
(`nox/q32_to_u128.rs`). An eth_call returns a 32-byte word; the wallet only
trusts it as a balance when the top sixteen bytes are zero, so a value that
would not fit u128 is refused rather than silently truncated into a wrong
balance. These theorems fix that guard and the bound it guarantees.
-/

namespace Nonos.WalletQuantity

/-- Big-endian value of sixteen bytes `b 0 .. b 15`, each `< 256`. -/
def be16 (b : Nat → Nat) : Nat :=
  (b 0) * 256 ^ 15 + (b 1) * 256 ^ 14 + (b 2) * 256 ^ 13 + (b 3) * 256 ^ 12 +
  (b 4) * 256 ^ 11 + (b 5) * 256 ^ 10 + (b 6) * 256 ^ 9 + (b 7) * 256 ^ 8 +
  (b 8) * 256 ^ 7 + (b 9) * 256 ^ 6 + (b 10) * 256 ^ 5 + (b 11) * 256 ^ 4 +
  (b 12) * 256 ^ 3 + (b 13) * 256 ^ 2 + (b 14) * 256 + (b 15)

/-- The reducer: `none` when the high half carries value, else the low-half
    big-endian integer. `highZero` is the guard `word[0..16] == 0`. -/
def reduce (highZero : Bool) (low : Nat → Nat) : Option Nat :=
  if highZero then some (be16 low) else none

/-- A word whose top sixteen bytes are non-zero is refused: no truncation into
    a bogus balance. -/
theorem high_bytes_refused (low : Nat → Nat) : reduce false low = none := rfl

/-- A word that fits u128 decodes to its low-half big-endian value. -/
theorem fitting_word_decoded (low : Nat → Nat) :
    reduce true low = some (be16 low) := rfl

/-- The decoded value is bounded by 2^128 when every low byte is a byte: the
    result genuinely fits the u128 the Rust reducer returns. -/
theorem decoded_lt_two_pow_128 (b : Nat → Nat) (h : ∀ i, b i < 256) :
    be16 b < 256 ^ 16 := by
  have h0 := h 0; have h1 := h 1; have h2 := h 2; have h3 := h 3
  have h4 := h 4; have h5 := h 5; have h6 := h 6; have h7 := h 7
  have h8 := h 8; have h9 := h 9; have h10 := h 10; have h11 := h 11
  have h12 := h 12; have h13 := h 13; have h14 := h 14; have h15 := h 15
  unfold be16
  omega

end Nonos.WalletQuantity
