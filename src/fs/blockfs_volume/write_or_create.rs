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

use super::create::create;
use super::error::VolumeError;
use super::state::VOLUME;
use super::write::write;
use crate::fs::blockfs::{self, BlockFsError, MODE_FILE};

pub fn write_or_create(path: &[u8], data: &[u8]) -> Result<(), VolumeError> {
    let exists = {
        let guard = VOLUME.read();
        let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
        match blockfs::resolve(&state.key, &state.mount, path) {
            Ok(_) => true,
            Err(BlockFsError::NotFound) => false,
            Err(e) => return Err(VolumeError::BlockFs(e)),
        }
    };
    if !exists {
        create(path, MODE_FILE)?;
    }
    write(path, data)
}
