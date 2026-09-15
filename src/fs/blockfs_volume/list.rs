// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
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

//! What is in a directory on the encrypted volume.

use alloc::vec::Vec;

use super::error::VolumeError;
use super::state::VOLUME;
use crate::fs::blockfs;

/// Names only. The addresses `blockfs::list` also returns are volume
/// internals: a caller outside the kernel has no use for a node address and
/// handing one out would make the layout part of the interface.
///
/// An empty path is the volume root, which is how a caller asks what the
/// machine kept without knowing any name in advance.
pub fn list(path: &[u8]) -> Result<Vec<Vec<u8>>, VolumeError> {
    let guard = VOLUME.read();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    let node_lba =
        blockfs::resolve(&state.key, &state.mount, path).map_err(VolumeError::BlockFs)?;
    let node = blockfs::read_node(&state.key, node_lba).map_err(VolumeError::BlockFs)?;
    let entries = blockfs::list(&state.key, &node).map_err(VolumeError::BlockFs)?;
    Ok(entries.into_iter().map(|e| e.name).collect())
}
