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

//! Probing the remapping units at boot and saying on the console what was
//! found. Read-only. Nothing here enables translation, and the line printed
//! says so, because a machine that reports an IOMMU it is not using should not
//! read as a machine that is protected.

use spin::Once;

use super::probe::{probe_first, unit_count, ProbeError, UnitInfo};
use crate::sys::serial;

static PROBED: Once<UnitInfo> = Once::new();

/// What the first remapping unit reported, once probed.
pub fn probed() -> Option<&'static UnitInfo> {
    PROBED.get()
}

/// Probe and report. Called once, after ACPI parsing.
pub fn init() {
    let count = unit_count();
    if count == 0 {
        serial::println(b"[VT-D] no remapping units in DMAR; DMA is unrestricted");
        return;
    }

    let info = match probe_first() {
        Ok(info) => info,
        Err(e) => {
            serial::print(b"[VT-D] probe failed (");
            serial::print(match e {
                ProbeError::NoUnits => b"no units".as_slice(),
                ProbeError::MapFailed => b"register window not mappable".as_slice(),
                ProbeError::NoUsableAgaw => b"no supported paging depth".as_slice(),
            });
            serial::println(b"); DMA is unrestricted");
            return;
        }
    };

    serial::print(b"[VT-D] units=");
    serial::print_hex(count as u64);
    serial::print(b" ver=");
    serial::print_hex(info.version as u64);
    serial::print(b" domains=");
    serial::print_hex(info.domains as u64);
    serial::print(b" gaw=");
    serial::print_hex(info.max_address_width as u64);
    serial::print(b" levels=");
    serial::print_hex(info.levels.page_table_levels() as u64);
    if info.caching_mode {
        serial::print(b" cm");
    }
    if info.requires_write_buffer_flush {
        serial::print(b" rwbf");
    }
    if info.translation_enabled {
        // Firmware left it on with its own tables. We do not own them, so this
        // is not protection we can reason about.
        serial::print(b" te=firmware");
    }
    serial::println(b"");
    serial::println(b"[VT-D] translation not programmed by this kernel; DMA is unrestricted");

    PROBED.call_once(|| info);
}
