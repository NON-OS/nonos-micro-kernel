// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

use uefi::prelude::*;

/// Read CPU timestamp counter. Always available on x86_64.
pub fn read_tsc() -> u64 {
    // SAFETY: RDTSC is always available on x86_64
    unsafe { core::arch::x86_64::_rdtsc() }
}

/// Convert UEFI time to Unix epoch milliseconds. Returns 0 on RTC failure.
pub fn get_uefi_time_epoch(st: &SystemTable<Boot>) -> u64 {
    if let Ok(time) = st.runtime_services().get_time() {
        let year = time.year() as u64;
        let month = time.month() as u64;
        let day = time.day() as u64;
        let hour = time.hour() as u64;
        let minute = time.minute() as u64;
        let second = time.second() as u64;
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        const DAYS_BEFORE: [u64; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let mut days = 0u64;
        for y in 1970..year {
            days += if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) { 366 } else { 365 };
        }
        days += DAYS_BEFORE[(month - 1) as usize];
        if is_leap && month > 2 {
            days += 1;
        }
        days += day - 1;
        (days * 86400 + hour * 3600 + minute * 60 + second) * 1000
    } else {
        0
    }
}
