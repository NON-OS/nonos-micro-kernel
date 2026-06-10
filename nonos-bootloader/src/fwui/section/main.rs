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
            theme::TEXT,
            b"NONOS stage-0 UEFI loader version.",
        ),
        info(b"PROCESSOR", cpu, theme::TEXT, b"CPU brand string from CPUID extended leaves."),
        info(
            b"LOGICAL CPUS",
            format!("{}", logical),
            theme::TEXT,
            b"Logical processors visible to the platform.",
        ),
        info(
            b"TOTAL MEMORY",
            format!("{} MB", sys.mem_mib()),
            theme::TEXT,
            b"Usable RAM summed from the UEFI memory map.",
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
