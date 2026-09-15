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

use super::types::Fe;

pub(crate) fn fe_tobytes(f: &Fe) -> [u8; 32] {
    let mut h = [0i64; 10];
    for (out, &limb) in h.iter_mut().zip(f.0.iter()) {
        *out = limb as i64;
    }

    let mut carry: i64;
    carry = (h[0] + (1 << 25)) >> 26;
    h[1] += carry;
    h[0] -= carry << 26;
    carry = (h[4] + (1 << 25)) >> 26;
    h[5] += carry;
    h[4] -= carry << 26;
    carry = (h[1] + (1 << 24)) >> 25;
    h[2] += carry;
    h[1] -= carry << 25;
    carry = (h[5] + (1 << 24)) >> 25;
    h[6] += carry;
    h[5] -= carry << 25;
    carry = (h[2] + (1 << 25)) >> 26;
    h[3] += carry;
    h[2] -= carry << 26;
    carry = (h[6] + (1 << 25)) >> 26;
    h[7] += carry;
    h[6] -= carry << 26;
    carry = (h[3] + (1 << 24)) >> 25;
    h[4] += carry;
    h[3] -= carry << 25;
    carry = (h[7] + (1 << 24)) >> 25;
    h[8] += carry;
    h[7] -= carry << 25;
    carry = (h[4] + (1 << 25)) >> 26;
    h[5] += carry;
    h[4] -= carry << 26;
    carry = (h[8] + (1 << 25)) >> 26;
    h[9] += carry;
    h[8] -= carry << 26;
    carry = (h[9] + (1 << 24)) >> 25;
    h[0] += carry * 19;
    h[9] -= carry << 25;

    carry = (h[0] + (1 << 25)) >> 26;
    h[1] += carry;
    h[0] -= carry << 26;

    carry = (h[0] + 19) >> 26;
    carry = (h[1] + carry) >> 25;
    carry = (h[2] + carry) >> 26;
    carry = (h[3] + carry) >> 25;
    carry = (h[4] + carry) >> 26;
    carry = (h[5] + carry) >> 25;
    carry = (h[6] + carry) >> 26;
    carry = (h[7] + carry) >> 25;
    carry = (h[8] + carry) >> 26;
    carry = (h[9] + carry) >> 25;

    h[0] += carry * 19;

    carry = h[0] >> 26;
    h[1] += carry;
    h[0] -= carry << 26;
    carry = h[1] >> 25;
    h[2] += carry;
    h[1] -= carry << 25;
    carry = h[2] >> 26;
    h[3] += carry;
    h[2] -= carry << 26;
    carry = h[3] >> 25;
    h[4] += carry;
    h[3] -= carry << 25;
    carry = h[4] >> 26;
    h[5] += carry;
    h[4] -= carry << 26;
    carry = h[5] >> 25;
    h[6] += carry;
    h[5] -= carry << 25;
    carry = h[6] >> 26;
    h[7] += carry;
    h[6] -= carry << 26;
    carry = h[7] >> 25;
    h[8] += carry;
    h[7] -= carry << 25;
    carry = h[8] >> 26;
    h[9] += carry;
    h[8] -= carry << 26;
    h[9] &= (1 << 25) - 1;

    let mut s = [0u8; 32];
    s[0] = h[0] as u8;
    s[1] = (h[0] >> 8) as u8;
    s[2] = (h[0] >> 16) as u8;
    s[3] = ((h[0] >> 24) | (h[1] << 2)) as u8;
    s[4] = (h[1] >> 6) as u8;
    s[5] = (h[1] >> 14) as u8;
    s[6] = ((h[1] >> 22) | (h[2] << 3)) as u8;
    s[7] = (h[2] >> 5) as u8;
    s[8] = (h[2] >> 13) as u8;
    s[9] = ((h[2] >> 21) | (h[3] << 5)) as u8;
    s[10] = (h[3] >> 3) as u8;
    s[11] = (h[3] >> 11) as u8;
    s[12] = ((h[3] >> 19) | (h[4] << 6)) as u8;
    s[13] = (h[4] >> 2) as u8;
    s[14] = (h[4] >> 10) as u8;
    s[15] = (h[4] >> 18) as u8;
    s[16] = h[5] as u8;
    s[17] = (h[5] >> 8) as u8;
    s[18] = (h[5] >> 16) as u8;
    s[19] = ((h[5] >> 24) | (h[6] << 1)) as u8;
    s[20] = (h[6] >> 7) as u8;
    s[21] = (h[6] >> 15) as u8;
    s[22] = ((h[6] >> 23) | (h[7] << 3)) as u8;
    s[23] = (h[7] >> 5) as u8;
    s[24] = (h[7] >> 13) as u8;
    s[25] = ((h[7] >> 21) | (h[8] << 4)) as u8;
    s[26] = (h[8] >> 4) as u8;
    s[27] = (h[8] >> 12) as u8;
    s[28] = ((h[8] >> 20) | (h[9] << 6)) as u8;
    s[29] = (h[9] >> 2) as u8;
    s[30] = (h[9] >> 10) as u8;
    s[31] = (h[9] >> 18) as u8;
    s
}

