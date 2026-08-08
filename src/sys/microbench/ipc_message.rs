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

/// Building one IPC message: derive the channel key and checksum the payload.
///
/// This is the work every send does before a byte moves, and it is the part
/// that scales with payload rather than with the scheduler, so it is measured
/// on its own.
pub fn build(rounds: usize, payload: &[u8]) -> Sample {
    let mut sample = Sample::new();
    for _ in 0..rounds.min(Sample::CAPACITY) {
        let start = crate::arch::read_time_counter();
        let message = crate::ipc::nonos_channel::IpcMessage::new(
            "bench.source",
            "bench.sink",
            payload,
        );
        let end = crate::arch::read_time_counter();
        // Keep the result observable so the work cannot be folded away.
        if message.is_ok() {
            sample.push(end.wrapping_sub(start));
        }
    }
    sample
}

/// Validating a received message, which is what the receiving side pays.
pub fn validate(rounds: usize, payload: &[u8]) -> Sample {
    let mut sample = Sample::new();
    let Ok(message) = crate::ipc::nonos_channel::IpcMessage::new(
        "bench.source",
        "bench.sink",
        payload,
    ) else {
        return sample;
    };
    for _ in 0..rounds.min(Sample::CAPACITY) {
        let start = crate::arch::read_time_counter();
        let good = message.validate_integrity();
        let end = crate::arch::read_time_counter();
        if good {
            sample.push(end.wrapping_sub(start));
        }
    }
    sample
}
