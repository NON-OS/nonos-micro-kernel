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

// Reduce a big-endian 256-bit word to u128, or None if the top 16 bytes carry
// value. Real token and staking amounts sit far below 2^128, so a value that
// does not fit signals a decode error rather than a genuine balance.
pub fn q32_to_u128(word: &[u8; 32]) -> Option<u128> {
    if word[0..16].iter().any(|b| *b != 0) {
        return None;
    }
    let mut v: u128 = 0;
    for b in &word[16..32] {
        v = (v << 8) | *b as u128;
    }
    Some(v)
}
