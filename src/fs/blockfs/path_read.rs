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

use super::read_file::read_file;
use super::read_node::read_node;
use super::resolve::resolve;
use super::{BlockFsError, BlockFsMount};

pub fn read_path(
    key: &[u8; 32],
    mount: &BlockFsMount,
    path: &[u8],
    out: &mut [u8],
) -> Result<usize, BlockFsError> {
    let lba = resolve(key, mount, path)?;
    let node = read_node(key, lba)?;
    read_file(key, &node, out)
}
