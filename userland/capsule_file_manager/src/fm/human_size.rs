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

use alloc::string::String;

// A short human-readable byte count: exact under 1K, then one decimal with a
// K/M/G suffix, so a listing column stays narrow and readable.
pub fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 3] = ["K", "M", "G"];
    if bytes < 1024 {
        return alloc::format!("{bytes}");
    }
    let mut value = bytes;
    let mut unit = 0usize;
    // Keep one fractional digit by carrying tenths through each division.
    let mut tenths = 0u64;
    while value >= 1024 && unit < UNITS.len() {
        tenths = (value % 1024) * 10 / 1024;
        value /= 1024;
        unit += 1;
    }
    alloc::format!("{value}.{tenths}{}", UNITS[unit - 1])
}
