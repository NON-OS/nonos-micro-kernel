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

use super::alloc_block::alloc_block;
use super::child_node::child_node;
use super::commit::commit;
use super::dir_link::link;
use super::write_node::write_node;
use super::{BlockFsError, BlockFsMount, BlockFsNode};

pub fn create(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    parent_lba: u64,
    parent: &mut BlockFsNode,
    name: &[u8],
    mode: u16,
) -> Result<u64, BlockFsError> {
    let child_lba = alloc_block(mount)?;
    commit(key, mount)?;
    let node = child_node(mount.superblock.generation, mode);
    write_node(key, child_lba, &node)?;
    link(key, mount, parent_lba, parent, name, child_lba)?;
    Ok(child_lba)
}
