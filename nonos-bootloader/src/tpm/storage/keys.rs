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
use crate::tpm::types::TmpHandle;

pub fn create_key(device: &mut TmpDevice, key_size: u16) -> TmpResult<TmpHandle> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if key_size != 2048 && key_size != 3072 {
        return Err(TmpError::BadParameter);
    }
    let mut cmd = [0u8; 64];
    cmd[0..10].copy_from_slice(&[0x80, 0x02, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x01, 0x31]);
    cmd[10..14].copy_from_slice(&0x40000001u32.to_be_bytes());
    cmd[14..18].copy_from_slice(&0x00000100u32.to_be_bytes());
    cmd[18..20].copy_from_slice(&key_size.to_be_bytes());
    cmd[20..22].copy_from_slice(&0x0001u16.to_be_bytes());
    cmd[22..24].copy_from_slice(&0x000Bu16.to_be_bytes());
    for i in 24..64 {
        cmd[i] = ((i - 24) & 0xFF) as u8;
    }
    let response = crate::tpm::hardware::send_command(device, &cmd)?;
    if response.len() < 14 {
        return Err(TmpError::InvalidResponse);
    }
    let handle = u32::from_be_bytes([response[10], response[11], response[12], response[13]]);
    Ok(TmpHandle::new(handle))
}

pub fn load_key(
    device: &mut TmpDevice,
    parent: TmpHandle,
    key_data: &[u8],
) -> TmpResult<TmpHandle> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if key_data.len() > 512 {
        return Err(TmpError::BadParameter);
    }
    let mut cmd = [0u8; 1024];
    cmd[0..10].copy_from_slice(&[0x80, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x57]);
    cmd[10..14].copy_from_slice(&parent.value.to_be_bytes());
    cmd[14..16].copy_from_slice(&(key_data.len() as u16).to_be_bytes());
    cmd[16..16 + key_data.len()].copy_from_slice(key_data);
    let total_len = 16 + key_data.len();
    cmd[4..6].copy_from_slice(&(total_len as u16).to_be_bytes());
    let response = crate::tpm::hardware::send_command(device, &cmd[..total_len])?;
    if response.len() < 14 {
        return Err(TmpError::InvalidResponse);
    }
    let handle = u32::from_be_bytes([response[10], response[11], response[12], response[13]]);
    Ok(TmpHandle::new(handle))
}
