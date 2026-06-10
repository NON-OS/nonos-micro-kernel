// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumType {
    Crc32,
    Sha256,
    Sha512,
    Md5,
}
const CRC32_TABLE: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i = 0u32;
    while i < 256 {
        let mut c = i;
        let mut j = 0;
        while j < 8 {
            c = if c & 1 != 0 { (c >> 1) ^ 0xEDB88320 } else { c >> 1 };
            j += 1;
        }
        t[i as usize] = c;
        i += 1;
    }
    t
};
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

pub fn calculate_crc32(data: &[u8]) -> u32 {
    let mut c = 0xFFFFFFFFu32;
    for &b in data {
        c = (c >> 8) ^ CRC32_TABLE[((c ^ u32::from(b)) & 0xFF) as usize];
    }
    !c
}

pub fn calculate_sha256(data: &[u8]) -> [u8; 32] {
    let mut h = [0u8; 32];
    let mut st: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    for chunk in data.chunks(64) {
        let mut w = [0u32; 64];
        for (i, bc) in chunk.chunks(4).enumerate() {
            w[i] = u32::from_be_bytes([
                bc.get(0).copied().unwrap_or(0),
                bc.get(1).copied().unwrap_or(0),
                bc.get(2).copied().unwrap_or(0),
                bc.get(3).copied().unwrap_or(0),
            ]);
        }
        for i in 16usize..64 {
            w[i] = w[i - 16]
                .wrapping_add(w[i - 7])
                .wrapping_add(
                    w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10),
                )
                .wrapping_add(
                    w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3),
                );
        }
        let mut wv = st;
        for i in 0usize..64 {
            let t1 = wv[7]
                .wrapping_add(
                    wv[4].rotate_right(6) ^ wv[4].rotate_right(11) ^ wv[4].rotate_right(25),
                )
                .wrapping_add((wv[4] & wv[5]) ^ (!wv[4] & wv[6]))
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let t2 = (wv[0].rotate_right(2) ^ wv[0].rotate_right(13) ^ wv[0].rotate_right(22))
                .wrapping_add((wv[0] & wv[1]) ^ (wv[0] & wv[2]) ^ (wv[1] & wv[2]));
            wv = [
                t1.wrapping_add(t2),
                wv[0],
                wv[1],
                wv[2],
                wv[3].wrapping_add(t1),
                wv[4],
                wv[5],
                wv[6],
            ];
        }
        for (i, &v) in wv.iter().enumerate() {
            st[i] = st[i].wrapping_add(v);
        }
    }
    for (c, &v) in h.chunks_mut(4).zip(&st) {
        c.copy_from_slice(&v.to_be_bytes());
    }
    h
}

pub fn verify_checksum(data: &[u8], expected: &[u8], ct: ChecksumType) -> bool {
    match ct {
        ChecksumType::Crc32 => {
            expected.len() == 4 && calculate_crc32(data).to_le_bytes() == expected
        }
        ChecksumType::Sha256 => {
            expected.len() == 32 && calculate_sha256(data).as_slice() == expected
        }
        _ => false,
    }
}
