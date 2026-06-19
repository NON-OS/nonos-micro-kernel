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

use super::{BlockFsError, BlockFsMount};

pub fn alloc_block(mount: &mut BlockFsMount) -> Result<u64, BlockFsError> {
    let lba = mount.superblock.free_lba;
    if lba >= mount.superblock.sectors {
        return Err(BlockFsError::OutOfSpace);
    }
    mount.superblock.free_lba = lba + 1;
    Ok(lba)
}
