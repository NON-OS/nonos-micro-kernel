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

use core::sync::atomic::Ordering;

use super::consts::{BOOT_TSC, BOOT_UNIX_MS, NTP_OFFSET_MS, TSC_HZ};
use super::rdtsc::rdtsc;

pub fn base_unix_ms() -> u64 {
    let tsc_hz = TSC_HZ.load(Ordering::Relaxed);
    if tsc_hz == 0 {
        return crate::time::timestamp_millis();
    }

    let boot_tsc = BOOT_TSC.load(Ordering::Relaxed);
    let current_tsc = rdtsc();
    let elapsed_tsc = current_tsc.saturating_sub(boot_tsc);

    let elapsed_ms = (elapsed_tsc * 1000) / tsc_hz;

    BOOT_UNIX_MS.load(Ordering::Relaxed) + elapsed_ms
}

pub fn since_boot_ms() -> u64 {
    let tsc_hz = TSC_HZ.load(Ordering::Relaxed);
    if tsc_hz == 0 {
        return crate::time::timestamp_millis().saturating_sub(BOOT_UNIX_MS.load(Ordering::Relaxed));
    }

    let boot_tsc = BOOT_TSC.load(Ordering::Relaxed);
    let current_tsc = rdtsc();
    let elapsed_tsc = current_tsc.saturating_sub(boot_tsc);

    (elapsed_tsc * 1000) / tsc_hz
}

pub fn unix_ms() -> u64 {
    let adjusted = base_unix_ms() as i64 + NTP_OFFSET_MS.load(Ordering::Relaxed);
    if adjusted < 0 {
        0
    } else {
        adjusted as u64
    }
}

pub fn set_ntp_offset_ms(offset: i64) {
    NTP_OFFSET_MS.store(offset, Ordering::Relaxed);
}
