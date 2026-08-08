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
use crate::sys::serial;

/// Print one measurement on the same channel the boot markers use, so the
/// benchmark harness parses both with one reader.
///
/// Shape is `[BENCH] <uptime> micro:<name> iters min avg max` with ticks, and
/// nanoseconds only where the platform actually told us the counter rate.
/// Where it did not, the field is absent rather than estimated: a made up
/// nanosecond figure would be quoted later as if it had been measured.
pub fn report(name: &[u8], sample: &Sample) {
    serial::print(b"[BENCH] ");
    serial::print_dec(crate::sys::timer::uptime_ms());
    serial::print(b" micro:");
    serial::print(name);
    serial::print(b" iters=");
    serial::print_dec(sample.iterations as u64);
    serial::print(b" min=");
    serial::print_dec(sample.min);
    serial::print(b" avg=");
    serial::print_dec(sample.avg);
    serial::print(b" max=");
    serial::print_dec(sample.max);

    match sample.min_nanos() {
        Some(ns) => {
            serial::print(b" min_ns=");
            serial::print_dec(ns);
            serial::println(b"");
        }
        None => serial::println(b" min_ns=uncalibrated"),
    }
}
