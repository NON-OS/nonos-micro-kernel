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

extern crate alloc;

use alloc::string::String;

use super::fmt_time::civil_from_days;

// Indexed straight off the epoch day rather than off a calendar: 1970-01-01 was
// a Thursday, so day 0 lands on the first entry and every later day rotates.
const WEEK: [&str; 7] =
    ["Thursday", "Friday", "Saturday", "Sunday", "Monday", "Tuesday", "Wednesday"];

const MONTH: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// The line under the greeting, read off the same wall clock the greeting is.
/// A zero clock has no date behind it, so it says so rather than printing the
/// epoch as if it were today.
pub fn date_line(now_ms: u64) -> String {
    if now_ms == 0 {
        return String::from("The system clock is not available.");
    }
    let days = (now_ms / 1000 / 86_400) as i64;
    let (_, month, day) = civil_from_days(days);
    let name = MONTH[(month.clamp(1, 12) - 1) as usize];
    alloc::format!("{}, {} {}", WEEK[days.rem_euclid(7) as usize], day, name)
}
