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

use crate::sys::serial;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub static TSC_FREQ_HZ: AtomicU64 = AtomicU64::new(0);
pub static BOOT_TSC: AtomicU64 = AtomicU64::new(0);
pub static BOOT_EPOCH_MS: AtomicU64 = AtomicU64::new(0);
pub static TIMER_INIT: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn rdtsc() -> u64 {
    unsafe {
        let lo: u32;
        let hi: u32;
        core::arch::asm!(
            "rdtsc",
            out("eax") lo,
            out("edx") hi,
            options(nomem, nostack)
        );
        ((hi as u64) << 32) | (lo as u64)
    }
}

pub fn init(tsc_hz: u64, boot_epoch_ms: u64) {
    if TIMER_INIT.load(Ordering::Relaxed) {
        return;
    }

    BOOT_TSC.store(rdtsc(), Ordering::SeqCst);

    if tsc_hz > 0 {
        TSC_FREQ_HZ.store(tsc_hz, Ordering::SeqCst);
    } else {
        TSC_FREQ_HZ.store(2_500_000_000, Ordering::SeqCst);
    }

    BOOT_EPOCH_MS.store(boot_epoch_ms, Ordering::SeqCst);

    TIMER_INIT.store(true, Ordering::SeqCst);

    serial::print(b"[TIMER] Initialized, TSC freq=");
    serial::print_dec(tsc_hz / 1_000_000);
    serial::println(b" MHz");
}

pub fn init_default() {
    let real_unix_timestamp = crate::arch::x86_64::time::rtc::read_unix_timestamp();
    let real_unix_ms = real_unix_timestamp * 1000;
    let calibrated = calibrate_tsc_hz();
    init(calibrated, real_unix_ms);
}

pub fn calibrate_tsc_hz() -> u64 {
    use crate::arch::x86_64::time::tsc as arch_tsc;
    if let Some(freq) = arch_tsc::get_cpuid_frequency() {
        return freq;
    }
    match arch_tsc::calibrate_with_pit() {
        Ok((freq, _confidence)) => freq,
        Err(_) => 0,
    }
}

pub fn tsc_frequency() -> u64 {
    TSC_FREQ_HZ.load(Ordering::Relaxed)
}

pub fn ticks_to_ns(ticks: u64) -> u64 {
    let freq = TSC_FREQ_HZ.load(Ordering::Relaxed);
    if freq == 0 {
        return 0;
    }
    let ns_per_tick = 1_000_000_000u128 / freq as u128;
    ((ticks as u128) * ns_per_tick) as u64
}

pub fn ticks_to_us(ticks: u64) -> u64 {
    let freq = TSC_FREQ_HZ.load(Ordering::Relaxed);
    if freq == 0 {
        return 0;
    }
    ticks * 1_000_000 / freq
}

pub fn ticks_to_ms(ticks: u64) -> u64 {
    let freq = TSC_FREQ_HZ.load(Ordering::Relaxed);
    if freq == 0 {
        return 0;
    }
    ticks * 1_000 / freq
}

pub fn us_to_ticks(us: u64) -> u64 {
    let freq = TSC_FREQ_HZ.load(Ordering::Relaxed);
    freq * us / 1_000_000
}

pub fn ms_to_ticks(ms: u64) -> u64 {
    let freq = TSC_FREQ_HZ.load(Ordering::Relaxed);
    freq * ms / 1_000
}
