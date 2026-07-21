/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the RLP rules the wallet uses to build the
EIP-1559 transaction payload it signs. A single byte below 0x80 is its own
encoding; anything else takes a length prefix. These theorems fix the prefix
boundaries so a field is never mis-framed, which would change the transaction
hash the signature commits to.
-/

namespace Nonos.WalletRlp

/-- Header byte for a string of `len` bytes whose sole byte (when `len = 1`) is
    `first`. Mirrors RLP: a lone byte below 0x80 is bare, a short string gets a
    `0x80 + len` prefix. `none` means "no prefix, the byte stands alone". -/
def header (len first : Nat) : Option Nat :=
  if len = 1 ∧ first < 0x80 then none
  else if len ≤ 55 then some (0x80 + len)
  else none  -- long strings use a different framing, out of this lemma's scope

/-- A single low byte carries no prefix: `[0x00, 0x7f]` encodes as itself, so
    small scalars are not needlessly framed. -/
theorem single_low_byte_bare (b : Nat) (h : b < 0x80) : header 1 b = none := by
  unfold header; simp [h]

/-- A byte `0x80` or above is framed as a one-byte string with prefix `0x81`,
    never left bare where a decoder would read it as a header. -/
theorem high_byte_gets_prefix (b : Nat) (h : 0x80 ≤ b) : header 1 b = some 0x81 := by
  unfold header
  rw [if_neg (by omega : ¬ (1 = 1 ∧ b < 0x80)), if_pos (by omega : (1 : Nat) ≤ 55)]

/-- The short-string prefix is always in the RLP short range `[0x80, 0xb7]`, so
    it can never collide with the single-byte or long-string encodings. -/
theorem short_prefix_in_range (len first p : Nat)
    (hlen : 2 ≤ len) (hle : len ≤ 55) (h : header len first = some p) :
    0x80 ≤ p ∧ p ≤ 0xb7 := by
  unfold header at h
  have hne : ¬ (len = 1 ∧ first < 0x80) := by omega
  simp [hne, hle] at h
  omega

end Nonos.WalletRlp
