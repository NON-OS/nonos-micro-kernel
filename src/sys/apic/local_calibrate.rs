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

pub fn calibrate_lapic_ticks_per_ms() -> u64 {
    let cached = LAPIC_TICKS_PER_MS.load(Ordering::Acquire);
    if cached != 0 {
        return cached;
    }

    let tsc_hz = tsc_frequency();
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
        while rdtsc() < target {
            core::hint::spin_loop();
        }
        let remaining = lapic_read_raw(LAPIC_TIMER_CURRENT);

        lapic_write_raw(LAPIC_TIMER_INIT, 0);
        lapic_write_raw(LAPIC_TIMER_DIV, 0x03);
        let prev = lapic_read_raw(LAPIC_LVT_TIMER);
        lapic_write_raw(LAPIC_LVT_TIMER, prev | LAPIC_TIMER_MASKED);

        let elapsed = u32::MAX.wrapping_sub(remaining) as u64;
        let ticks_per_ms = elapsed / window_ms;
        LAPIC_TICKS_PER_MS.store(ticks_per_ms, Ordering::Release);
        ticks_per_ms
    }
}
