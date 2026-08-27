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

use super::message::reason;
use super::run::bring_up;
use crate::arch::x86_64::iommu::globals::page_levels;
use crate::arch::x86_64::iommu::unit::fault::drain_faults;
use crate::arch::x86_64::iommu::unit::report::probed;
use crate::sys::serial;

/// Never panics, and never claims protection it did not achieve. Identity
/// mapping does not confine a known device; what it buys is that unenumerated
/// devices are denied. The console says exactly that.
pub fn init() {
    if probed().is_none() {
        return;
    }
    if !cfg!(feature = "nonos-iommu-enforce") {
        serial::println(b"[VT-D] enforcement not built in; DMA is unrestricted");
        return;
    }
    match bring_up() {
        Ok(assigned) => {
            serial::print(b"[VT-D] translation enabled, levels=");
            serial::print_hex(page_levels().unwrap_or(0) as u64);
            serial::print(b" devices=");
            serial::print_hex(assigned as u64);
            serial::println(b"");
            serial::println(b"[VT-D] enumerated devices identity mapped; others denied");
            // Anything recorded before this point came from firmware's own
            // tables and describes a machine we no longer run.
            drain_faults();
        }
        Err(e) => {
            serial::print(b"[VT-D] bring-up failed (");
            serial::print(reason(e));
            serial::println(b"); DMA is unrestricted");
        }
    }
}
