// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::sys::clock::core::unix_ms;
use crate::sys::policy;

pub(super) fn local_unix_secs() -> u64 {
    let ms = unix_ms();
    let tz_offset = policy::timezone_offset() as i64;
    ((ms / 1000) as i64 + tz_offset * 3600).max(0) as u64
}

pub(super) struct DateParts {
    pub month: usize,
    pub day: u32,
    pub dow: u32,
}

pub(super) fn date_from_secs(local_secs: u64) -> DateParts {
    let total_days = (local_secs / 86400) as u32;
    let mut year = 1970u32;
    let mut remaining = total_days;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if remaining < days_in_year {
            break;
        }
        remaining -= days_in_year;
        year += 1;
    }
    let days_in_months = month_days(year);
    let mut month = 0usize;
    for (i, &days) in days_in_months.iter().enumerate() {
        if remaining < days {
            month = i;
            break;
        }
        remaining -= days;
    }
    DateParts { month, day: remaining + 1, dow: (total_days + 4) % 7 }
}

pub(super) fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub(super) fn month_days(year: u32) -> [u32; 12] {
    if is_leap(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    }
}

pub(super) const DOW_NAMES: [&[u8; 3]; 7] =
    [b"Sun", b"Mon", b"Tue", b"Wed", b"Thu", b"Fri", b"Sat"];
pub(super) const MON_NAMES: [&[u8; 3]; 12] = [
    b"Jan", b"Feb", b"Mar", b"Apr", b"May", b"Jun", b"Jul", b"Aug", b"Sep", b"Oct", b"Nov", b"Dec",
];
