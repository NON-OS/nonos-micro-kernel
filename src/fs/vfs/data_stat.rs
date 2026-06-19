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

use super::error::VfsResult;
use super::map_volume_err::map_volume_err;
use super::types::{FileMetadata, FileType};

pub(super) fn data_stat(path: &str, rel: &[u8]) -> VfsResult<FileMetadata> {
    let (size, is_dir) = crate::fs::blockfs_volume::stat(rel).map_err(map_volume_err)?;
    let mut inode = 1u64;
    for byte in path.bytes() {
        inode = inode.wrapping_mul(31).wrapping_add(byte as u64);
    }
    let now = crate::time::timestamp_secs();
    Ok(FileMetadata {
        size,
        atime: now,
        mtime: now,
        ctime: now,
        file_type: if is_dir { FileType::Directory } else { FileType::File },
        mode: if is_dir { 0o040755 } else { 0o100644 },
        inode,
    })
}
