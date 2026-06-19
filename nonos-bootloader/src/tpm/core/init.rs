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

use super::device::TmpDevice;
use super::error::{TmpError, TmpResult};

pub fn initialize_tpm() -> TmpResult<TmpDevice> {
    let base_addr = detect_device()?;
    let mut device = TmpDevice::new(base_addr);
    validate_device(&device)?;
    startup_sequence(&mut device)?;
    device.active = true;
    Ok(device)
}

fn detect_device() -> TmpResult<u64> {
    let addrs = [0xFED40000, 0xFED41000, 0xFED42000, 0xFED43000, 0xFED44000];
    for &addr in &addrs {
        let did_vid = unsafe { core::ptr::read_volatile((addr + 0xF00) as *const u32) };
        if did_vid != 0 && did_vid != 0xFFFFFFFF {
            return Ok(addr);
        }
    }
    Err(TmpError::DeviceNotFound)
}

fn validate_device(device: &TmpDevice) -> TmpResult<()> {
    let did_vid = unsafe { core::ptr::read_volatile((device.base_addr + 0xF00) as *const u32) };
    if did_vid == 0 || did_vid == 0xFFFFFFFF {
        Err(TmpError::InvalidResponse)
    } else {
        Ok(())
    }
}

fn startup_sequence(device: &mut TmpDevice) -> TmpResult<()> {
    let startup_cmd = [0x80, 0x01, 0x00, 0x00, 0x00, 0x0C, 0x00, 0x00, 0x01, 0x44, 0x00, 0x00];
    let health_cmd = [0x80, 0x01, 0x00, 0x00, 0x00, 0x0B, 0x00, 0x00, 0x01, 0x43, 0x01];
    execute_command(device, &startup_cmd)?;
    execute_command(device, &health_cmd)?;
    device.capabilities = 0x1F;
    Ok(())
}

fn execute_command(device: &mut TmpDevice, cmd: &[u8]) -> TmpResult<()> {
    crate::tpm::hardware::acquire_locality(device, 0)?;
    let _response = crate::tpm::hardware::send_command(device, cmd)?;
    Ok(())
}
