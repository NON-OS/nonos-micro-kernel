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

use crate::hardware::tpm::{nv_read, NvIndex};
use crate::security::anti_rollback::types::{RollbackError, VersionState, NVRAM_VERSION_INDEX};
use crate::security::anti_rollback::util::constant_time_eq_32;

pub(crate) fn read_from_nvram() -> Result<VersionState, RollbackError> {
    let index = NvIndex::new(NVRAM_VERSION_INDEX);
    let mut buf = [0u8; 48];
    match nv_read(&index, &mut buf) {
        Ok(48) => {
            let state = VersionState::from_bytes(&buf);
            let stored_hash = read_nvram_hash()?;
            if !constant_time_eq_32(&stored_hash, &state.compute_hash()) {
                return Err(RollbackError::NvramReadFailed);
            }
            Ok(state)
        }
        _ => Err(RollbackError::NvramReadFailed),
    }
}

fn read_nvram_hash() -> Result<[u8; 32], RollbackError> {
    let index = NvIndex::new(NVRAM_VERSION_INDEX + 1);
    let mut buf = [0u8; 32];
    match nv_read(&index, &mut buf) {
        Ok(32) => Ok(buf),
        _ => Err(RollbackError::NvramReadFailed),
    }
}
