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

use super::acpi_data_struct::AcpiData;
use crate::arch::x86_64::acpi::tables::PmProfile;

impl AcpiData {
    pub fn new() -> Self {
        Self {
            revision: 0,
            oem_id: [0; 6],
            lapic_address: 0xFEE0_0000,
            has_legacy_pics: true,
            has_8042: true,
            processors: Vec::new(),
            ioapics: Vec::new(),
            overrides: Vec::new(),
            nmis: Vec::new(),
            numa_regions: Vec::new(),
            pcie_segments: Vec::new(),
            hpet_address: None,
            pm1a_control: 0,
            pm1b_control: 0,
            slp_typ: [0; 6],
            reset_reg: None,
            reset_value: 0,
            pm_profile: PmProfile::Unspecified,
            sci_interrupt: 9,
        }
    }
}
