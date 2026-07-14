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

use super::sane::sane;
use crate::sys::timer::tsc::tsc_frequency;

pub(super) fn accurate_tsc_hz() -> u64 {
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
