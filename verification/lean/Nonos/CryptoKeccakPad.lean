/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of Keccak / SHA-3 `pad10*1` block framing, used
whenever the wallet hashes a transaction or derives an address. The message is
padded to a whole number of rate-sized blocks, and the padding is never empty
so the domain and terminating bits always have room. These theorems fix that the
padded length is a multiple of the rate and strictly larger than the message,
so the sponge absorbs complete blocks and no message is confused with its own
padding.
-/

namespace Nonos.CryptoKeccakPad

/-- Number of absorbed blocks after padding: at least one more than fits, since
    `pad10*1` always appends. -/
def blocks (len rate : Nat) : Nat := len / rate + 1

/-- Padded length in bytes: a whole number of rate-sized blocks. -/
def padLen (len rate : Nat) : Nat := blocks len rate * rate

/-- The padded message is an exact multiple of the rate: the sponge only ever
    absorbs full blocks. -/
theorem multiple_of_rate (len rate : Nat) : padLen len rate % rate = 0 := by
  unfold padLen blocks
  exact Nat.mul_mod_left _ rate

/-- Padding is always applied: the padded length strictly exceeds the message,
    so at least one pad byte carrying the domain and stop bits is present. -/
theorem always_pads (len rate : Nat) (hr : 0 < rate) : len < padLen len rate := by
  unfold padLen blocks
  rw [← Nat.div_lt_iff_lt_mul hr]
  exact Nat.lt_succ_self _

/-- The padding is non-empty: there is always at least one byte between the
    message and the block boundary. -/
theorem padding_nonempty (len rate : Nat) (hr : 0 < rate) :
    0 < padLen len rate - len :=
  Nat.sub_pos_of_lt (always_pads len rate hr)

/-- At least one full block is absorbed even for the empty message. -/
theorem at_least_one_block (len rate : Nat) : 1 ≤ blocks len rate := by
  unfold blocks
  exact Nat.le_add_left 1 (len / rate)

end Nonos.CryptoKeccakPad
