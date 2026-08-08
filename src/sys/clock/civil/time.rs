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

use super::days::{days_in_month, days_in_year};

pub const UNIX_EPOCH_YEAR: u16 = 1970;
const SECS_PER_MIN: u64 = 60;
const SECS_PER_HOUR: u64 = 60 * SECS_PER_MIN;
const SECS_PER_DAY: u64 = 24 * SECS_PER_HOUR;

/// A date and time in UTC, with no timezone and no leap seconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CivilTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// Split seconds since the Unix epoch into a date and time.
pub fn from_unix(timestamp: u64) -> CivilTime {
    let mut remaining = timestamp;
    let second = (remaining % 60) as u8;
    remaining /= 60;
    let minute = (remaining % 60) as u8;
    remaining /= 60;
    let hour = (remaining % 24) as u8;
    remaining /= 24;

    let mut year = UNIX_EPOCH_YEAR;
    while remaining >= days_in_year(year) as u64 {
        remaining -= days_in_year(year) as u64;
        year += 1;
    }
    let mut month = 1u8;
    while remaining >= days_in_month(year, month) as u64 {
        remaining -= days_in_month(year, month) as u64;
        month += 1;
    }
    CivilTime { year, month, day: (remaining + 1) as u8, hour, minute, second }
}

/// Seconds since the Unix epoch for a date and time.
pub fn to_unix(t: &CivilTime) -> u64 {
    let mut days = 0u64;
    let mut year = UNIX_EPOCH_YEAR;
    while year < t.year {
        days += days_in_year(year) as u64;
        year += 1;
    }
    let mut month = 1u8;
    while month < t.month {
        days += days_in_month(t.year, month) as u64;
        month += 1;
    }
    days += t.day.saturating_sub(1) as u64;
    days * SECS_PER_DAY
        + (t.hour as u64) * SECS_PER_HOUR
        + (t.minute as u64) * SECS_PER_MIN
        + t.second as u64
}
