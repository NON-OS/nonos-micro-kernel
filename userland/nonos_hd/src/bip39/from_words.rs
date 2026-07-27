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

use crate::sha256::sha256;
use crate::wipe::wipe;

/// Rebuild entropy from BIP39 word indices and verify the checksum. Returns
/// the entropy (front-filled) and its byte length, or None for a wrong word
/// count, an out-of-range index, or a checksum mismatch: a mistyped or
/// reordered phrase is rejected here, before any key is derived from it.
pub fn words_to_entropy(indices: &[u16]) -> Option<([u8; 32], usize)> {
    let word_count = indices.len();
    if !matches!(word_count, 12 | 15 | 18 | 21 | 24) {
        return None;
    }
    if indices.iter().any(|&i| i >= 2048) {
        return None;
    }

    let total_bits = word_count * 11;
    let checksum_bits = word_count / 3;
    let entropy_bits = total_bits - checksum_bits;
    let entropy_len = entropy_bits / 8;

    let mut entropy = [0u8; 32];
    let mut checksum: u8 = 0;
    for bit in 0..total_bits {
        let value = (indices[bit / 11] >> (10 - (bit % 11))) & 1;
        if bit < entropy_bits {
            entropy[bit / 8] |= (value as u8) << (7 - (bit % 8));
        } else {
            checksum |= (value as u8) << (checksum_bits - 1 - (bit - entropy_bits));
        }
    }

    let expected = sha256(&entropy[..entropy_len])[0] >> (8 - checksum_bits);
    if expected != checksum {
        wipe(&mut entropy);
        return None;
    }
    Some((entropy, entropy_len))
}
