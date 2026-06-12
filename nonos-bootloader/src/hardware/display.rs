// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use super::types::HardwareInfo;
use uefi::cstr16;
use uefi::prelude::*;

pub fn display_hardware_summary(h: &HardwareInfo, st: &mut SystemTable<Boot>) {
    let _ = st.stdout().output_string(cstr16!("=== HW Summary ===\r\n"));
    let _ = st.stdout().output_string(if h.acpi_available {
        cstr16!("ACPI: available\r\n")
    } else {
        cstr16!("ACPI: not found\r\n")
    });
    let _ = st.stdout().output_string(cstr16!("Memory: reported\r\n"));
    let _ = st.stdout().output_string(cstr16!("CPUs: detected\r\n"));
    let _ = st.stdout().output_string(cstr16!("Storage: counted\r\n"));
    let _ = st.stdout().output_string(cstr16!("Network: counted\r\n"));
    let _ = st.stdout().output_string(cstr16!("Graphics: counted\r\n"));
    let _ = st.stdout().output_string(cstr16!("PCI: counted\r\n"));
    let _ = st.stdout().output_string(cstr16!("==================\r\n"));
}
