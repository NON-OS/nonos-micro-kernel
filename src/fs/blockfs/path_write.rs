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

use super::read_node::read_node;
use super::resolve::resolve;
use super::write_file::write_file;
use super::{BlockFsError, BlockFsMount};

pub fn write_path(
    key: &[u8; 32],
    mount: &mut BlockFsMount,
    path: &[u8],
    data: &[u8],
) -> Result<(), BlockFsError> {
    let lba = resolve(key, mount, path)?;
    let mut node = read_node(key, lba)?;
    write_file(key, mount, lba, &mut node, data)
}
