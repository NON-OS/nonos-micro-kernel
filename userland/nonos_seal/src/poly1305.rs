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

// Poly1305 one-time authenticator, RFC 8439. Accumulator arithmetic modulo
// 2^130 - 5 on five 26-bit limbs, the standard portable representation.

pub struct Poly1305 {
    r: [u32; 5],
    h: [u32; 5],
    pad: [u32; 4],
    leftover: usize,
    buffer: [u8; 16],
}

impl Poly1305 {
    pub fn new(key: &[u8; 32]) -> Self {
        // Clamp r per the spec.
        let r0 = u32::from_le_bytes([key[0], key[1], key[2], key[3]]) & 0x3ff_ffff;
        let r1 = (u32::from_le_bytes([key[3], key[4], key[5], key[6]]) >> 2) & 0x3ff_ff03;
        let r2 = (u32::from_le_bytes([key[6], key[7], key[8], key[9]]) >> 4) & 0x3ff_c0ff;
        let r3 = (u32::from_le_bytes([key[9], key[10], key[11], key[12]]) >> 6) & 0x3f0_3fff;
        let r4 = (u32::from_le_bytes([key[12], key[13], key[14], key[15]]) >> 8) & 0x00f_ffff;
        Self {
            r: [r0, r1, r2, r3, r4],
            h: [0; 5],
            pad: [
                u32::from_le_bytes([key[16], key[17], key[18], key[19]]),
                u32::from_le_bytes([key[20], key[21], key[22], key[23]]),
                u32::from_le_bytes([key[24], key[25], key[26], key[27]]),
                u32::from_le_bytes([key[28], key[29], key[30], key[31]]),
            ],
            leftover: 0,
            buffer: [0u8; 16],
        }
    }

    fn block(&mut self, m: &[u8; 16], final_block: bool) {
        let hibit: u32 = if final_block { 0 } else { 1 << 24 };

        let t0 = u32::from_le_bytes([m[0], m[1], m[2], m[3]]);
        let t1 = u32::from_le_bytes([m[4], m[5], m[6], m[7]]);
        let t2 = u32::from_le_bytes([m[8], m[9], m[10], m[11]]);
        let t3 = u32::from_le_bytes([m[12], m[13], m[14], m[15]]);

        self.h[0] += t0 & 0x3ff_ffff;
        self.h[1] += ((t0 >> 26) | (t1 << 6)) & 0x3ff_ffff;
        self.h[2] += ((t1 >> 20) | (t2 << 12)) & 0x3ff_ffff;
        self.h[3] += ((t2 >> 14) | (t3 << 18)) & 0x3ff_ffff;
        self.h[4] += (t3 >> 8) | hibit;

        let r = self.r;
        let s1 = r[1] * 5;
        let s2 = r[2] * 5;
        let s3 = r[3] * 5;
        let s4 = r[4] * 5;
        let h = self.h;

        let d0 = mul(h[0], r[0]) + mul(h[1], s4) + mul(h[2], s3) + mul(h[3], s2) + mul(h[4], s1);
        let d1 = mul(h[0], r[1]) + mul(h[1], r[0]) + mul(h[2], s4) + mul(h[3], s3) + mul(h[4], s2);
        let d2 =
            mul(h[0], r[2]) + mul(h[1], r[1]) + mul(h[2], r[0]) + mul(h[3], s4) + mul(h[4], s3);
        let d3 =
            mul(h[0], r[3]) + mul(h[1], r[2]) + mul(h[2], r[1]) + mul(h[3], r[0]) + mul(h[4], s4);
        let d4 =
            mul(h[0], r[4]) + mul(h[1], r[3]) + mul(h[2], r[2]) + mul(h[3], r[1]) + mul(h[4], r[0]);

        let mut c: u64;
        c = d0 >> 26;
        self.h[0] = (d0 as u32) & 0x3ff_ffff;
        let d1 = d1 + c;
        c = d1 >> 26;
        self.h[1] = (d1 as u32) & 0x3ff_ffff;
        let d2 = d2 + c;
        c = d2 >> 26;
        self.h[2] = (d2 as u32) & 0x3ff_ffff;
        let d3 = d3 + c;
        c = d3 >> 26;
        self.h[3] = (d3 as u32) & 0x3ff_ffff;
        let d4 = d4 + c;
        c = d4 >> 26;
        self.h[4] = (d4 as u32) & 0x3ff_ffff;
        self.h[0] += (c as u32) * 5;
        c = (self.h[0] >> 26) as u64;
        self.h[0] &= 0x3ff_ffff;
        self.h[1] += c as u32;
    }

