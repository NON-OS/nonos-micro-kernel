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

use crate::crypto::constant_time::{compiler_fence, secure_zero};
use crate::crypto::symmetric::aes::{Aes128, Aes256};

pub(super) const BLOCK_SIZE: usize = 16;

const GF128_R: u64 = 0xE100_0000_0000_0000;

#[derive(Clone)]
pub(super) struct GhashKey {
    h: (u64, u64),
}

impl GhashKey {
    pub(super) fn new(aes: &Aes256) -> Self {
        let zero = [0u8; BLOCK_SIZE];
        let h_block = aes.encrypt_block(&zero);
        let h = block_to_u128(&h_block);
        Self { h }
    }

    pub(super) fn new_128(aes: &Aes128) -> Self {
        let zero = [0u8; BLOCK_SIZE];
        let h_block = aes.encrypt_block(&zero);
        let h = block_to_u128(&h_block);
        Self { h }
    }

    #[inline]
    pub(super) fn mul(&self, x: (u64, u64)) -> (u64, u64) {
        gf128_mul_bitwise(x, self.h)
    }
}

impl Drop for GhashKey {
    fn drop(&mut self) {
        // SAFETY: Using volatile writes to prevent compiler from optimizing away the zeroing
        unsafe {
            core::ptr::write_volatile(&mut self.h.0, 0);
            core::ptr::write_volatile(&mut self.h.1, 0);
        }
        compiler_fence();
    }
}

#[inline]
pub(super) fn block_to_u128(block: &[u8; 16]) -> (u64, u64) {
    let hi = u64::from_be_bytes([
        block[0], block[1], block[2], block[3], block[4], block[5], block[6], block[7],
    ]);
    let lo = u64::from_be_bytes([
        block[8], block[9], block[10], block[11], block[12], block[13], block[14], block[15],
    ]);
    (hi, lo)
}

#[inline]
pub(super) fn u128_to_block(val: (u64, u64)) -> [u8; 16] {
    let mut block = [0u8; 16];
    block[0..8].copy_from_slice(&val.0.to_be_bytes());
    block[8..16].copy_from_slice(&val.1.to_be_bytes());
    block
}

#[inline(always)]
pub(super) fn gf128_xor(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    (a.0 ^ b.0, a.1 ^ b.1)
}

#[inline(never)]
pub(super) fn gf128_mul_bitwise(x: (u64, u64), y: (u64, u64)) -> (u64, u64) {
    let mut z = (0u64, 0u64);
    let mut v = y;

    for i in 0..64 {
        let bit = (x.0 >> (63 - i)) & 1;
        let mask = 0u64.wrapping_sub(bit);
        z.0 ^= v.0 & mask;
        z.1 ^= v.1 & mask;

        let lsb = v.1 & 1;
        v.1 = (v.1 >> 1) | (v.0 << 63);
        v.0 >>= 1;
        let reduce_mask = 0u64.wrapping_sub(lsb);
        v.0 ^= reduce_mask & GF128_R;
    }

    for i in 0..64 {
        let bit = (x.1 >> (63 - i)) & 1;
        let mask = 0u64.wrapping_sub(bit);
        z.0 ^= v.0 & mask;
        z.1 ^= v.1 & mask;

        let lsb = v.1 & 1;
        v.1 = (v.1 >> 1) | (v.0 << 63);
        v.0 >>= 1;
        let reduce_mask = 0u64.wrapping_sub(lsb);
        v.0 ^= reduce_mask & GF128_R;
    }

    z
}

pub(super) struct GhashState {
    key: GhashKey,
    y: (u64, u64),
    aad_len: u64,
    ct_len: u64,
    buffer: [u8; 16],
    buffer_len: usize,
    aad_done: bool,
}

impl GhashState {
    pub(super) fn new(key: GhashKey) -> Self {
        Self {
            key,
            y: (0, 0),
            aad_len: 0,
            ct_len: 0,
            buffer: [0u8; 16],
            buffer_len: 0,
            aad_done: false,
        }
    }

    pub(super) fn update_aad(&mut self, data: &[u8]) {
        if self.aad_done {
            return;
        }
        self.aad_len += data.len() as u64;
        self.update_internal(data);
    }

    pub(super) fn finalize_aad(&mut self) {
        if self.aad_done {
            return;
        }

        if self.buffer_len > 0 {
            for i in self.buffer_len..16 {
                self.buffer[i] = 0;
            }
            let block = block_to_u128(&self.buffer);
            self.y = self.key.mul(gf128_xor(self.y, block));
            self.buffer_len = 0;
        }

        self.aad_done = true;
    }

    pub(super) fn update_ct(&mut self, data: &[u8]) {
        if !self.aad_done {
            self.finalize_aad();
        }
        self.ct_len += data.len() as u64;
        self.update_internal(data);
    }

    fn update_internal(&mut self, data: &[u8]) {
        let mut offset = 0;

        if self.buffer_len > 0 {
            let need = 16 - self.buffer_len;
            let take = need.min(data.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&data[..take]);
            self.buffer_len += take;
            offset = take;

            if self.buffer_len == 16 {
                let block = block_to_u128(&self.buffer);
                self.y = self.key.mul(gf128_xor(self.y, block));
                self.buffer_len = 0;
            }
        }

        while offset + 16 <= data.len() {
            let mut block = [0u8; 16];
            block.copy_from_slice(&data[offset..offset + 16]);
            let x = block_to_u128(&block);
            self.y = self.key.mul(gf128_xor(self.y, x));
            offset += 16;
        }

        if offset < data.len() {
            let remaining = data.len() - offset;
            self.buffer[..remaining].copy_from_slice(&data[offset..]);
            self.buffer_len = remaining;
        }
    }

    pub(super) fn finalize(mut self) -> (u64, u64) {
        if !self.aad_done {
            self.finalize_aad();
        }

        if self.buffer_len > 0 {
            for i in self.buffer_len..16 {
                self.buffer[i] = 0;
            }
            let block = block_to_u128(&self.buffer);
            self.y = self.key.mul(gf128_xor(self.y, block));
        }

        let len_block = (self.aad_len * 8, self.ct_len * 8);

        self.key.mul(gf128_xor(self.y, len_block))
    }
}

impl Drop for GhashState {
    fn drop(&mut self) {
        self.y = (0, 0);
        secure_zero(&mut self.buffer);
        self.buffer_len = 0;
        compiler_fence();
    }
}
