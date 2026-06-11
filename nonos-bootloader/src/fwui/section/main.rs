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
use crate::fwui::data::{fmt_date, fmt_time, Sys};
use crate::fwui::theme;
use alloc::format;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use uefi::table::runtime::Time;

pub fn main(sys: &Sys, time: &Time) -> Vec<Row> {
    let cpu = if sys.cpu.brand.is_empty() { sys.cpu.vendor.clone() } else { sys.cpu.brand.clone() };
    let logical = if sys.cpu.logical > 0 { sys.cpu.logical as usize } else { sys.cpu_count };
    let mut prot: Vec<&str> = Vec::new();
    if sys.feat.nxe {
        prot.push("NX");
    }
    if sys.feat.smep {
        prot.push("SMEP");
    }
    if sys.feat.smap {
        prot.push("SMAP");
    }
    if sys.feat.umip {
        prot.push("UMIP");
    }
    let protections = if prot.is_empty() { "none reported".to_string() } else { prot.join(" ") };
    vec![
        info(
            b"FIRMWARE VENDOR",
            sys.fw_vendor.clone(),
            theme::TEXT,
            b"Identity string reported by the platform firmware.",
        ),
        info(
            b"UEFI VERSION",
            format!("{}.{:02}", sys.uefi_major, sys.uefi_minor),
            theme::ACCENT,
            b"UEFI specification revision implemented by the firmware.",
        ),
        info(
            b"FIRMWARE REVISION",
            format!("0x{:08X}", sys.fw_rev),
            theme::TEXT,
            b"Vendor firmware build revision.",
        ),
        info(
            b"BOOTLOADER",
            format!("v{}", sys.boot_ver),
            theme::ACCENT,
            b"NONOS stage-0 UEFI loader version.",
        ),
        info(b"PROCESSOR", cpu, theme::TEXT, b"CPU brand string from CPUID extended leaves."),
        info(
            b"VENDOR",
            sys.cpu.vendor.clone(),
            theme::TEXT,
            b"CPU vendor identification string from CPUID leaf 0.",
        ),
        info(
            b"LOGICAL CPUS",
            format!("{}", logical),
            theme::TEXT,
            b"Logical processors visible to the platform.",
        ),
        info(
            b"CPU PROTECTIONS",
            protections,
            theme::ACCENT,
            b"Hardware protection features the loader confirmed active.",
        ),
        info(
            b"TOTAL MEMORY",
            format!("{} MB", sys.mem_mib()),
            theme::TEXT,
            b"Usable RAM summed from the UEFI memory map.",
        ),
        info(
            b"ACPI",
            if sys.acpi { "present".to_string() } else { "absent".to_string() },
            theme::TEXT,
            b"Whether the firmware exposes ACPI configuration tables.",
        ),
        info(
            b"PCI DEVICES",
            format!("{}", sys.pci),
            theme::TEXT,
            b"PCI functions enumerated on the platform bus.",
        ),
        info(
            b"STORAGE",
            format!("{}", sys.storage),
            theme::TEXT,
            b"Block storage controllers seen during enumeration.",
        ),
        info(
            b"NETWORK",
            format!("{}", sys.net),
            theme::TEXT,
            b"Network interfaces seen during enumeration.",
        ),
        info(
            b"GRAPHICS",
            format!("{}", sys.gpu),
            theme::TEXT,
            b"Display adapters seen during enumeration.",
        ),
        info(
            b"SYSTEM DATE",
            fmt_date(time),
            theme::TEXT,
            b"Current date from the platform real-time clock.",
        ),
        info(
            b"SYSTEM TIME",
            fmt_time(time),
            theme::ACCENT,
            b"Current time from the platform real-time clock.",
        ),
        info(
            b"ACCESS LEVEL",
            "ADMINISTRATOR".to_string(),
            theme::ACCENT,
            b"Pre-OS firmware access level.",
        ),
    ]
}
