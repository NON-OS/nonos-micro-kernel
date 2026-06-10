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
use crate::tpm::types::NvIndex;

pub fn nv_read(device: &mut TmpDevice, index: NvIndex, length: u16) -> TmpResult<[u8; 256]> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if length > 256 {
        return Err(TmpError::BadParameter);
    }
    let mut cmd = [0u8; 22];
    cmd[0..10].copy_from_slice(&[0x80, 0x01, 0x00, 0x00, 0x00, 0x16, 0x00, 0x00, 0x01, 0x4E]);
    cmd[10..14].copy_from_slice(&index.value.to_be_bytes());
    cmd[14..18].copy_from_slice(&index.value.to_be_bytes());
    cmd[18..20].copy_from_slice(&length.to_be_bytes());
    let response = crate::tpm::hardware::send_command(device, &cmd)?;
    if response.len() < (12 + length as usize) {
        return Err(TmpError::InvalidResponse);
    }
    let mut data = [0u8; 256];
    let actual_length = u16::from_be_bytes([response[10], response[11]]) as usize;
    if actual_length > 0 && actual_length <= 256 {
        data[..actual_length].copy_from_slice(&response[12..12 + actual_length]);
    }
    Ok(data)
}

pub fn nv_write(device: &mut TmpDevice, index: NvIndex, data: &[u8]) -> TmpResult<()> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if data.len() > 256 {
        return Err(TmpError::BadParameter);
    }
    let mut cmd = [0u8; 512];
    cmd[0..10].copy_from_slice(&[0x80, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x37]);
    cmd[10..14].copy_from_slice(&index.value.to_be_bytes());
    cmd[14..18].copy_from_slice(&index.value.to_be_bytes());
    cmd[18..20].copy_from_slice(&(data.len() as u16).to_be_bytes());
    cmd[20..20 + data.len()].copy_from_slice(data);
    let total_len = 22 + data.len();
    cmd[4..6].copy_from_slice(&(total_len as u16).to_be_bytes());
    let response = crate::tpm::hardware::send_command(device, &cmd[..total_len])?;
    if response.len() < 10 {
        return Err(TmpError::InvalidResponse);
    }
    Ok(())
}
