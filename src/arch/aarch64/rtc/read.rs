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

use super::state::base;

/// `RTCDR`, the data register: seconds since the Unix epoch.
const DR: u64 = 0x00;

/// The count is 32 bits, so it wraps in 2106. Reporting a wrapped value as a
/// date in 1970 would be worse than reporting nothing, but there is no way to
/// tell the two apart from this register alone, so the reading is passed
/// through and the clock's own sanity window in `sys::clock` rejects it.
pub fn unix_timestamp() -> Option<u64> {
    let base = base()?;
    // SAFETY: `base` is the MMIO window the device tree published for the
    // PL031 and the boot map covered as Device memory. One volatile word read
    // of a register with no side effects.
    let seconds = unsafe { core::ptr::read_volatile((base + DR) as *const u32) };
    Some(seconds as u64)
}
