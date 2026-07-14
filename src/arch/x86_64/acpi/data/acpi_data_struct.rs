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

use alloc::vec::Vec;

use super::interrupt::{InterruptOverride, NmiConfig};
use super::ioapic::IoApicInfo;
use super::numa::NumaMemoryRegion;
use super::pcie::PcieSegment;
use super::processor::ProcessorInfo;
use crate::arch::x86_64::acpi::tables::{GenericAddress, PmProfile};

#[derive(Debug)]
pub struct AcpiData {
    pub revision: u8,
    pub oem_id: [u8; 6],
    pub lapic_address: u64,
    pub has_legacy_pics: bool,
    // Reflects the FADT IA-PC boot architecture HAS_8042 flag. Defaults
    // to true so a missing or unparsed FADT keeps the i8042 present.
    pub has_8042: bool,
    pub processors: Vec<ProcessorInfo>,
    pub ioapics: Vec<IoApicInfo>,
    pub overrides: Vec<InterruptOverride>,
    pub nmis: Vec<NmiConfig>,
    pub numa_regions: Vec<NumaMemoryRegion>,
    pub pcie_segments: Vec<PcieSegment>,
    pub hpet_address: Option<u64>,
    pub pm1a_control: u32,
    pub pm1b_control: u32,
    pub slp_typ: [u8; 6],
    pub reset_reg: Option<GenericAddress>,
    pub reset_value: u8,
    pub pm_profile: PmProfile,
    pub sci_interrupt: u16,
}

impl Default for AcpiData {
    fn default() -> Self {
        Self::new()
    }
}
