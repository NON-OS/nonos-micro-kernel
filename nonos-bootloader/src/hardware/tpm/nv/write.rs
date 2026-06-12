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
use crate::hardware::tpm::types::{NvIndex, TpmError};

pub fn nv_write_impl(state: &TpmState, index: &NvIndex, data: &[u8]) -> Result<(), TpmError> {
    if !state.initialized {
        return Err(TpmError::NotPresent);
    }
    state.request_locality()?;
    let cmd_len = 22 + data.len();
    let mut cmd = [0u8; 256];
    cmd[0..2].copy_from_slice(&0x8001u16.to_be_bytes());
    cmd[2..6].copy_from_slice(&(cmd_len as u32).to_be_bytes());
    cmd[6..10].copy_from_slice(&0x0000_0137u32.to_be_bytes());
    cmd[10..14].copy_from_slice(&index.raw().to_be_bytes());
    cmd[14..18].copy_from_slice(&index.raw().to_be_bytes());
    cmd[18..20].copy_from_slice(&0u16.to_be_bytes());
    cmd[20..22].copy_from_slice(&(data.len() as u16).to_be_bytes());
    cmd[22..22 + data.len()].copy_from_slice(data);
    state.send_command(&cmd[..cmd_len])?;
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
