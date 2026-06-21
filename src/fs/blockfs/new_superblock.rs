// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::constants::FIRST_ALLOC_LBA;
use super::BlockFsSuperblock;

pub fn new_superblock(sectors: u64, uuid: [u8; 16]) -> BlockFsSuperblock {
    BlockFsSuperblock {
        generation: 1,
        sectors,
        root_lba: FIRST_ALLOC_LBA,
        free_lba: FIRST_ALLOC_LBA + 1,
        block_bytes: crate::fs::cryptoblock::PLAIN_BLOCK_BYTES as u64,
        uuid,
        digest: [0u8; 32],
    }
}
