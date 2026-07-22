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

use super::MAX_WORDS;

/// Turn 16/20/24/28/32 bytes of entropy into BIP39 word indices: the entropy
/// bits followed by the first `bits/32` bits of its SHA-256, read out in
/// 11-bit groups. Writes into `out` and returns the word count, or None for
/// an entropy length outside the standard.
pub fn entropy_to_words(entropy: &[u8], out: &mut [u16; MAX_WORDS]) -> Option<usize> {
    let bits = entropy.len() * 8;
    if !matches!(bits, 128 | 160 | 192 | 224 | 256) {
        return None;
    }
    let checksum_bits = bits / 32;
    let word_count = (bits + checksum_bits) / 11;
    let checksum = sha256(entropy)[0];

    // Read the concatenated entropy||checksum bit stream 11 bits at a time.
    let bit_at = |i: usize| -> u16 {
        if i < bits {
            ((entropy[i / 8] >> (7 - (i % 8))) & 1) as u16
        } else {
            ((checksum >> (7 - (i - bits))) & 1) as u16
        }
    };

    for (w, slot) in out.iter_mut().take(word_count).enumerate() {
        let mut index = 0u16;
        for b in 0..11 {
            index = (index << 1) | bit_at(w * 11 + b);
        }
        *slot = index;
    }
    Some(word_count)
}
