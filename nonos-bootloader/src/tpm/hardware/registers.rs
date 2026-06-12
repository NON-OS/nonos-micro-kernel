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

pub fn read_status(device: &TmpDevice) -> TmpResult<u32> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    let base = device.base_addr + (device.locality as u64 * 0x1000);
    Ok(unsafe { core::ptr::read_volatile((base + 0x18) as *const u32) })
}

pub fn write_data(device: &TmpDevice, data: &[u8]) -> TmpResult<()> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if data.len() > 4096 {
        return Err(TmpError::BadParameter);
    }
    let base = device.base_addr + (device.locality as u64 * 0x1000);
    for chunk in data.chunks(4) {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        unsafe {
            core::ptr::write_volatile((base + 0x24) as *mut u32, u32::from_le_bytes(word));
        }
    }
    Ok(())
}

pub fn read_data(device: &TmpDevice, length: usize) -> TmpResult<[u8; 4096]> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if length > 4096 {
        return Err(TmpError::BadParameter);
    }
    let base = device.base_addr + (device.locality as u64 * 0x1000);
    let mut data = [0u8; 4096];
    for i in (0..length).step_by(4) {
        let value = unsafe { core::ptr::read_volatile((base + 0x24) as *const u32) };
        let end = core::cmp::min(i + 4, length);
        data[i..end].copy_from_slice(&value.to_le_bytes()[..end - i]);
    }
    Ok(data)
}
