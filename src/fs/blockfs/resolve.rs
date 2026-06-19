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

use super::dir_lookup::lookup;
use super::read_node::read_node;
use super::{BlockFsError, BlockFsMount};

pub fn resolve(key: &[u8; 32], mount: &BlockFsMount, path: &[u8]) -> Result<u64, BlockFsError> {
    let mut node_lba = mount.superblock.root_lba;
    let mut node = read_node(key, node_lba)?;
    for comp in path.split(|&b| b == b'/').filter(|c| !c.is_empty()) {
        match lookup(key, &node, comp)? {
            Some(child) => {
                node_lba = child;
                node = read_node(key, node_lba)?;
            }
            None => return Err(BlockFsError::NotFound),
        }
    }
    Ok(node_lba)
}
