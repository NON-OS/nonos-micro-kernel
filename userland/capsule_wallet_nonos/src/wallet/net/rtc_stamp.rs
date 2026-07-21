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

use nonos_libc::{mk_time_rtc, RtcTime};

pub fn rtc_stamp() -> Option<u64> {
    let mut t = RtcTime::default();
    if mk_time_rtc(&mut t as *mut RtcTime) != 0 {
        return None;
    }
    Some(
        t.year as u64 * 10_000_000_000
            + t.month as u64 * 100_000_000
            + t.day as u64 * 1_000_000
            + t.hour as u64 * 10_000
            + t.minute as u64 * 100
            + t.second as u64,
    )
}
