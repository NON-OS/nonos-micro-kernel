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

use core::sync::atomic::{AtomicU64, Ordering};

use super::local::constants::{
    LAPIC_LVT_TIMER, LAPIC_TIMER_CURRENT, LAPIC_TIMER_DIV, LAPIC_TIMER_INIT, LAPIC_TIMER_MASKED,
};
use super::local::regs::{lapic_read_raw, lapic_write_raw};
use crate::sys::timer::tsc::{rdtsc, tsc_frequency};

static LAPIC_TICKS_PER_MS: AtomicU64 = AtomicU64::new(0);

// Trust the TSC frequency to time the LAPIC only if it is a real
// measurement. The bootloader hands over an estimate and the kernel falls
// back to a hardcoded 2.5 GHz when it has nothing; both are wrong on most
// real CPUs, and a wrong TSC rate scales the LAPIC timer by the same
// factor, so the scheduler tick runs fast or slow and the whole system
// drifts. The PIT is a fixed 1.193182 MHz part on every x86 machine;
// measuring the TSC against it gives the true rate independent of any
// firmware claim.
// A TSC that reports outside this band is a broken measurement, not a real
// CPU. Rejecting it keeps a flaky PIT emulation (some hypervisors) or a bad
// firmware value from producing a nonsense window that would spin the
// calibration loop below effectively forever.
const TSC_HZ_MIN: u64 = 300_000_000;
const TSC_HZ_MAX: u64 = 6_000_000_000;

fn sane(hz: u64) -> Option<u64> {
    (TSC_HZ_MIN..=TSC_HZ_MAX).contains(&hz).then_some(hz)
}

fn accurate_tsc_hz() -> u64 {
    use crate::arch::x86_64::time::tsc::get_cpuid_frequency;
    // Enumerated frequency (CPUID leaf 0x15/0x16) is exact and needs no
    // port I/O. Under hardware virtualization every port access is a VM
    // exit, so a PIT-polling calibration would take tens of seconds there;
    // this path touches no ports and works on real Intel and hypervisors
    // alike. When CPUID does not report it, the bootloader estimate is the
    // fallback, and the final tick rate is clamped below so even a rough
    // estimate still yields a working timer.
    if let Some(hz) = get_cpuid_frequency().and_then(sane) {
        return hz;
    }
    sane(tsc_frequency()).unwrap_or(2_000_000_000)
}

// A LAPIC timer runs off the CPU bus clock: 100 to 400 MHz on real parts,
// so with the divide-by-16 configured below it decrements 6250 to 25000
// times per millisecond. Clamping the measured rate to a slightly wider
// band means a wrong TSC estimate can shift the tick a little but can never
// program a pathologically short period, which is what made an
// uncalibrated timer fire thousands of times a second and wedge the box.
const LAPIC_TICKS_PER_MS_MIN: u64 = 2_000;
const LAPIC_TICKS_PER_MS_MAX: u64 = 60_000;

pub fn calibrate_lapic_ticks_per_ms() -> u64 {
    let cached = LAPIC_TICKS_PER_MS.load(Ordering::Acquire);
    if cached != 0 {
        return cached;
    }

    let tsc_hz = accurate_tsc_hz();
    if tsc_hz == 0 {
        return 0;
    }

    let window_ms: u64 = 10;
    let tsc_window = (tsc_hz / 1000) * window_ms;

    unsafe {
        lapic_write_raw(LAPIC_TIMER_DIV, 0x03);
        lapic_write_raw(LAPIC_TIMER_INIT, u32::MAX);

        let tsc_start = rdtsc();
        let target = tsc_start.wrapping_add(tsc_window);
        // Backstop against a non-advancing TSC. A sane window is at most
        // ~6e7 TSC ticks and each loop turn lets the TSC move tens of ticks,
        // so a real clock exits on the comparison in a few million turns;
        // 50 million leaves wide margin yet bails in a fraction of a second
        // if rdtsc is frozen, instead of spinning for a minute.
        let mut guard: u64 = 50_000_000;
        while rdtsc() < target && guard > 0 {
            guard -= 1;
            core::hint::spin_loop();
        }
        let remaining = lapic_read_raw(LAPIC_TIMER_CURRENT);

        lapic_write_raw(LAPIC_TIMER_INIT, 0);
        lapic_write_raw(LAPIC_TIMER_DIV, 0x03);
        let prev = lapic_read_raw(LAPIC_LVT_TIMER);
        lapic_write_raw(LAPIC_LVT_TIMER, prev | LAPIC_TIMER_MASKED);

        let elapsed = u32::MAX.wrapping_sub(remaining) as u64;
        let ticks_per_ms =
            (elapsed / window_ms).clamp(LAPIC_TICKS_PER_MS_MIN, LAPIC_TICKS_PER_MS_MAX);
        LAPIC_TICKS_PER_MS.store(ticks_per_ms, Ordering::Release);
        ticks_per_ms
    }
}
