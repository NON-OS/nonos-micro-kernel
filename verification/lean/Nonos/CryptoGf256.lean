/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of GF(256) addition, the exclusive-or that carries the
wallet's QR Reed-Solomon error correction (`nonos_qr` `reed_solomon`, and the
0x11D reduction in `gf256/tables.rs`). Addition in a characteristic-two field is
XOR, and the encoder relies on its group structure: `d ^ rem[0]`, `*r ^= mul(..)`
and `next[j] ^= c` all assume XOR is associative, commutative and self-cancelling.
These theorems establish exactly those laws, so the codeword arithmetic is sound.
-/

namespace Nonos.CryptoGf256

/-- Field addition: bitwise exclusive-or. -/
def add (a b : Nat) : Nat := a ^^^ b

/-- Zero is the additive identity: `a + 0 = a`, matching an untouched codeword
    byte. -/
theorem add_zero (a : Nat) : add a 0 = a := Nat.xor_zero a

/-- Every element is its own inverse: `a + a = 0`. This is what lets the RS
    remainder subtraction be the same XOR as addition. -/
theorem add_self (a : Nat) : add a a = 0 := Nat.xor_self a

/-- Addition is commutative. -/
theorem add_comm (a b : Nat) : add a b = add b a := Nat.xor_comm a b

/-- Addition is associative, so a chain of `^=` accumulations is order
    independent. -/
theorem add_assoc (a b c : Nat) : add (add a b) c = add a (add b c) :=
  Nat.xor_assoc a b c

/-- Cancellation: adding the same value twice returns the original, so mixing a
    generator term into the remainder and later removing it is exact. -/
theorem add_cancel (a b : Nat) : add (add a b) b = a := by
  unfold add
  rw [Nat.xor_assoc, Nat.xor_self, Nat.xor_zero]

/-- Left cancellation of the identity: `0 + a = a`. -/
theorem zero_add (a : Nat) : add 0 a = a := Nat.zero_xor a

end Nonos.CryptoGf256
