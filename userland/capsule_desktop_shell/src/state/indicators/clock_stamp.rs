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

use super::clock::hhmm;
use nonos_libc::{mk_time_rtc, RtcTime};

/// `Thu 20 Aug  02:33` — the menu bar's single-line stamp.
pub const STAMP_LEN: usize = 17;

const DAYS: [&[u8; 3]; 7] = [b"Sun", b"Mon", b"Tue", b"Wed", b"Thu", b"Fri", b"Sat"];
const MONTHS: [&[u8; 3]; 12] = [
    b"Jan", b"Feb", b"Mar", b"Apr", b"May", b"Jun", b"Jul", b"Aug", b"Sep", b"Oct", b"Nov", b"Dec",
];
const SHIFT: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];

/// Fill `buf` with the stamp, or leave it untouched and answer `false` when the
/// RTC does not respond.
pub fn stamp(buf: &mut [u8; STAMP_LEN], h24: bool) -> bool {
    let mut t = RtcTime::default();
    if mk_time_rtc(&mut t as *mut RtcTime) != 0 {
        return false;
    }
    let month = (t.month as usize).clamp(1, 12);
    buf[..3].copy_from_slice(DAYS[weekday(t.year as i32, month, t.day as i32)]);
    buf[3] = b' ';
    buf[4] = b'0' + (t.day / 10) % 10;
    buf[5] = b'0' + t.day % 10;
    buf[6] = b' ';
    buf[7..10].copy_from_slice(MONTHS[month - 1]);
    buf[10] = b' ';
    buf[11] = b' ';
    let mut hm = [b'-'; 5];
    if !hhmm(&mut hm, h24) {
        return false;
    }
    buf[12..].copy_from_slice(&hm);
    true
}

// Sakamoto: 0 is Sunday, and January and February count as months 13 and 14 of
// the year before.
fn weekday(year: i32, month: usize, day: i32) -> usize {
    let y = year - (month < 3) as i32;
    (y + y / 4 - y / 100 + y / 400 + SHIFT[month - 1] + day).rem_euclid(7) as usize
}
