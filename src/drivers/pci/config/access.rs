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

use core::sync::atomic::{AtomicU64, Ordering};

use spin::Mutex;

use super::super::constants::*;
use super::super::error::{PciError, Result};
use super::port_io::{inl, outl};

pub static CONFIG_READS: AtomicU64 = AtomicU64::new(0);
pub static CONFIG_WRITES: AtomicU64 = AtomicU64::new(0);

// Serialise the (PCI_CONFIG_ADDRESS, PCI_CONFIG_DATA) port pair. On
// SMP, an interleaving between one CPU's address write and the other
// CPU's data read returns garbage from the wrong device. The lock is
// held only across the two outl/inl that form one PCI config word
// access, so contention is bounded by the port-IO latency.
static CONFIG_PORT_LOCK: Mutex<()> = Mutex::new(());

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

#[inline]
pub fn make_config_address(bus: u8, device: u8, function: u8, offset: u16) -> u32 {
    (1u32 << 31)
        | ((bus as u32) << 16)
        | ((device as u32) << 11)
        | ((function as u32) << 8)
        | ((offset as u32) & 0xFC)
}

pub fn read8(bus: u8, device: u8, function: u8, offset: u16) -> Result<u8> {
    validate_access(bus, device, function, offset)?;
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);

    let addr = make_config_address(bus, device, function, offset);
    let byte_offset = (offset & 3) as u16;

    let _g = CONFIG_PORT_LOCK.lock();
    // SAFETY: PCI config space ports are valid for kernel access; the
    // lock guarantees the (address, data) pair is atomic.
    let value = unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        let data = inl(PCI_CONFIG_DATA);
        ((data >> (byte_offset * 8)) & 0xFF) as u8
    };

    Ok(value)
}

pub fn read16(bus: u8, device: u8, function: u8, offset: u16) -> Result<u16> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 2)?;
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);

    let addr = make_config_address(bus, device, function, offset);
    let word_offset = (offset & 2) as u16;

    let _g = CONFIG_PORT_LOCK.lock();
    // SAFETY: PCI config space ports are valid for kernel access; the
    // lock guarantees the (address, data) pair is atomic.
    let value = unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        let data = inl(PCI_CONFIG_DATA);
        ((data >> (word_offset * 8)) & 0xFFFF) as u16
    };

    Ok(value)
}

pub fn read32(bus: u8, device: u8, function: u8, offset: u16) -> Result<u32> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 4)?;
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);

    let addr = make_config_address(bus, device, function, offset);

    let _g = CONFIG_PORT_LOCK.lock();
    // SAFETY: PCI config space ports are valid for kernel access; the
    // lock guarantees the (address, data) pair is atomic.
    let value = unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        inl(PCI_CONFIG_DATA)
    };

    Ok(value)
}

pub fn write8(bus: u8, device: u8, function: u8, offset: u16, value: u8) -> Result<()> {
    validate_access(bus, device, function, offset)?;
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);

    let addr = make_config_address(bus, device, function, offset);
    let byte_offset = (offset & 3) as u32;
    let mask = 0xFFu32 << (byte_offset * 8);

    // SAFETY: PCI config space ports are valid for kernel access
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        let current = inl(PCI_CONFIG_DATA);
        let new_value = (current & !mask) | ((value as u32) << (byte_offset * 8));
        outl(PCI_CONFIG_DATA, new_value);
    }

    Ok(())
}

pub fn write16(bus: u8, device: u8, function: u8, offset: u16, value: u16) -> Result<()> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 2)?;
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);

    let addr = make_config_address(bus, device, function, offset);
    let word_offset = (offset & 2) as u32;
    let mask = 0xFFFFu32 << (word_offset * 8);

    // SAFETY: PCI config space ports are valid for kernel access
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        let current = inl(PCI_CONFIG_DATA);
        let new_value = (current & !mask) | ((value as u32) << (word_offset * 8));
        outl(PCI_CONFIG_DATA, new_value);
    }

    Ok(())
}

pub fn write32(bus: u8, device: u8, function: u8, offset: u16, value: u32) -> Result<()> {
    validate_access(bus, device, function, offset)?;
    validate_alignment(offset, 4)?;
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);

    let addr = make_config_address(bus, device, function, offset);

    // SAFETY: PCI config space ports are valid for kernel access
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        outl(PCI_CONFIG_DATA, value);
    }

    Ok(())
}

#[inline]
pub fn read32_unchecked(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    CONFIG_READS.fetch_add(1, Ordering::Relaxed);
    let addr = pci_config_address(bus, device, function, offset);
    // SAFETY: PCI config space ports are valid for kernel access
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        inl(PCI_CONFIG_DATA)
    }
}

#[inline]
pub fn write32_unchecked(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    CONFIG_WRITES.fetch_add(1, Ordering::Relaxed);
    let addr = pci_config_address(bus, device, function, offset);
    // SAFETY: PCI config space ports are valid for kernel access
    unsafe {
        outl(PCI_CONFIG_ADDRESS, addr);
        outl(PCI_CONFIG_DATA, value);
    }
}

pub fn get_config_stats() -> (u64, u64) {
    (CONFIG_READS.load(Ordering::Relaxed), CONFIG_WRITES.load(Ordering::Relaxed))
}

pub fn reset_config_stats() {
    CONFIG_READS.store(0, Ordering::Relaxed);
    CONFIG_WRITES.store(0, Ordering::Relaxed);
}
