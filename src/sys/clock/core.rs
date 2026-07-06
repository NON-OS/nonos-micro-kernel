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

use core::sync::atomic::{AtomicI64, AtomicU64, Ordering};

pub static TSC_HZ: AtomicU64 = AtomicU64::new(0);
pub static BOOT_UNIX_MS: AtomicU64 = AtomicU64::new(0);
pub static BOOT_TSC: AtomicU64 = AtomicU64::new(0);
pub static NTP_OFFSET_MS: AtomicI64 = AtomicI64::new(0);

pub fn init(tsc_hz: u64, unix_epoch_ms: u64) {
    TSC_HZ.store(tsc_hz, Ordering::Relaxed);
    BOOT_UNIX_MS.store(unix_epoch_ms, Ordering::Relaxed);
    BOOT_TSC.store(rdtsc(), Ordering::Relaxed);
}

#[inline]
pub fn rdtsc() -> u64 {
    unsafe {
        let lo: u32;
        let hi: u32;
        core::arch::asm!("rdtsc", out("eax") lo, out("edx") hi);
        ((hi as u64) << 32) | (lo as u64)
    }
}

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
