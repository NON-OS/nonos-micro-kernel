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

use super::state::TABLES;
use crate::arch::x86_64::acpi::data::*;
use crate::arch::x86_64::acpi::tables::PmProfile;

pub fn revision() -> Option<u8> {
    TABLES.read().as_ref().map(|t| t.data.revision)
}

pub fn oem_id() -> Option<[u8; 6]> {
    TABLES.read().as_ref().map(|t| t.data.oem_id)
}

pub fn lapic_address() -> Option<u64> {
    TABLES.read().as_ref().map(|t| t.data.lapic_address)
}

pub fn has_legacy_pics() -> Option<bool> {
    TABLES.read().as_ref().map(|t| t.data.has_legacy_pics)
}

pub fn has_8042() -> Option<bool> {
    TABLES.read().as_ref().map(|t| t.data.has_8042)
}

pub fn processors() -> alloc::vec::Vec<ProcessorInfo> {
    TABLES.read().as_ref().map(|t| t.data.processors.clone()).unwrap_or_default()
}

pub fn ioapics() -> alloc::vec::Vec<IoApicInfo> {
    TABLES.read().as_ref().map(|t| t.data.ioapics.clone()).unwrap_or_default()
}

pub fn interrupt_overrides() -> alloc::vec::Vec<InterruptOverride> {
    TABLES.read().as_ref().map(|t| t.data.overrides.clone()).unwrap_or_default()
}

pub fn nmi_configs() -> alloc::vec::Vec<NmiConfig> {
    TABLES.read().as_ref().map(|t| t.data.nmis.clone()).unwrap_or_default()
}

pub fn numa_regions() -> alloc::vec::Vec<NumaMemoryRegion> {
    TABLES.read().as_ref().map(|t| t.data.numa_regions.clone()).unwrap_or_default()
}

pub fn pcie_segments() -> alloc::vec::Vec<PcieSegment> {
    TABLES.read().as_ref().map(|t| t.data.pcie_segments.clone()).unwrap_or_default()
}

pub fn hpet_address() -> Option<u64> {
    TABLES.read().as_ref().and_then(|t| t.data.hpet_address)
}

pub fn pm_profile() -> Option<PmProfile> {
    TABLES.read().as_ref().map(|t| t.data.pm_profile)
}

pub fn sci_interrupt() -> Option<u16> {
    TABLES.read().as_ref().map(|t| t.data.sci_interrupt)
}
