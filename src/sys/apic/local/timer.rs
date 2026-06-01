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

use crate::sys::serial;

use super::constants::{
    LAPIC_LVT_TIMER, LAPIC_TIMER_DIV, LAPIC_TIMER_INIT, TIMER_MODE_PERIODIC, TIMER_VECTOR,
};
use super::init::init_local_apic;
use super::regs::lapic_write_raw;
use super::state::LAPIC_INIT;

pub fn setup_timer(frequency_hz: u32) {
    if !LAPIC_INIT.load(Ordering::Relaxed) {
        init_local_apic();
    }
    serial::print(b"[APIC] Setting up timer at ");
    serial::print_dec(frequency_hz as u64);
    serial::println(b" Hz");

    let ticks_per_ms = crate::sys::apic::local_calibrate::calibrate_lapic_ticks_per_ms();
    let initial_count = compute_lapic_initial_count(ticks_per_ms, frequency_hz);

    unsafe {
        lapic_write_raw(LAPIC_TIMER_DIV, 0x03);
        let timer_config = (TIMER_VECTOR as u32) | TIMER_MODE_PERIODIC;
        lapic_write_raw(LAPIC_LVT_TIMER, timer_config);
        lapic_write_raw(LAPIC_TIMER_INIT, initial_count);
    }
}

fn compute_lapic_initial_count(ticks_per_ms: u64, frequency_hz: u32) -> u32 {
    if frequency_hz == 0 {
        return 0;
    }
    if ticks_per_ms == 0 {
        return 10_000_000 / frequency_hz;
    }
    let target = (ticks_per_ms * 1000) / (frequency_hz as u64);
    if target > u32::MAX as u64 {
        u32::MAX
    } else if target == 0 {
        1
    } else {
        target as u32
    }
}
