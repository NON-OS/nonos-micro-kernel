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

use super::sample::Sample;

/// Emit one case as a marker the host side can collect.
///
/// Ticks are reported alongside nanoseconds because the tick rate is a
/// property of the part, and a reader comparing two machines needs the raw
/// count as well as the conversion.
pub fn report(name: &[u8], sample: &mut Sample, overhead: u64) {
    sample.sort();
    let hz = crate::arch::time_counter_hz();
    let net = |ticks: u64| ticks.saturating_sub(overhead);
    let ns = |ticks: u64| {
        if hz == 0 {
            0
        } else {
            net(ticks).saturating_mul(1_000_000_000) / hz
        }
    };

    crate::sys::serial::print(b"[UBENCH] ");
    crate::sys::serial::print(name);
    crate::sys::serial::print(b" n=");
    crate::sys::serial::print_dec(sample.len() as u64);
    crate::sys::serial::print(b" min=");
    crate::sys::serial::print_dec(net(sample.min()));
    crate::sys::serial::print(b" p50=");
    crate::sys::serial::print_dec(net(sample.quantile(1, 2)));
    crate::sys::serial::print(b" p95=");
    crate::sys::serial::print_dec(net(sample.quantile(95, 100)));
    crate::sys::serial::print(b" max=");
    crate::sys::serial::print_dec(net(sample.max()));
    crate::sys::serial::print(b" ticks p50_ns=");
    crate::sys::serial::print_dec(ns(sample.quantile(1, 2)));
    crate::sys::serial::println(b"");
}
