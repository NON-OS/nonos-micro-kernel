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

use core::ptr;
use core::sync::atomic::{compiler_fence, Ordering};

use super::constants::{INITIAL_STATE, K};
use super::Hash512;

pub struct Sha512 {
    state: [u64; 8],
    buffer: [u8; 128],
    buffer_len: usize,
    bit_len: u128,
}

impl Sha512 {
    #[inline]
    pub fn new() -> Self {
        Self { state: INITIAL_STATE, buffer: [0u8; 128], buffer_len: 0, bit_len: 0 }
    }

    /// Override initial hash state (used by SHA-384 which shares the
    /// SHA-512 compression function but starts with a different IV).
    pub fn set_state(&mut self, iv: [u64; 8]) {
        self.state = iv;
    }

    pub fn reset(&mut self) {
        for v in &mut self.state {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { ptr::write_volatile(v, 0) };
        }
        for b in &mut self.buffer {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { ptr::write_volatile(b, 0) };
        }
        // SAFETY: volatile write ensures zeroization isn't optimized out
        unsafe { ptr::write_volatile(&mut self.bit_len, 0) };
        compiler_fence(Ordering::SeqCst);

        self.state = INITIAL_STATE;
        self.buffer_len = 0;
        self.bit_len = 0;
    }

    pub fn update(&mut self, mut input: &[u8]) {
        self.bit_len = self.bit_len.wrapping_add((input.len() as u128) * 8);

        if self.buffer_len != 0 {
            let to_copy = core::cmp::min(128 - self.buffer_len, input.len());
            let dst = &mut self.buffer[self.buffer_len..self.buffer_len + to_copy];
            dst.copy_from_slice(&input[..to_copy]);
            self.buffer_len += to_copy;
            input = &input[to_copy..];

            if self.buffer_len == 128 {
                self.process_block(&self.buffer.clone());
                self.buffer_len = 0;
            }
        }

        while input.len() >= 128 {
            let mut block = [0u8; 128];
            block.copy_from_slice(&input[..128]);
            self.process_block(&block);
            input = &input[128..];
        }

        if !input.is_empty() {
            self.buffer[..input.len()].copy_from_slice(input);
            self.buffer_len = input.len();
        }
    }

    pub fn finalize(mut self) -> Hash512 {
        let mut pad_buf = [0u8; 256];
        pad_buf[..self.buffer_len].copy_from_slice(&self.buffer[..self.buffer_len]);

        pad_buf[self.buffer_len] = 0x80;

        let len_after_1 = self.buffer_len + 1;
        let pad_zeros =
            if len_after_1 <= 112 { 112 - len_after_1 } else { (128 - len_after_1) + 112 };

        let total_pad = 1 + pad_zeros + 16;
        let total_len = self.buffer_len + total_pad;

        let bit_len_be = self.bit_len.to_be_bytes();
        let len_pos = self.buffer_len + 1 + pad_zeros;
        pad_buf[len_pos..len_pos + 16].copy_from_slice(&bit_len_be);

        let mut offset = 0;
        while offset < total_len {
            let mut chunk = [0u8; 128];
            chunk.copy_from_slice(&pad_buf[offset..offset + 128]);
            self.process_block(&chunk);
            offset += 128;
        }

        let mut out = [0u8; 64];
        for (i, &v) in self.state.iter().enumerate() {
            out[i * 8..(i + 1) * 8].copy_from_slice(&v.to_be_bytes());
        }

        for v in &mut self.state {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { ptr::write_volatile(v, 0) };
        }
        for b in &mut self.buffer {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { ptr::write_volatile(b, 0) };
        }
        // SAFETY: volatile write ensures zeroization isn't optimized out
        unsafe { ptr::write_volatile(&mut self.bit_len, 0) };
        self.buffer_len = 0;
        compiler_fence(Ordering::SeqCst);

        out
    }

    fn process_block(&mut self, block: &[u8; 128]) {
        let mut w = [0u64; 80];

        for (word, chunk) in w.iter_mut().zip(block.chunks_exact(8)) {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(chunk);
            *word = u64::from_be_bytes(bytes);
        }

        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}

impl Default for Sha512 {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Sha512 {
    fn drop(&mut self) {
        for v in &mut self.state {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { ptr::write_volatile(v, 0) };
        }
        for b in &mut self.buffer {
            // SAFETY: volatile write ensures zeroization isn't optimized out
            unsafe { ptr::write_volatile(b, 0) };
        }
        // SAFETY: volatile write ensures zeroization isn't optimized out
        unsafe { ptr::write_volatile(&mut self.bit_len, 0) };
        self.buffer_len = 0;
        compiler_fence(Ordering::SeqCst);
    }
}
