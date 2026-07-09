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

use crate::{fmt_time, human_size};

#[test]
fn human_size_exact_under_1k() {
    assert_eq!(human_size(0), "0");
    assert_eq!(human_size(1), "1");
    assert_eq!(human_size(500), "500");
    assert_eq!(human_size(1023), "1023");
}

#[test]
fn human_size_kilobytes() {
    assert_eq!(human_size(1024), "1.0K");
    assert_eq!(human_size(1536), "1.5K");
    assert_eq!(human_size(1024 * 10), "10.0K");
}

#[test]
fn human_size_mega_and_giga() {
    assert_eq!(human_size(1024 * 1024), "1.0M");
    assert_eq!(human_size(1024 * 1024 * 3 / 2), "1.5M");
    assert_eq!(human_size(1024 * 1024 * 1024), "1.0G");
}

#[test]
fn fmt_time_unknown_is_dash() {
    assert_eq!(fmt_time(0), "--");
}

#[test]
fn fmt_time_known_epoch() {
    // 1970-01-15 12:34:00 UTC -> day 14, 45240s into the day.
    assert_eq!(fmt_time(1_254_840_000), "01-15 12:34");
}

#[test]
fn fmt_time_handles_leap_day() {
    // 2024-02-29 00:00:00 UTC (a leap day) is 1_709_164_800 s.
    assert_eq!(fmt_time(1_709_164_800_000), "02-29 00:00");
}

#[test]
fn fmt_time_year_boundary() {
    // 2025-01-01 00:00:00 UTC is 1_735_689_600 s.
    assert_eq!(fmt_time(1_735_689_600_000), "01-01 00:00");
}
