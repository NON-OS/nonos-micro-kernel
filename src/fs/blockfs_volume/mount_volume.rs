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
use super::key_to_array::key_to_array;
use super::state::{VolumeState, VOLUME};
use crate::fs::blockfs;
use crate::security::keyring_capsule::retrieve;

pub fn mount_volume(key_id: u32) -> Result<(), VolumeError> {
    let bytes = retrieve(key_id).map_err(VolumeError::Keyring)?;
    let key = key_to_array(&bytes)?;
    let mount = blockfs::mount(&key).map_err(VolumeError::BlockFs)?;
    *VOLUME.write() = Some(VolumeState { key, mount });
    Ok(())
}