    pub fn update(&mut self, mut data: &[u8]) {
        if self.leftover > 0 {
            let want = core::cmp::min(16 - self.leftover, data.len());
            self.buffer[self.leftover..self.leftover + want].copy_from_slice(&data[..want]);
            self.leftover += want;
            data = &data[want..];
            if self.leftover < 16 {
                return;
            }
            let block = self.buffer;
            self.block(&block, false);
            self.leftover = 0;
        }
        while data.len() >= 16 {
            let mut block = [0u8; 16];
            block.copy_from_slice(&data[..16]);
            self.block(&block, false);
            data = &data[16..];
        }
        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
            self.leftover = data.len();
        }
    }

    // The final reduction carries across limbs in sequence, so the indices are
    // load-bearing and the reduction reads far clearer written against them
    // than as an iterator chain.
    #[allow(clippy::needless_range_loop)]
    pub fn finalize(mut self) -> [u8; 16] {
        if self.leftover > 0 {
            let mut block = [0u8; 16];
            block[..self.leftover].copy_from_slice(&self.buffer[..self.leftover]);
            block[self.leftover] = 1;
            self.block(&block, true);
        }

        // Fully carry h.
        let mut c = self.h[1] >> 26;
        self.h[1] &= 0x3ff_ffff;
        for i in 2..5 {
            self.h[i] += c;
            c = self.h[i] >> 26;
            self.h[i] &= 0x3ff_ffff;
        }
        self.h[0] += c * 5;
        c = self.h[0] >> 26;
        self.h[0] &= 0x3ff_ffff;
        self.h[1] += c;

        // Compute h + -p (i.e. h - (2^130 - 5)) and select if no borrow.
        let mut g = [0u32; 5];
        c = 5;
        for i in 0..5 {
            let v = self.h[i].wrapping_add(c);
            g[i] = v & 0x3ff_ffff;
            c = v >> 26;
        }
        g[4] = g[4].wrapping_sub(1 << 26);

        let mask = (g[4] >> 31).wrapping_sub(1);
        for i in 0..5 {
            g[i] &= mask;
        }
        let nmask = !mask;
        for i in 0..5 {
            self.h[i] = (self.h[i] & nmask) | g[i];
        }

        // Serialize h as a 128-bit little-endian number, add pad.
        let h0 = self.h[0] | (self.h[1] << 26);
        let h1 = (self.h[1] >> 6) | (self.h[2] << 20);
        let h2 = (self.h[2] >> 12) | (self.h[3] << 14);
        let h3 = (self.h[3] >> 18) | (self.h[4] << 8);

        let mut f = (h0 as u64) + (self.pad[0] as u64);
        let mut tag = [0u8; 16];
        tag[0..4].copy_from_slice(&(f as u32).to_le_bytes());
        f = (h1 as u64) + (self.pad[1] as u64) + (f >> 32);
        tag[4..8].copy_from_slice(&(f as u32).to_le_bytes());
        f = (h2 as u64) + (self.pad[2] as u64) + (f >> 32);
        tag[8..12].copy_from_slice(&(f as u32).to_le_bytes());
        f = (h3 as u64) + (self.pad[3] as u64) + (f >> 32);
        tag[12..16].copy_from_slice(&(f as u32).to_le_bytes());
        tag
    }
}

#[inline]
fn mul(a: u32, b: u32) -> u64 {
    (a as u64) * (b as u64)
}
