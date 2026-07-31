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
use core::sync::atomic::Ordering;

use super::consts::{BOOT_EPOCH_MS, BOOT_TSC, TIMER_INIT, TSC_FREQ_HZ};
use super::rdtsc::rdtsc;

pub fn init(tsc_hz: u64, boot_epoch_ms: u64) {
    if TIMER_INIT.load(Ordering::Relaxed) {
        return;
    }

    BOOT_TSC.store(rdtsc(), Ordering::SeqCst);

    // A caller that has no frequency to hand over gets one from the counter
    // itself before falling back to a guess: aarch64 always publishes the real
    // rate in CNTFRQ_EL0, and a PC-shaped 2.5 GHz there is off by more than an
    // order of magnitude, which turns every timeout into the wrong duration.
    let hz = match (tsc_hz, calibrate_tsc_hz()) {
        (0, 0) => 2_500_000_000,
        (0, probed) => probed,
        (given, _) => given,
    };
    TSC_FREQ_HZ.store(hz, Ordering::SeqCst);

    BOOT_EPOCH_MS.store(boot_epoch_ms, Ordering::SeqCst);

    TIMER_INIT.store(true, Ordering::SeqCst);

    serial::print(b"[TIMER] Initialized, TSC freq=");
    serial::print_dec(hz / 1_000_000);
    serial::println(b" MHz");
}

pub fn init_default() {
    let real_unix_timestamp = crate::arch::wall_clock::unix_timestamp().unwrap_or(0);
    let real_unix_ms = real_unix_timestamp * 1000;
    let calibrated = calibrate_tsc_hz();
    init(calibrated, real_unix_ms);
}

pub fn calibrate_tsc_hz() -> u64 {
    crate::arch::time_counter_hz()
}
