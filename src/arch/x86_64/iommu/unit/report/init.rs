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
//! found. Read-only: nothing here enables translation. Whether DMA ends up
//! confined is bring-up's verdict to print, and it prints one on every path,
//! so this module never states a posture it is too early to know.

use super::super::probe::{probe_first, unit_count};
use super::{describe, failure, state};
use crate::sys::serial;

/// Probe and report. Called once, after ACPI parsing has published the DRHD
/// bases and the MMIO mapper can hand out a register window.
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
            serial::print(failure::reason(e));
            serial::println(b"); DMA is unrestricted");
            return;
        }
    };

    describe::unit(count, &info);
    state::record(info);
}
