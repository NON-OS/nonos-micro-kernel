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

use super::super::sub_word::sub_word;
use super::super::xtime::xtime;
use super::types::{Aes256, EXPANDED_WORDS, KEY_BYTES, NK};

impl Aes256 {
    pub fn new(key: &[u8; KEY_BYTES]) -> Self {
        let mut w = [0u32; EXPANDED_WORDS];
        for (i, word) in w.iter_mut().enumerate().take(NK) {
            let b = i * 4;
            *word = u32::from_be_bytes([key[b], key[b + 1], key[b + 2], key[b + 3]]);
        }
        let mut rcon: u8 = 1;
        for i in NK..EXPANDED_WORDS {
            let mut temp = w[i - 1];
            if i % NK == 0 {
                temp = sub_word(temp.rotate_left(8)) ^ ((rcon as u32) << 24);
                rcon = xtime(rcon);
            } else if i % NK == 4 {
                // The extra substitution at the half-way word exists only for
                // 256-bit keys; without it the schedule silently diverges.
                temp = sub_word(temp);
            }
            w[i] = w[i - NK] ^ temp;
        }
        Self { round_keys: w }
    }
}
