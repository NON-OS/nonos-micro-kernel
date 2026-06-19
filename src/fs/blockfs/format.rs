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

use super::header_lba::header_lba;
use super::new_superblock::new_superblock;
use super::root_node::root_node;
use super::serialize::serialize;
use super::validate_geometry::validate_geometry;
use super::write_node::write_node;
use super::{BlockFsError, BlockFsMount};

pub fn format(key: &[u8; 32], uuid: [u8; 16]) -> Result<BlockFsMount, BlockFsError> {
    let sectors = validate_geometry()?;
    let superblock = new_superblock(sectors, uuid);
    let payload = serialize(&superblock);
    let header_lba = header_lba(superblock.generation);
    write_node(key, superblock.root_lba, &root_node(superblock.generation))?;
    crate::fs::cryptoblock::write(key, header_lba, &payload).map_err(BlockFsError::CryptoBlock)?;
    Ok(BlockFsMount { header_lba, superblock })
}
