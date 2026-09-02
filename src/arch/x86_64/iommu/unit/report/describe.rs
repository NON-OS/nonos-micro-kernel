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

//! One line describing the unit as the hardware reports it. Facts only. What
//! the kernel then does about them is bring-up's line to print, not this one,
//! because this code runs before anything has been decided.

use super::super::probe::UnitInfo;
use crate::sys::serial;

pub(super) fn unit(count: usize, info: &UnitInfo) {
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
        // is not protection we can reason about, and bring-up will refuse the
        // unit rather than swap a root table out from under live transfers.
        serial::print(b" te=firmware");
    }
    serial::println(b"");
}
