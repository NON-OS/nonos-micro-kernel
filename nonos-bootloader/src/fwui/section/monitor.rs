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

use super::info::info;
use super::row::Row;
use crate::fwui::data::Sys;
use crate::fwui::theme;
use alloc::format;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

pub fn monitor(sys: &Sys) -> Vec<Row> {
    let vendor =
        if sys.cpu.vendor.is_empty() { "unknown".to_string() } else { sys.cpu.vendor.clone() };
    let (acpi, acol) = if sys.acpi {
        ("PRESENT".to_string(), theme::OK)
    } else {
        ("ABSENT".to_string(), theme::WARN)
    };
    vec![
        info(b"CPU VENDOR", vendor, theme::TEXT, b"CPU vendor identification from CPUID leaf 0."),
        info(
            b"CPU CORES",
            format!("{}", sys.cpu_count),
            theme::TEXT,
            b"Processor count derived from ACPI MADT.",
        ),
        info(b"ACPI TABLES", acpi, acol, b"ACPI RSDP discovery result."),
        info(b"PCI DEVICES", format!("{}", sys.pci), theme::TEXT, b"Enumerated PCI(e) devices."),
        info(
            b"STORAGE DEVICES",
            format!("{}", sys.storage),
            theme::TEXT,
            b"Block I/O storage handles discovered.",
        ),
        info(
            b"NETWORK IFACES",
            format!("{}", sys.net),
            theme::TEXT,
            b"Network interface handles discovered.",
        ),
        info(
            b"GRAPHICS DEVICES",
            format!("{}", sys.gpu),
            theme::TEXT,
            b"Graphics output handles discovered.",
        ),
    ]
}
