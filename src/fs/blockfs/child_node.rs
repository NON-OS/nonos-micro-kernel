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

use super::BlockFsNode;

pub(crate) fn child_node(generation: u64, mode: u16) -> BlockFsNode {
    BlockFsNode {
        generation,
        mode,
        uid: 0,
        gid: 0,
        links: 1,
        size: 0,
        blocks: 1,
        ctime: 0,
        mtime: 0,
        atime: 0,
        first_record_lba: 0,
        digest: [0u8; 32],
    }
}
