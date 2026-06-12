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

//! `MkPciConfigWrite` handler. Cap-gated by `Capability::Driver`
//! and a live device claim; the broker authority module enforces
//! the allowlist and the actual PCI write.

use super::errnos::{ERRNO_INVAL, ERRNO_NODEV, ERRNO_PERM, ERRNO_STALE};
use crate::capabilities::Capability;
use crate::hardware::broker::{
    pci_config_read, pci_config_write, PciReadError, PciReadRequest, PciWriteError, PciWriteRequest,
};
use crate::process::{caps, current_pid};

pub fn sys_pci_config_read(device_id: u64, claim_epoch: u64, offset: u32, width: u32) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if !caps::has(pid, Capability::Driver.bit()) {
        return ERRNO_PERM;
    }
    let req = PciReadRequest { device_id, claim_epoch, offset, width };
    match pci_config_read(pid, req) {
        Ok(value) => value as i64,
        Err(e) => read_errno(e),
    }
}

pub fn sys_pci_config_write(device_id: u64, claim_epoch: u64, offset: u32, value: u32) -> i64 {
    let pid = match current_pid() {
        Some(p) => p,
        None => return ERRNO_PERM,
    };
    if !caps::has(pid, Capability::Driver.bit()) {
        return ERRNO_PERM;
    }
    let req = PciWriteRequest { device_id, claim_epoch, offset, value: value as u16 };
    match pci_config_write(pid, req) {
        Ok(()) => 0,
        Err(e) => write_errno(e),
    }
}

fn read_errno(e: PciReadError) -> i64 {
    match e {
        PciReadError::NotClaimed => ERRNO_PERM,
        PciReadError::StaleEpoch => ERRNO_STALE,
        PciReadError::NoDeviceHandle | PciReadError::PlatformError => ERRNO_NODEV,
        PciReadError::OffsetNotAllowed | PciReadError::WidthNotAllowed => ERRNO_INVAL,
    }
}

fn write_errno(e: PciWriteError) -> i64 {
    match e {
        PciWriteError::NotClaimed => ERRNO_PERM,
        PciWriteError::StaleEpoch => ERRNO_STALE,
        PciWriteError::NoDeviceHandle | PciWriteError::PlatformError => ERRNO_NODEV,
        PciWriteError::OffsetNotAllowed | PciWriteError::BitsNotAllowed => ERRNO_INVAL,
    }
}
