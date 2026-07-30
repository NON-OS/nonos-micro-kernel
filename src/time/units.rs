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

//! The same reading in the units each caller wants. Every one of these is
//! `now_ns` divided down, so they cannot disagree with each other about the
//! order of two events.

use super::now::now_ns;

/// Milliseconds since the clock was anchored. The kernel's usual unit: every
/// timeout, sleep deadline and rate limit is expressed in it.
#[inline]
pub fn timestamp_millis() -> u64 {
    now_ns() / 1_000_000
}

/// Seconds since the clock was anchored.
#[inline]
pub fn timestamp_secs() -> u64 {
    now_ns() / 1_000_000_000
}

/// Milliseconds, under the name the scheduler uses for them.
#[inline]
pub fn current_ticks() -> u64 {
    timestamp_millis()
}

/// Nanoseconds, under the name the monitoring paths use.
#[inline]
pub fn get_kernel_time_ns() -> u64 {
    now_ns()
}

/// Nanoseconds, named for the property callers rely on: it never goes
/// backwards, which is what makes a difference between two readings a
/// duration rather than a guess.
#[inline]
pub fn monotonic_ns() -> u64 {
    now_ns()
}
