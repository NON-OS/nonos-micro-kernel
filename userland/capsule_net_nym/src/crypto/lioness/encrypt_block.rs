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

use super::mac_into_left::mac_into_left;
use super::stream_into_right::stream_into_right;
use super::types::{BlockTooShort, Lioness, MAC_BYTES};

impl Lioness {
    /// The block must be longer than the MAC width, not merely equal: an empty
    /// right half would leave the left half unmixed.
    pub fn encrypt_block(&self, block: &mut [u8]) -> Result<(), BlockTooShort> {
        if block.len() <= MAC_BYTES {
            return Err(BlockTooShort);
        }
        let (left, right) = block.split_at_mut(MAC_BYTES);
        stream_into_right(left, right, &self.k1);
        mac_into_left(left, right, &self.k2);
        stream_into_right(left, right, &self.k3);
        mac_into_left(left, right, &self.k4);
        Ok(())
    }
}
