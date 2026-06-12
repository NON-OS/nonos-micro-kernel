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

const ACCESS_VALID: u8 = 0x80;
const ACCESS_REQUEST_USE: u8 = 0x02;
const ACCESS_ACTIVE_LOCALITY: u8 = 0x20;

pub fn acquire_locality(device: &mut TmpDevice, locality: u8) -> TmpResult<()> {
    if locality > 4 {
        return Err(TmpError::BadParameter);
    }

    let locality_base = device.base_addr + (locality as u64 * 0x1000);
    request_locality(locality_base)?;
    wait_for_locality(locality_base)?;
    device.locality = locality;
    Ok(())
}

pub fn release_locality(device: &mut TmpDevice) -> TmpResult<()> {
    let locality_base = device.base_addr + (device.locality as u64 * 0x1000);
    unsafe {
        core::ptr::write_volatile((locality_base + 0x008) as *mut u8, ACCESS_ACTIVE_LOCALITY);
    }
    device.locality = 0xFF;
    Ok(())
}

fn request_locality(base: u64) -> TmpResult<()> {
    unsafe {
        core::ptr::write_volatile((base + 0x008) as *mut u8, ACCESS_REQUEST_USE);
    }
    Ok(())
}

fn wait_for_locality(base: u64) -> TmpResult<()> {
    for _ in 0..1000 {
        let access = unsafe { core::ptr::read_volatile((base + 0x008) as *const u8) };
        if (access & ACCESS_VALID) != 0 && (access & ACCESS_ACTIVE_LOCALITY) != 0 {
            return Ok(());
        }
        for _ in 0..1000 {
            core::hint::spin_loop();
        }
    }
    Err(TmpError::LocalityTimeout)
}
