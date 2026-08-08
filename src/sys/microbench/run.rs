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

use super::{counter_overhead, ipc_message, report::report};

const ROUNDS: usize = 1024;
const SMALL: usize = 64;
const LARGE: usize = 1024;

/// Run every case and emit the results.
///
/// Called once the heap and the clock are up, since the counter frequency is
/// needed to turn ticks into nanoseconds and the IPC secret has to be latched
/// before a channel key can be derived.
pub fn run() {
    let overhead = counter_overhead::measure(ROUNDS);
    crate::sys::serial::print(b"[UBENCH] counter_overhead ticks=");
    crate::sys::serial::print_dec(overhead);
    crate::sys::serial::print(b" hz=");
    crate::sys::serial::print_dec(crate::arch::time_counter_hz());
    crate::sys::serial::println(b"");

    let small = [0x5Au8; SMALL];
    let large = [0xA5u8; LARGE];

    let mut sample = ipc_message::build(ROUNDS, &small);
    report(b"ipc_build_64B", &mut sample, overhead);

    let mut sample = ipc_message::build(ROUNDS, &large);
    report(b"ipc_build_1KiB", &mut sample, overhead);

    let mut sample = ipc_message::validate(ROUNDS, &small);
    report(b"ipc_validate_64B", &mut sample, overhead);

    let mut sample = ipc_message::validate(ROUNDS, &large);
    report(b"ipc_validate_1KiB", &mut sample, overhead);

    crate::sys::serial::println(b"[UBENCH] done");
}
