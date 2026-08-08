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

use super::iv::IV;
use super::mix::mix;
use super::sigma::SIGMA;
use super::types::{Blake2b, BLOCK_BYTES};

impl Blake2b {
    pub(super) fn compress(&mut self, block: &[u8; BLOCK_BYTES], last: bool) {
        let mut m = [0u64; 16];
        for (i, word) in m.iter_mut().enumerate() {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&block[i * 8..i * 8 + 8]);
            *word = u64::from_le_bytes(bytes);
        }
        let mut v = [0u64; 16];
        v[..8].copy_from_slice(&self.h);
        v[8..].copy_from_slice(&IV);
        v[12] ^= self.counter as u64;
        v[13] ^= (self.counter >> 64) as u64;
        if last {
            v[14] = !v[14];
        }
        for round in 0..12 {
            let s = &SIGMA[round];
            mix(&mut v, 0, 4, 8, 12, m[s[0]], m[s[1]]);
            mix(&mut v, 1, 5, 9, 13, m[s[2]], m[s[3]]);
            mix(&mut v, 2, 6, 10, 14, m[s[4]], m[s[5]]);
            mix(&mut v, 3, 7, 11, 15, m[s[6]], m[s[7]]);
            mix(&mut v, 0, 5, 10, 15, m[s[8]], m[s[9]]);
            mix(&mut v, 1, 6, 11, 12, m[s[10]], m[s[11]]);
            mix(&mut v, 2, 7, 8, 13, m[s[12]], m[s[13]]);
            mix(&mut v, 3, 4, 9, 14, m[s[14]], m[s[15]]);
        }
        for i in 0..8 {
            self.h[i] ^= v[i] ^ v[i + 8];
        }
    }
}
