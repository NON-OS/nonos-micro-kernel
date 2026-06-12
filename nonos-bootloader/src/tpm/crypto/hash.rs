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

pub fn compute_hash(device: &mut TmpDevice, data: &[u8], algorithm: u16) -> TmpResult<[u8; 32]> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if data.len() > 1024 {
        return Err(TmpError::BadParameter);
    }

    let mut cmd = [0u8; 1024];
    cmd[0..10].copy_from_slice(&[0x80, 0x01, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x01, 0x7D]);
    cmd[10..12].copy_from_slice(&algorithm.to_be_bytes());
    cmd[12..14].copy_from_slice(&(data.len() as u16).to_be_bytes());
    cmd[14..14 + data.len()].copy_from_slice(data);

    let response = crate::tpm::hardware::send_command(device, &cmd[..14 + data.len()])?;
    if response.len() < 44 {
        return Err(TmpError::InvalidResponse);
    }

    let mut hash = [0u8; 32];
    hash.copy_from_slice(&response[12..44]);
    Ok(hash)
}
