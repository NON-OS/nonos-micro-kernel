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

#[inline]
fn rotl(x: u64, b: u32) -> u64 {
    (x << b) | (x >> (64 - b))
}

#[inline]
fn round(v: &mut [u64; 4]) {
    v[0] = v[0].wrapping_add(v[1]);
    v[1] = rotl(v[1], 13);
    v[1] ^= v[0];
    v[0] = rotl(v[0], 32);
    v[2] = v[2].wrapping_add(v[3]);
    v[3] = rotl(v[3], 16);
    v[3] ^= v[2];
    v[0] = v[0].wrapping_add(v[3]);
    v[3] = rotl(v[3], 21);
    v[3] ^= v[0];
    v[2] = v[2].wrapping_add(v[1]);
    v[1] = rotl(v[1], 17);
    v[1] ^= v[2];
    v[2] = rotl(v[2], 32);
}

pub fn siphash24(key: [u64; 2], data: &[u8]) -> u64 {
    let mut v = [
        0x736f_6d65_7073_6575 ^ key[0],
        0x646f_7261_6e64_6f6d ^ key[1],
        0x6c79_6765_6e65_7261 ^ key[0],
        0x7465_6462_7974_6573 ^ key[1],
    ];
    let len = data.len();
    let mut i = 0;
    while i + 8 <= len {
        let mut m = 0u64;
        for j in 0..8 {
            m |= (data[i + j] as u64) << (8 * j);
        }
        v[3] ^= m;
        round(&mut v);
        round(&mut v);
        v[0] ^= m;
        i += 8;
    }
    let mut last = (len as u64) << 56;
    let mut shift = 0;
    while i < len {
        last |= (data[i] as u64) << shift;
        shift += 8;
        i += 1;
    }
    v[3] ^= last;
    round(&mut v);
    round(&mut v);
    v[0] ^= last;
    v[2] ^= 0xff;
    round(&mut v);
    round(&mut v);
    round(&mut v);
    round(&mut v);
    v[0] ^ v[1] ^ v[2] ^ v[3]
}
