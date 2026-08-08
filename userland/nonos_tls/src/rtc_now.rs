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
//! The clock a certificate is judged against.

use nonos_libc::{mk_time_rtc, RtcTime};

/// The wall clock as `YYYYMMDDhhmmss`, which is the shape the validity check
/// compares against a certificate's notBefore and notAfter.
///
/// Zero when the clock cannot be read. A caller passing zero is saying it does
/// not know the time, and every certificate then looks expired, which is the
/// safe direction to fail.
pub fn rtc_now() -> u64 {
    let mut t = RtcTime::default();
    if mk_time_rtc(&mut t as *mut RtcTime) != 0 {
        return 0;
    }
    (t.year as u64) * 10_000_000_000
        + (t.month as u64) * 100_000_000
        + (t.day as u64) * 1_000_000
        + (t.hour as u64) * 10_000
        + (t.minute as u64) * 100
        + t.second as u64
}
