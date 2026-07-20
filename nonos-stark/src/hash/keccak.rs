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

extern crate alloc;
use alloc::vec::Vec;

use super::constants::{PI_LANE, RHO_OFFSETS, ROUND_CONSTANTS};

pub(crate) fn keccak_f(state: &mut [u64; 25]) {
    for round in 0..24 {
        let mut c = [0u64; 5];
        for x in 0..5 {
            c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
        }

        let mut d = [0u64; 5];
        for x in 0..5 {
            d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
        }

        for x in 0..5 {
            for y in 0..5 {
                state[y * 5 + x] ^= d[x];
            }
        }

        let mut current = state[1];
        for i in 0..24 {
            let j = PI_LANE[i];
            let temp = state[j];
            state[j] = current.rotate_left(RHO_OFFSETS[i]);
            current = temp;
        }

        for y in 0..5 {
            let t = [
                state[y * 5 + 0],
                state[y * 5 + 1],
                state[y * 5 + 2],
                state[y * 5 + 3],
                state[y * 5 + 4],
            ];
            for x in 0..5 {
                state[y * 5 + x] = t[x] ^ ((!t[(x + 1) % 5]) & t[(x + 2) % 5]);
            }
        }

        state[0] ^= ROUND_CONSTANTS[round];
    }
}

pub struct Keccak {
    state: [u64; 25],
    buffer: Vec<u8>,
    rate: usize,
    pub(crate) output_len: usize,
    suffix: u8,
}

impl Keccak {
    pub fn new(capacity: usize, output_len: usize, suffix: u8) -> Self {
        assert!(capacity <= 1600);
        assert!(capacity % 8 == 0);

        Self {
            state: [0u64; 25],
            buffer: Vec::new(),
            rate: (1600 - capacity) / 8,
            output_len,
            suffix,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    fn absorb(&mut self) {
        self.buffer.push(self.suffix);

        while self.buffer.len() % self.rate != 0 {
            self.buffer.push(0);
        }

        if let Some(last) = self.buffer.last_mut() {
            *last |= 0x80;
        }

        for chunk in self.buffer.chunks_exact(self.rate) {
            for (i, &byte) in chunk.iter().enumerate() {
                let lane_idx = i / 8;
                let byte_idx = i % 8;
                let byte_shift = byte_idx * 8;
                self.state[lane_idx] ^= (byte as u64) << byte_shift;
            }

            keccak_f(&mut self.state);
        }
    }

    fn squeeze(&mut self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.output_len);
        let mut remaining = self.output_len;

        while remaining > 0 {
            let to_extract = core::cmp::min(remaining, self.rate);

            for i in 0..to_extract {
                let lane_idx = i / 8;
                let byte_idx = i % 8;
                let byte = (self.state[lane_idx] >> (byte_idx * 8)) as u8;
                output.push(byte);
            }

            remaining -= to_extract;

            if remaining > 0 {
                keccak_f(&mut self.state);
            }
        }

        output
    }

    pub fn finalize(mut self) -> Vec<u8> {
        self.absorb();
        self.squeeze()
    }
}

impl Drop for Keccak {
    fn drop(&mut self) {
        for lane in &mut self.state {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { core::ptr::write_volatile(lane, 0) };
        }
        for byte in &mut self.buffer {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
