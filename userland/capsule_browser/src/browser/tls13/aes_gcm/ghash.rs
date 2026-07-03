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

// GHASH over GF(2^128) as defined by NIST SP 800-38D, used to authenticate the
// additional data and ciphertext under the hash subkey H.

fn gf_mul(x: &[u8; 16], y: &[u8; 16]) -> [u8; 16] {
    let mut z = [0u8; 16];
    let mut v = *y;
    let mut i = 0;
    while i < 128 {
        let bit = (x[i / 8] >> (7 - (i % 8))) & 1;
        if bit == 1 {
            let mut j = 0;
            while j < 16 {
                z[j] ^= v[j];
                j += 1;
            }
        }
        let lsb = v[15] & 1;
        let mut j = 15;
        while j > 0 {
            v[j] = (v[j] >> 1) | ((v[j - 1] & 1) << 7);
            j -= 1;
        }
        v[0] >>= 1;
        if lsb == 1 {
            v[0] ^= 0xe1;
        }
        i += 1;
    }
    z
}

fn absorb(y: &mut [u8; 16], h: &[u8; 16], block: &[u8]) {
    let mut b = [0u8; 16];
    b[..block.len()].copy_from_slice(block);
    let mut j = 0;
    while j < 16 {
        y[j] ^= b[j];
        j += 1;
    }
    *y = gf_mul(y, h);
}

// GHASH(H, aad || ciphertext || len(aad) || len(ciphertext)).
pub fn ghash(h: &[u8; 16], aad: &[u8], ct: &[u8]) -> [u8; 16] {
    let mut y = [0u8; 16];
    for chunk in aad.chunks(16) {
        absorb(&mut y, h, chunk);
    }
    for chunk in ct.chunks(16) {
        absorb(&mut y, h, chunk);
    }
    let mut lens = [0u8; 16];
    lens[..8].copy_from_slice(&((aad.len() as u64) * 8).to_be_bytes());
    lens[8..].copy_from_slice(&((ct.len() as u64) * 8).to_be_bytes());
    let mut j = 0;
    while j < 16 {
        y[j] ^= lens[j];
        j += 1;
    }
    gf_mul(&y, h)
}
