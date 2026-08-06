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

use super::types::{Aes128, BLOCK_BYTES};

impl Aes128 {
    pub(super) fn add_round_key(&self, block: &mut [u8; BLOCK_BYTES], round: usize) {
        for col in 0..4 {
            let word = self.round_keys[round * 4 + col].to_be_bytes();
            for row in 0..4 {
                block[col * 4 + row] ^= word[row];
            }
        }
    }
}
