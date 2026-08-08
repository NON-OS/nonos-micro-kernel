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

use super::quarter::quarter;
use super::types::{ChaCha20, BLOCK_BYTES};

impl ChaCha20 {
    pub(super) fn next_block(&mut self) -> [u8; BLOCK_BYTES] {
        let mut w = self.state;
        for _ in 0..10 {
            quarter(&mut w, 0, 4, 8, 12);
            quarter(&mut w, 1, 5, 9, 13);
            quarter(&mut w, 2, 6, 10, 14);
            quarter(&mut w, 3, 7, 11, 15);
            quarter(&mut w, 0, 5, 10, 15);
            quarter(&mut w, 1, 6, 11, 12);
            quarter(&mut w, 2, 7, 8, 13);
            quarter(&mut w, 3, 4, 9, 14);
        }
        let mut out = [0u8; BLOCK_BYTES];
        for i in 0..16 {
            let word = w[i].wrapping_add(self.state[i]);
            out[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
        }
        self.state[12] = self.state[12].wrapping_add(1);
        out
    }
}
