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

use crate::handoff::timing::read_tsc;

pub fn estimate_tsc_frequency(bs: &uefi::table::boot::BootServices) -> u64 {
    let tsc_start = read_tsc();
    let _ = bs.stall(10_000);
    let tsc_end = read_tsc();
    if tsc_end > tsc_start {
        (tsc_end - tsc_start) * 100
    } else {
        2_000_000_000
    }
}
