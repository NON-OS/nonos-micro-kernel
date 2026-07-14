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

use super::consts::TSC_FREQ_HZ;

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
