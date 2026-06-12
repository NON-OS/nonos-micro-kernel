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

pub fn get_random(device: &mut TmpDevice, length: u16) -> TmpResult<[u8; 32]> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if length == 0 || length > 32 {
        return Err(TmpError::BadParameter);
    }

    let mut cmd = [0u8; 12];
    cmd[0..10].copy_from_slice(&[0x80, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x01, 0x7B]);
    cmd[10..12].copy_from_slice(&length.to_be_bytes());

    let response = crate::tpm::hardware::send_command(device, &cmd)?;
    if response.len() < (12 + length as usize) {
        return Err(TmpError::InvalidResponse);
    }

    let mut random_data = [0u8; 32];
    let actual_length = u16::from_be_bytes([response[10], response[11]]) as usize;
    if actual_length > 0 && actual_length <= 32 {
        random_data[..actual_length].copy_from_slice(&response[12..12 + actual_length]);
    }
    Ok(random_data)
}
