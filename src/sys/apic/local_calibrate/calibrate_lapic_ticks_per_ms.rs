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

use core::sync::atomic::Ordering;

use super::super::local::constants::{
    LAPIC_LVT_TIMER, LAPIC_TIMER_CURRENT, LAPIC_TIMER_DIV, LAPIC_TIMER_INIT, LAPIC_TIMER_MASKED,
};
use super::super::local::regs::{lapic_read_raw, lapic_write_raw};
use super::accurate_tsc_hz::accurate_tsc_hz;
use super::consts::{LAPIC_TICKS_PER_MS, LAPIC_TICKS_PER_MS_MAX, LAPIC_TICKS_PER_MS_MIN};
use crate::sys::timer::tsc::rdtsc;

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
