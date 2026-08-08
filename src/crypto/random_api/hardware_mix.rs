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

use crate::crypto::util::rng;

const HW_BUF_LEN: usize = 64;

pub(super) fn mix_hardware_entropy(buffer: &mut [u8]) {
    if buffer.is_empty() {
        return;
    }
    mix_virtio_rng(buffer);
    mix_cpu_rng(buffer);
}

fn mix_virtio_rng(buffer: &mut [u8]) {
    if !crate::drivers::virtio_rng::is_available() {
        return;
    }

    let mut hw_buf = [0u8; HW_BUF_LEN];
    if crate::drivers::virtio_rng::fill_random(&mut hw_buf).is_ok() {
        xor_repeating(buffer, &hw_buf);
    }
    zeroize(&mut hw_buf);
}

fn mix_cpu_rng(buffer: &mut [u8]) {
    if !(rng::has_cpu_random() || rng::has_cpu_entropy()) {
        return;
    }

    for chunk in buffer.chunks_mut(8) {
        let Some(v) = next_cpu_random() else {
            continue;
        };
        let bytes = v.to_le_bytes();
        for (idx, byte) in chunk.iter_mut().enumerate() {
            *byte ^= bytes[idx];
        }
    }
}

fn next_cpu_random() -> Option<u64> {
    rng::cpu_entropy64().or_else(rng::cpu_random64)
}

fn xor_repeating(out: &mut [u8], input: &[u8]) {
    for (idx, byte) in out.iter_mut().enumerate() {
        *byte ^= input[idx % input.len()];
    }
}

fn zeroize(bytes: &mut [u8]) {
    for byte in bytes {
        unsafe { core::ptr::write_volatile(byte, 0) };
    }
}
