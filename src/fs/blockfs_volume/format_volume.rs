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
use super::state::{VolumeState, VOLUME};
use crate::fs::blockfs;
use crate::security::keyring_capsule::{store, KeyType};

pub fn format_volume(uuid: [u8; 16]) -> Result<u32, VolumeError> {
    let mut key = [0u8; 32];
    crate::crypto::rng::fill_random_bytes(&mut key);
    let key_id = store(KeyType::MasterKey, &key, 0).map_err(VolumeError::Keyring)?;
    let mount = blockfs::format(&key, uuid).map_err(VolumeError::BlockFs)?;
    *VOLUME.write() = Some(VolumeState { key, mount });
    Ok(key_id)
}