pub(crate) fn fe_frombytes(s: &[u8; 32]) -> Fe {
    let h0 = load4(&s[0..4]);
    let h1 = load3(&s[4..7]) << 6;
    let h2 = load3(&s[7..10]) << 5;
    let h3 = load3(&s[10..13]) << 3;
    let h4 = load3(&s[13..16]) << 2;
    let h5 = load4(&s[16..20]);
    let h6 = load3(&s[20..23]) << 7;
    let h7 = load3(&s[23..26]) << 5;
    let h8 = load3(&s[26..29]) << 4;
    let h9 = (load3(&s[29..32]) & 0x7fffff) << 2;

    let mut carry: i64;
    let mut h = [h0, h1, h2, h3, h4, h5, h6, h7, h8, h9];

    carry = (h[9] + (1 << 24)) >> 25;
    h[0] += carry * 19;
    h[9] -= carry << 25;
    carry = (h[1] + (1 << 24)) >> 25;
    h[2] += carry;
    h[1] -= carry << 25;
    carry = (h[3] + (1 << 24)) >> 25;
    h[4] += carry;
    h[3] -= carry << 25;
    carry = (h[5] + (1 << 24)) >> 25;
    h[6] += carry;
    h[5] -= carry << 25;
    carry = (h[7] + (1 << 24)) >> 25;
    h[8] += carry;
    h[7] -= carry << 25;

    carry = (h[0] + (1 << 25)) >> 26;
    h[1] += carry;
    h[0] -= carry << 26;
    carry = (h[2] + (1 << 25)) >> 26;
    h[3] += carry;
    h[2] -= carry << 26;
    carry = (h[4] + (1 << 25)) >> 26;
    h[5] += carry;
    h[4] -= carry << 26;
    carry = (h[6] + (1 << 25)) >> 26;
    h[7] += carry;
    h[6] -= carry << 26;
    carry = (h[8] + (1 << 25)) >> 26;
    h[9] += carry;
    h[8] -= carry << 26;

    Fe([
        h[0] as i32,
        h[1] as i32,
        h[2] as i32,
        h[3] as i32,
        h[4] as i32,
        h[5] as i32,
        h[6] as i32,
        h[7] as i32,
        h[8] as i32,
        h[9] as i32,
    ])
}

#[inline]
pub(crate) fn load3(s: &[u8]) -> i64 {
    (s[0] as i64) | ((s[1] as i64) << 8) | ((s[2] as i64) << 16)
}

#[inline]
pub(crate) fn load4(s: &[u8]) -> i64 {
    (s[0] as i64) | ((s[1] as i64) << 8) | ((s[2] as i64) << 16) | ((s[3] as i64) << 24)
}
