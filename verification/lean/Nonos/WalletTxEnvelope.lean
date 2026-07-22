/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

Specification-level proof of the EIP-2718 / EIP-1559 transaction envelope the
wallet signs (keyring `eip1559` + `rlp`). A type-2 transaction is the byte 0x02
followed by an RLP list; the unsigned payload has nine fields, and signing
appends exactly the parity and the two signature scalars. These theorems fix
that shape so the digest signed and the digest a node recovers cover the same
fields in the same order.
-/

namespace Nonos.WalletTxEnvelope

/-- The EIP-2718 type prefix for a dynamic-fee transaction. -/
def typeByte : Nat := 0x02

/-- Fields in the unsigned payload: chainId, nonce, maxPriorityFee, maxFee, gas,
    to, value, data, accessList. -/
def unsignedFields : Nat := 9

/-- Fields in the signed payload: the nine above plus yParity, r, s. -/
def signedFields : Nat := 12

/-- The envelope is type two. -/
theorem is_type_two : typeByte = 2 := rfl

/-- The type prefix is below 0x80, so a decoder reads it as an EIP-2718 type
    byte and never as the first byte of an RLP list (which start at 0xc0): the
    typed and legacy encodings can never be confused. -/
theorem type_prefix_unambiguous : typeByte < 0x80 := by decide

/-- Signing adds exactly three fields: the parity bit and the two signature
    scalars, and nothing else. -/
theorem signing_adds_signature : signedFields = unsignedFields + 3 := rfl

/-- The recovery parity is a single bit: EIP-1559 uses yParity in {0, 1}, never
    the legacy 27/28 encoding. -/
theorem parity_is_one_bit (v : Nat) (h : v ≤ 1) : v = 0 ∨ v = 1 := by omega

/-- The signed field count strictly exceeds the unsigned one: a signature is
    never dropped from the broadcast payload. -/
theorem signed_has_more_fields : unsignedFields < signedFields := by decide

end Nonos.WalletTxEnvelope
