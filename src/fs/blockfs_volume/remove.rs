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

use super::error::VolumeError;
use super::state::VOLUME;
use crate::fs::blockfs;

pub fn remove(path: &[u8]) -> Result<(), VolumeError> {
    let guard = VOLUME.write();
    let state = guard.as_ref().ok_or(VolumeError::NotMounted)?;
    blockfs::unlink_path(&state.key, &state.mount, path).map_err(VolumeError::BlockFs)
}
