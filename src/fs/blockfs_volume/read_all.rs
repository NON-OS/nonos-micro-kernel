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

extern crate alloc;

use alloc::vec::Vec;

use super::error::VolumeError;
use super::state::VOLUME;
use crate::fs::blockfs;

pub fn read_all(path: &[u8]) -> Result<Vec<u8>, VolumeError> {
    let guard = VOLUME.read();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    let lba = blockfs::resolve(&state.key, &state.mount, path).map_err(VolumeError::BlockFs)?;
    let node = blockfs::read_node(&state.key, lba).map_err(VolumeError::BlockFs)?;
    // Clamp the allocation to the largest a file can actually occupy. node.size
    // is read verbatim from a stored inode; a corrupt or hostile value (up to
    // u64::MAX) would otherwise force an arbitrarily large zeroed allocation and
    // OOM the FS. A real file cannot exceed MAX_FILE_BYTES (the index holds at
    // most MAX_PTRS block pointers), and read_file bounds the actual copy.
    let cap = (node.size as usize).min(blockfs::MAX_FILE_BYTES);
    let mut out = alloc::vec![0u8; cap];
    let n = blockfs::read_file(&state.key, &node, &mut out).map_err(VolumeError::BlockFs)?;
    out.truncate(n);
    Ok(out)
}
