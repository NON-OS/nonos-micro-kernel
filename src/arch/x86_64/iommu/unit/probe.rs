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

//! Mapping the units DMAR reported and reading what they support. Read-only:
//! nothing here changes how a device reaches memory.

use super::access::{RemapUnit, UNIT_WINDOW};
use crate::arch::x86_64::acpi::parser::other::remap_unit_bases;
use crate::arch::x86_64::iommu::regs::{cap, offsets};
use crate::memory::addr::PhysAddr;

/// What one unit supports, as read from its Capability register.
#[derive(Debug, Clone, Copy)]
pub struct UnitInfo {
    pub unit: RemapUnit,
    pub version: u32,
    pub cap: u64,
    pub ecap: u64,
    pub levels: cap::AgawLevels,
    pub max_address_width: u8,
    pub domains: u32,
    pub caching_mode: bool,
    pub requires_write_buffer_flush: bool,
    /// Whether translation is already on, which firmware sometimes leaves set.
    pub translation_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeError {
    NoUnits,
    MapFailed,
    /// The unit reports no supported paging depth, so it cannot translate.
    NoUsableAgaw,
}

/// Map the first unit DMAR reported and read its capabilities.
///
/// This reads a single unit. A machine with several needs each programmed, and
/// picking only the first would silently leave devices behind the others
/// unconfined, so `probe_all` reports the count and callers must not assume one
/// covers the machine.
pub fn probe_first() -> Result<UnitInfo, ProbeError> {
    let bases = remap_unit_bases();
    let base = *bases.first().ok_or(ProbeError::NoUnits)?;
    probe_at(base)
}

/// How many remapping units the firmware reported.
pub fn unit_count() -> usize {
    remap_unit_bases().len()
}

/// Map one unit at its DRHD base and read what it supports.
pub fn probe_at(base_pa: u64) -> Result<UnitInfo, ProbeError> {
    let va = crate::memory::mmio::map_device_memory(PhysAddr::new(base_pa), UNIT_WINDOW)
        .map_err(|_| ProbeError::MapFailed)?;
    // SAFETY: map_device_memory returned a live uncached mapping of the whole
    // register window over base_pa, and it is not unmapped while in use.
    let unit = unsafe { RemapUnit::from_mapped(va.as_u64(), base_pa) };

    let version = unit.read32(offsets::VER);
    let capability = unit.read64(offsets::CAP);
    let ecap = unit.read64(offsets::ECAP);
    let status = unit.read32(offsets::GSTS);

    let levels = cap::preferred_levels(capability).ok_or(ProbeError::NoUsableAgaw)?;

    Ok(UnitInfo {
        unit,
        version,
        cap: capability,
        ecap,
        levels,
        max_address_width: cap::max_address_width(capability),
        domains: cap::domain_count(capability),
        caching_mode: cap::caching_mode(capability),
        requires_write_buffer_flush: cap::requires_write_buffer_flush(capability),
        translation_enabled: status & offsets::GSTS_TES != 0,
    })
}
