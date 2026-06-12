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

use crate::tpm::core::{TmpDevice, TmpError, TmpResult};
use crate::tpm::types::Session;

pub fn create_session(device: &mut TmpDevice, session_type: u8) -> TmpResult<Session> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if device.session_count >= device.max_sessions {
        return Err(TmpError::ResourceUnavailable);
    }

    let mut cmd = [0u8; 30];
    cmd[0..10].copy_from_slice(&[0x80, 0x01, 0x00, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x01, 0x76]);
    cmd[10..14].copy_from_slice(&0x40000007u32.to_be_bytes());
    cmd[14..18].copy_from_slice(&0u32.to_be_bytes());
    cmd[18] = session_type;
    cmd[19..21].copy_from_slice(&0x0010u16.to_be_bytes());
    cmd[21..23].copy_from_slice(&0x000Bu16.to_be_bytes());
    cmd[23..25].copy_from_slice(&16u16.to_be_bytes());
    for i in 25..30 {
        cmd[i] = (i - 25) as u8;
    }

    let response = crate::tpm::hardware::send_command(device, &cmd)?;
    if response.len() < 20 {
        return Err(TmpError::InvalidResponse);
    }

    let handle = u32::from_be_bytes([response[10], response[11], response[12], response[13]]);
    device.session_count += 1;

    Ok(Session { handle: handle.into(), session_type, active: true, policy_digest: [0u8; 32] })
}
