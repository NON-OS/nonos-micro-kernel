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

use super::dot::dot;
use super::types::Polyval;

impl Polyval {
    pub fn new(key: &[u8; 16]) -> Self {
        Self { h: load(key), acc: [0u64; 2] }
    }

    /// Absorb one 16-byte block. Short blocks are the caller's problem: the
    /// AEAD above pads to the block boundary with a defined length encoding.
    pub fn update(&mut self, block: &[u8; 16]) {
        let b = load(block);
        self.acc[0] ^= b[0];
        self.acc[1] ^= b[1];
        let h = self.h;
        dot(&mut self.acc, &h);
    }
}

fn load(bytes: &[u8; 16]) -> [u64; 2] {
    let mut lo = [0u8; 8];
    let mut hi = [0u8; 8];
    lo.copy_from_slice(&bytes[..8]);
    hi.copy_from_slice(&bytes[8..]);
    [u64::from_le_bytes(lo), u64::from_le_bytes(hi)]
}
