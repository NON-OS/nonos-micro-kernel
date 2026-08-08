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

use super::super::mix_columns::mix_columns;
use super::super::shift_rows::shift_rows;
use super::super::sub_bytes::sub_bytes;
use super::super::types::BLOCK_BYTES;
use super::types::{Aes256, ROUNDS};

impl Aes256 {
    pub fn encrypt_block(&self, block: &mut [u8; BLOCK_BYTES]) {
        self.add_round_key(block, 0);
        for round in 1..ROUNDS {
            sub_bytes(block);
            shift_rows(block);
            mix_columns(block);
            self.add_round_key(block, round);
        }
        sub_bytes(block);
        shift_rows(block);
        self.add_round_key(block, ROUNDS);
    }

    fn add_round_key(&self, block: &mut [u8; BLOCK_BYTES], round: usize) {
        for col in 0..4 {
            let word = self.round_keys[round * 4 + col].to_be_bytes();
            for row in 0..4 {
                block[col * 4 + row] ^= word[row];
            }
        }
    }
}
