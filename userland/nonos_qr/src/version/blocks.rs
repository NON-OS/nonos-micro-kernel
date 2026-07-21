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

//! Error-correction block layout per (version, level), ISO/IEC 18004 versions
//! 1 through 10.

use super::ecc::Ecc;

/// Block layout for one (version, level): EC codewords per block, then the two
/// size groups as (block_count, data_codewords_per_block).
pub(crate) struct Blocks {
    pub ec_per_block: u16,
    pub g1_blocks: u16,
    pub g1_data: u16,
    pub g2_blocks: u16,
    pub g2_data: u16,
}

impl Blocks {
    pub(crate) fn total_data_codewords(&self) -> usize {
        (self.g1_blocks * self.g1_data + self.g2_blocks * self.g2_data) as usize
    }
    pub(crate) fn total_blocks(&self) -> usize {
        (self.g1_blocks + self.g2_blocks) as usize
    }
}

// [version-1][ecc-index] = (ec_per_block, g1_blocks, g1_data, g2_blocks, g2_data).
#[rustfmt::skip]
const TABLE: [[(u16, u16, u16, u16, u16); 4]; 10] = [
    [(7,1,19,0,0),(10,1,16,0,0),(13,1,13,0,0),(17,1,9,0,0)],
    [(10,1,34,0,0),(16,1,28,0,0),(22,1,22,0,0),(28,1,16,0,0)],
    [(15,1,55,0,0),(26,1,44,0,0),(18,2,17,0,0),(22,2,13,0,0)],
    [(20,1,80,0,0),(18,2,32,0,0),(26,2,24,0,0),(16,4,9,0,0)],
    [(26,1,108,0,0),(24,2,43,0,0),(18,2,15,2,16),(22,2,11,2,12)],
    [(18,2,68,0,0),(16,4,27,0,0),(24,4,19,0,0),(28,4,15,0,0)],
    [(20,2,78,0,0),(18,4,31,0,0),(18,2,14,4,15),(26,4,13,1,14)],
    [(24,2,97,0,0),(22,2,38,2,39),(22,4,18,2,19),(26,4,14,2,15)],
    [(30,2,116,0,0),(22,3,36,2,37),(20,4,16,4,17),(24,4,12,4,13)],
    [(18,2,68,2,69),(26,4,43,1,44),(24,6,19,2,20),(28,6,15,2,16)],
];

pub(crate) fn blocks(version: u8, ecc: Ecc) -> Blocks {
    let (ec, g1b, g1d, g2b, g2d) = TABLE[(version - 1) as usize][ecc.index()];
    Blocks { ec_per_block: ec, g1_blocks: g1b, g1_data: g1d, g2_blocks: g2b, g2_data: g2d }
}
