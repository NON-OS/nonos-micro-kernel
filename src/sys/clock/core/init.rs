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

use super::consts::{BOOT_TSC, BOOT_UNIX_MS, TSC_HZ};
use super::rdtsc::rdtsc;

pub fn init(tsc_hz: u64, unix_epoch_ms: u64) {
    TSC_HZ.store(resolve_tsc_hz(tsc_hz), Ordering::Relaxed);
    BOOT_UNIX_MS.store(resolve_epoch_ms(unix_epoch_ms), Ordering::Relaxed);
    BOOT_TSC.store(rdtsc(), Ordering::Relaxed);
}

// When the handoff carries no TSC frequency, fall back to the timer subsystem's
// calibrated value, then to a fresh CPUID/PIT calibration, so the wall clock
// does not silently degrade to uptime on firmware that omits it.
fn resolve_tsc_hz(handoff_hz: u64) -> u64 {
    super::super::resolve::pick_nonzero(
        handoff_hz,
        crate::sys::timer::tsc::tsc_frequency,
        crate::sys::timer::tsc::calibrate_tsc_hz,
    )
}

// Likewise for the boot epoch: the handoff value, then the timer subsystem's
// RTC read, then a direct RTC read.
fn resolve_epoch_ms(handoff_ms: u64) -> u64 {
    super::super::resolve::pick_nonzero(
        handoff_ms,
        || crate::sys::timer::tsc::BOOT_EPOCH_MS.load(Ordering::Relaxed),
        || crate::arch::wall_clock::unix_timestamp().unwrap_or(0).saturating_mul(1000),
    )
}
