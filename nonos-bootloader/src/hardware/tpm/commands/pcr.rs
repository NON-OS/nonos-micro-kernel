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

use crate::hardware::tpm::state::TpmState;
use crate::hardware::tpm::types::TpmError;

pub fn pcr_extend_impl(
    state: &TpmState,
    pcr_index: u32,
    digest: &[u8; 32],
) -> Result<(), TpmError> {
    if !state.initialized {
        return Err(TpmError::NotPresent);
    }
    state.request_locality()?;
    let mut cmd = [0u8; 51];
    cmd[0..2].copy_from_slice(&0x8001u16.to_be_bytes());
    cmd[2..6].copy_from_slice(&51u32.to_be_bytes());
    cmd[6..10].copy_from_slice(&0x0000_0182u32.to_be_bytes());
    cmd[10..14].copy_from_slice(&pcr_index.to_be_bytes());
    cmd[14..18].copy_from_slice(&1u32.to_be_bytes());
    cmd[18..19].copy_from_slice(&[0x0B]);
    cmd[19..51].copy_from_slice(digest);
    state.send_command(&cmd)?;
    let mut response = [0u8; 32];
    let len = state.receive_response(&mut response)?;
    state.release_locality();
    if len < 10 {
        return Err(TpmError::InvalidResponse);
    }
    let rc = u32::from_be_bytes([response[6], response[7], response[8], response[9]]);
    if rc != 0 {
        return Err(TpmError::CommandFailed(rc));
    }
    Ok(())
}
