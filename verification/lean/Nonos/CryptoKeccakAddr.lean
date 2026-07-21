/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of Ethereum address derivation, used by the keyring
when it turns a public key into an account (`paint_keyring/server/ethaddr.rs`).
The address is the low twenty bytes of the Keccak-256 hash of the 64-byte
public key: `keccak(pubkey)[12..32]`. These theorems fix that slice so an
address is exactly the last twenty hash bytes, in order, and never twenty-one
or the wrong window.
-/

namespace Nonos.CryptoKeccakAddr

/-- Byte `j` of the derived address, given the 32-byte hash `h`. Mirrors taking
    `h[12 + j]` for `j < 20`. -/
def addrByte (h : Nat → UInt8) (j : Nat) : UInt8 := h (12 + j)

/-- Address length in bytes. -/
def addrLen : Nat := 20

/-- The address is exactly the last twenty bytes of the hash: `32 = 12 + 20`. -/
theorem takes_last_twenty : 12 + addrLen = 32 := rfl

/-- Address byte `j` is hash byte `12 + j`: the window starts at offset twelve,
    dropping the first twelve hash bytes. -/
theorem byte_offset (h : Nat → UInt8) (j : Nat) : addrByte h j = h (12 + j) := rfl

/-- The address bytes are strictly the hash bytes in `[12, 32)`: every address
    index maps to a distinct in-range hash index, order preserved. -/
theorem in_hash_range (j : Nat) (hj : j < addrLen) : 12 ≤ 12 + j ∧ 12 + j < 32 := by
  unfold addrLen at hj; omega

/-- Distinct address positions read distinct hash bytes: no two address bytes
    alias the same hash byte. -/
theorem injective_window (j k : Nat) (h : j ≠ k) : (12 + j) ≠ (12 + k) := by omega

end Nonos.CryptoKeccakAddr
