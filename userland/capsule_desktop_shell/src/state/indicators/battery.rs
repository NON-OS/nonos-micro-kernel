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

use nonos_libc::mk_battery_status;

/// Battery charge as a percent, or None on AC power or when the reading is
/// unavailable. Used to fill the battery glyph on the menu bar.
pub fn percent() -> Option<u32> {
    let rc = mk_battery_status();
    if (0..=100).contains(&rc) {
        Some(rc as u32)
    } else {
        None
    }
}

pub fn label(buf: &mut [u8; 4]) -> usize {
    let rc = mk_battery_status();
    if rc < 0 || rc > 100 {
        buf[0] = b'A';
        buf[1] = b'C';
        return 2;
    }
    let p = rc as u32;
    let mut n = 0;
    if p >= 100 {
        buf[0] = b'1';
        buf[1] = b'0';
        buf[2] = b'0';
        buf[3] = b'%';
        return 4;
    }
    if p >= 10 {
        buf[n] = b'0' + ((p / 10) % 10) as u8;
        n += 1;
    }
    buf[n] = b'0' + (p % 10) as u8;
    n += 1;
    buf[n] = b'%';
    n + 1
}
