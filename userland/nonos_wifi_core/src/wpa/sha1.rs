// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! SHA-1 (RFC 3174). Deprecated as a general hash, but WPA2 is defined over it:
//! HMAC-SHA1, PBKDF2 and the 802.11i PRF all build on this. Streaming, no_std,
//! no allocation. Checked against RFC known-answer vectors in `iwlwifi_proofs`.

pub struct Sha1 {
    h: [u32; 5],
    block: [u8; 64],
    block_len: usize,
    total_len: u64,
}

impl Sha1 {
    pub fn new() -> Self {
        Self {
            h: [0x6745_2301, 0xEFCD_AB89, 0x98BA_DCFE, 0x1032_5476, 0xC3D2_E1F0],
            block: [0u8; 64],
            block_len: 0,
            total_len: 0,
        }
    }

    pub fn update(&mut self, mut data: &[u8]) {
        self.total_len = self.total_len.wrapping_add(data.len() as u64);
        while !data.is_empty() {
            let n = core::cmp::min(64 - self.block_len, data.len());
            self.block[self.block_len..self.block_len + n].copy_from_slice(&data[..n]);
            self.block_len += n;
            data = &data[n..];
            if self.block_len == 64 {
                self.process();
                self.block_len = 0;
            }
        }
    }

    fn process(&mut self) {
        let mut w = [0u32; 80];
        for (wi, chunk) in w[..16].iter_mut().zip(self.block.chunks_exact(4)) {
            *wi = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let [mut a, mut b, mut c, mut d, mut e] = self.h;
        for (i, &wi) in w.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | ((!b) & d), 0x5A82_7999u32),
                20..=39 => (b ^ c ^ d, 0x6ED9_EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1B_BCDC),
                _ => (b ^ c ^ d, 0xCA62_C1D6),
            };
            let t = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(wi);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = t;
        }
        self.h[0] = self.h[0].wrapping_add(a);
        self.h[1] = self.h[1].wrapping_add(b);
        self.h[2] = self.h[2].wrapping_add(c);
        self.h[3] = self.h[3].wrapping_add(d);
        self.h[4] = self.h[4].wrapping_add(e);
    }

    pub fn finalize(mut self) -> [u8; 20] {
        let bit_len = self.total_len.wrapping_mul(8);
        self.block[self.block_len] = 0x80;
        self.block_len += 1;
        if self.block_len > 56 {
            for i in self.block_len..64 {
                self.block[i] = 0;
            }
            self.process();
            self.block_len = 0;
        }
        for i in self.block_len..56 {
            self.block[i] = 0;
        }
        self.block[56..64].copy_from_slice(&bit_len.to_be_bytes());
        self.process();
        let mut out = [0u8; 20];
        for (chunk, hv) in out.chunks_exact_mut(4).zip(self.h.iter()) {
            chunk.copy_from_slice(&hv.to_be_bytes());
        }
        out
    }
}

impl Default for Sha1 {
    fn default() -> Self {
        Self::new()
    }
}

/// One-shot SHA-1 of a byte string.
pub fn sha1(data: &[u8]) -> [u8; 20] {
    let mut h = Sha1::new();
    h.update(data);
    h.finalize()
}
