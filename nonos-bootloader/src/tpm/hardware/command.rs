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

pub fn send_command(device: &mut TmpDevice, command: &[u8]) -> TmpResult<[u8; 4096]> {
    if !device.active {
        return Err(TmpError::DeviceNotFound);
    }
    if command.len() < 10 || command.len() > 4096 {
        return Err(TmpError::BadParameter);
    }
    let base = device.base_addr + (device.locality as u64 * 0x1000);
    wait_ready(base)?;
    write_command(base, command)?;
    execute(base)?;
    wait_done(base)?;
    read_response(base)
}

fn wait_ready(base: u64) -> TmpResult<()> {
    for _ in 0..1000 {
        let status = unsafe { core::ptr::read_volatile((base + 0x18) as *const u32) };
        if (status & 0x40) != 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(TmpError::CommandTimeout)
}

fn write_command(base: u64, data: &[u8]) -> TmpResult<()> {
    for chunk in data.chunks(4) {
        let mut word = [0u8; 4];
        word[..chunk.len()].copy_from_slice(chunk);
        unsafe {
            core::ptr::write_volatile((base + 0x24) as *mut u32, u32::from_le_bytes(word));
        }
    }
    Ok(())
}

fn execute(base: u64) -> TmpResult<()> {
    unsafe {
        core::ptr::write_volatile((base + 0x18) as *mut u32, 0x20);
    }
    Ok(())
}

fn wait_done(base: u64) -> TmpResult<()> {
    for _ in 0..10000 {
        let status = unsafe { core::ptr::read_volatile((base + 0x18) as *const u32) };
        if (status & 0x90) != 0 {
            return Ok(());
        }
        core::hint::spin_loop();
    }
    Err(TmpError::CommandTimeout)
}

fn read_response(base: u64) -> TmpResult<[u8; 4096]> {
    let mut response = [0u8; 4096];
    for i in (0..4096).step_by(4) {
        let value = unsafe { core::ptr::read_volatile((base + 0x24) as *const u32) };
        response[i..i + 4].copy_from_slice(&value.to_le_bytes());
    }
    Ok(response)
}
