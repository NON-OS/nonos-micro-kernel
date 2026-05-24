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

use super::freq::frequency;

pub fn nanoseconds_to_ticks(ns: u64) -> u64 {
    ns.saturating_mul(frequency()) / 1_000_000_000
}

pub fn ticks_to_nanoseconds(ticks: u64) -> u64 {
    let freq = frequency();
    if freq == 0 {
        return 0;
    }
    ticks.saturating_mul(1_000_000_000) / freq
}

pub fn microseconds_to_ticks(us: u64) -> u64 {
    us.saturating_mul(frequency()) / 1_000_000
}

pub fn ticks_to_microseconds(ticks: u64) -> u64 {
    let freq = frequency();
    if freq == 0 {
        return 0;
    }
    ticks.saturating_mul(1_000_000) / freq
}

pub fn milliseconds_to_ticks(ms: u64) -> u64 {
    ms.saturating_mul(frequency()) / 1_000
}

pub fn ticks_to_milliseconds(ticks: u64) -> u64 {
    let freq = frequency();
    if freq == 0 {
        return 0;
    }
    ticks.saturating_mul(1_000) / freq
}
