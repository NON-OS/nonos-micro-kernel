// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/// Bitcoin-style base58, which is how Nym renders every key in an address.
const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Decode into exactly 32 bytes.
///
/// Leading zero bytes are not encoded positionally in base58, so a key with
/// them decodes short and is right-aligned rather than rejected.
pub fn decode32(text: &[u8]) -> Option<[u8; 32]> {
    let mut acc = [0u8; 32];
    for ch in text {
        let digit = ALPHABET.iter().position(|a| a == ch)? as u32;
        let mut carry = digit;
        for byte in acc.iter_mut().rev() {
            let value = (*byte as u32) * 58 + carry;
            *byte = value as u8;
            carry = value >> 8;
        }
        if carry != 0 {
            return None;
        }
    }
    Some(acc)
}
