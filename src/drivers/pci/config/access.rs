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

//! Validated config-space access.
//!
//! What belongs here is the checking and the counting. How an access reaches
//! the bus is `transport`'s problem, which is where the port pair and the ECAM
//! window both live. Before that split this file carried six copies of the
//! same 0xCF8/0xCFC sequence and could only ever run on a PC.

use core::sync::atomic::{AtomicU64, Ordering};

use super::super::constants::*;
use super::super::error::{PciError, Result};
use super::transport;

pub static CONFIG_READS: AtomicU64 = AtomicU64::new(0);
pub static CONFIG_WRITES: AtomicU64 = AtomicU64::new(0);

pub fn validate_access(_bus: u8, device: u8, function: u8, offset: u16) -> Result<()> {
    if device > PCI_MAX_DEVICE {
        return Err(PciError::InvalidDevice(device));
    }
    if function > PCI_MAX_FUNCTION {
        return Err(PciError::InvalidFunction(function));
    }
    if offset >= PCI_CONFIG_SPACE_SIZE {
        return Err(PciError::InvalidOffset(offset));
    }
    Ok(())
}

pub fn validate_alignment(offset: u16, size: u8) -> Result<()> {
    if (offset & ((size as u16) - 1)) != 0 {
        return Err(PciError::UnalignedAccess { offset, alignment: size });
    }
    Ok(())
}

/// The word the legacy address port takes. Kept because the scan path reports
/// it in diagnostics; it means nothing to an ECAM access.
#[inline]
pub fn make_config_address(bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    transport::config_address(bus, device, function, offset)
}

pub fn read8(bus: u8, device: u8, function: u8, offset: u16) -> Result<u8> {
    validate_access(bus, device, function, offset)?;
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);
    transport::read8(bus, device, function, offset)
}

pub fn read16(bus: u8, device: u8, function: u8, offset: u16) -> Result<u16> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 2)?;
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);
    transport::read16(bus, device, function, offset)
}

pub fn read32(bus: u8, device: u8, function: u8, offset: u16) -> Result<u32> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 4)?;
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);
    transport::read32(bus, device, function, offset)
}

pub fn write8(bus: u8, device: u8, function: u8, offset: u16, value: u8) -> Result<()> {
    validate_access(bus, device, function, offset)?;
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);
    transport::write8(bus, device, function, offset, value)
}

pub fn write16(bus: u8, device: u8, function: u8, offset: u16, value: u16) -> Result<()> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 2)?;
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);
    transport::write16(bus, device, function, offset, value)
}

pub fn write32(bus: u8, device: u8, function: u8, offset: u16, value: u32) -> Result<()> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 4)?;
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);
    transport::write32(bus, device, function, offset, value)
}

/// Scan-path read that skips validation because the caller already bounded the
/// offset to a byte. An absent function reads as all-ones, which is how the
/// bus reports one and what the scan is looking for, so an unreachable config
/// space collapses into the same answer rather than a new error case.
#[inline]
pub fn read32_unchecked(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);
    transport::read32(bus, device, function, offset as u16).unwrap_or(!0)
}

#[inline]
pub fn write32_unchecked(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);
    let _ = transport::write32(bus, device, function, offset as u16, value);
}

pub fn get_config_stats() -> (u64, u64) {
    (CONFIG_READS.load(Ordering::Relaxed), CONFIG_WRITES.load(Ordering::Relaxed))
}

pub fn reset_config_stats() {
    CONFIG_READS.store(0, Ordering::Relaxed);
    CONFIG_WRITES.store(0, Ordering::Relaxed);
}
