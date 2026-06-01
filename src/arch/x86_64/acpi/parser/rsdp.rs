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

use core::ptr;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::arch::x86_64::acpi::error::{AcpiError, AcpiResult};
use crate::arch::x86_64::acpi::tables::rsdp::{self, Rsdp, RsdpExtended};

static RSDP_OVERRIDE: AtomicU64 = AtomicU64::new(0);

pub fn set_rsdp_address(phys: u64) {
    RSDP_OVERRIDE.store(phys, Ordering::SeqCst);
}

pub fn find_rsdp() -> AcpiResult<RsdpExtended> {
    let hinted = RSDP_OVERRIDE.load(Ordering::Relaxed);
    if hinted != 0 {
        if let Some(rsdp) = read_rsdp_at(hinted as usize) {
            return Ok(rsdp);
        }
    }

    unsafe {
        let ebda_segment = ptr::read_volatile(rsdp::EBDA_PTR_ADDR as *const u16);
        if ebda_segment != 0 {
            let ebda_start = (ebda_segment as usize) << 4;
            if let Some(rsdp) = search_rsdp_range(ebda_start, 1024) {
                return Ok(rsdp);
            }
        }
    }

    if let Some(rsdp) = search_rsdp_range(rsdp::BIOS_ROM_START, rsdp::BIOS_ROM_SIZE) {
        return Ok(rsdp);
    }

    Err(AcpiError::RsdpNotFound)
}

fn read_rsdp_at(addr: usize) -> Option<RsdpExtended> {
    if addr == 0 {
        return None;
    }
    let addr = super::phys::directmap(addr as u64)? as usize;
    unsafe {
        let ptr = addr as *const Rsdp;
        let sig = ptr::read_volatile(&(*ptr).signature);
        if sig != rsdp::RSDP_SIGNATURE {
            return None;
        }
        let base = ptr::read_volatile(ptr);
        if !base.validate_checksum() {
            return None;
        }
        if base.is_acpi2() {
            let ext_ptr = addr as *const RsdpExtended;
            let ext = ptr::read_volatile(ext_ptr);
            if ext.validate_extended_checksum() {
                return Some(ext);
            }
            None
        } else {
            Some(RsdpExtended::from_rsdp(base))
        }
    }
}

fn search_rsdp_range(start: usize, length: usize) -> Option<RsdpExtended> {
    for addr in (start..start + length).step_by(rsdp::RSDP_ALIGNMENT) {
        unsafe {
            let ptr = addr as *const Rsdp;
            let sig = ptr::read_volatile(&(*ptr).signature);

            if sig == rsdp::RSDP_SIGNATURE {
                let rsdp = ptr::read_volatile(ptr);

                if !rsdp.validate_checksum() {
                    continue;
                }

                if rsdp.is_acpi2() {
                    let ext_ptr = addr as *const RsdpExtended;
                    let ext_rsdp = ptr::read_volatile(ext_ptr);

                    if ext_rsdp.validate_extended_checksum() {
                        return Some(ext_rsdp);
                    }
                } else {
                    return Some(RsdpExtended::from_rsdp(rsdp));
                }
            }
        }
    }
    None
}
