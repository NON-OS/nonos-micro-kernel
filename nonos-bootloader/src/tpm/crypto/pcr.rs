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

pub fn extend_pcr(device: &mut TmpDevice, pcr_index: u32, digest: &[u8; 32]) -> TmpResult<()> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if pcr_index > 23 {
        return Err(TmpError::BadParameter);
    }

    let mut cmd = [0u8; 54];
    cmd[0..10].copy_from_slice(&[0x80, 0x02, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x01, 0x82]);
    cmd[10..14].copy_from_slice(&pcr_index.to_be_bytes());
    cmd[14..18].copy_from_slice(&1u32.to_be_bytes());
    cmd[18..20].copy_from_slice(&0x000Bu16.to_be_bytes());
    cmd[20..22].copy_from_slice(&32u16.to_be_bytes());
    cmd[22..54].copy_from_slice(digest);

    let response = crate::tpm::hardware::send_command(device, &cmd)?;
    if response.len() < 10 {
        return Err(TmpError::InvalidResponse);
    }
    Ok(())
}

pub fn read_pcr(device: &mut TmpDevice, pcr_index: u32) -> TmpResult<[u8; 32]> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if pcr_index > 23 {
        return Err(TmpError::BadParameter);
    }

    let mut cmd = [0u8; 20];
    cmd[0..10].copy_from_slice(&[0x80, 0x01, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x01, 0x7E]);
    cmd[10..14].copy_from_slice(&1u32.to_be_bytes());
    cmd[14..16].copy_from_slice(&0x000Bu16.to_be_bytes());
    cmd[16..20].copy_from_slice(&pcr_index.to_be_bytes());

    let response = crate::tpm::hardware::send_command(device, &cmd)?;
    if response.len() < 42 {
        return Err(TmpError::InvalidResponse);
    }

    let mut pcr_value = [0u8; 32];
    pcr_value.copy_from_slice(&response[10..42]);
    Ok(pcr_value)
}
