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

use core::sync::atomic::AtomicU64;

pub(super) static LAPIC_TICKS_PER_MS: AtomicU64 = AtomicU64::new(0);

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
pub(super) const TSC_HZ_MIN: u64 = 300_000_000;
pub(super) const TSC_HZ_MAX: u64 = 6_000_000_000;

// A LAPIC timer runs off the CPU bus clock: 100 to 400 MHz on real parts,
// so with the divide-by-16 configured below it decrements 6250 to 25000
// times per millisecond. Clamping the measured rate to a slightly wider
// band means a wrong TSC estimate can shift the tick a little but can never
// program a pathologically short period, which is what made an
// uncalibrated timer fire thousands of times a second and wedge the box.
pub(super) const LAPIC_TICKS_PER_MS_MIN: u64 = 2_000;
pub(super) const LAPIC_TICKS_PER_MS_MAX: u64 = 60_000;
