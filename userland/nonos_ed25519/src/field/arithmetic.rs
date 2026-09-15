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

#[inline]
pub(crate) fn fe_add(a: &Fe, b: &Fe) -> Fe {
    let mut r = [0i32; 10];
    for (out, (x, y)) in r.iter_mut().zip(a.0.iter().zip(b.0.iter())) {
        *out = x + y;
    }
    Fe(r)
}

#[inline]
pub(crate) fn fe_sub(a: &Fe, b: &Fe) -> Fe {
    let mut r = [0i32; 10];
    for (out, (x, y)) in r.iter_mut().zip(a.0.iter().zip(b.0.iter())) {
        *out = x - y;
    }
    Fe(r)
}

pub(crate) fn fe_mul(a: &Fe, b: &Fe) -> Fe {
    let a0 = a.0[0] as i64;
    let a1 = a.0[1] as i64;
    let a2 = a.0[2] as i64;
    let a3 = a.0[3] as i64;
    let a4 = a.0[4] as i64;
    let a5 = a.0[5] as i64;
    let a6 = a.0[6] as i64;
    let a7 = a.0[7] as i64;
    let a8 = a.0[8] as i64;
    let a9 = a.0[9] as i64;

    let b0 = b.0[0] as i64;
    let b1 = b.0[1] as i64;
    let b2 = b.0[2] as i64;
    let b3 = b.0[3] as i64;
    let b4 = b.0[4] as i64;
    let b5 = b.0[5] as i64;
    let b6 = b.0[6] as i64;
    let b7 = b.0[7] as i64;
    let b8 = b.0[8] as i64;
    let b9 = b.0[9] as i64;

    let b1_19 = b1 * 19;
    let b2_19 = b2 * 19;
    let b3_19 = b3 * 19;
    let b4_19 = b4 * 19;
    let b5_19 = b5 * 19;
    let b6_19 = b6 * 19;
    let b7_19 = b7 * 19;
    let b8_19 = b8 * 19;
    let b9_19 = b9 * 19;
    let a1_2 = a1 * 2;
    let a3_2 = a3 * 2;
    let a5_2 = a5 * 2;
    let a7_2 = a7 * 2;
    let a9_2 = a9 * 2;

    let mut c0 = a0 * b0
        + a1_2 * b9_19
        + a2 * b8_19
        + a3_2 * b7_19
        + a4 * b6_19
        + a5_2 * b5_19
        + a6 * b4_19
        + a7_2 * b3_19
        + a8 * b2_19
        + a9_2 * b1_19;
    let mut c1 = a0 * b1
        + a1 * b0
        + a2 * b9_19
        + a3 * b8_19
        + a4 * b7_19
        + a5 * b6_19
        + a6 * b5_19
        + a7 * b4_19
        + a8 * b3_19
        + a9 * b2_19;
    let mut c2 = a0 * b2
        + a1_2 * b1
        + a2 * b0
        + a3_2 * b9_19
        + a4 * b8_19
        + a5_2 * b7_19
        + a6 * b6_19
        + a7_2 * b5_19
        + a8 * b4_19
        + a9_2 * b3_19;
    let mut c3 = a0 * b3
        + a1 * b2
        + a2 * b1
        + a3 * b0
        + a4 * b9_19
        + a5 * b8_19
        + a6 * b7_19
        + a7 * b6_19
        + a8 * b5_19
        + a9 * b4_19;
    let mut c4 = a0 * b4
        + a1_2 * b3
        + a2 * b2
        + a3_2 * b1
        + a4 * b0
        + a5_2 * b9_19
        + a6 * b8_19
        + a7_2 * b7_19
        + a8 * b6_19
        + a9_2 * b5_19;
    let mut c5 = a0 * b5
        + a1 * b4
        + a2 * b3
        + a3 * b2
        + a4 * b1
        + a5 * b0
        + a6 * b9_19
        + a7 * b8_19
        + a8 * b7_19
        + a9 * b6_19;
    let mut c6 = a0 * b6
        + a1_2 * b5
        + a2 * b4
        + a3_2 * b3
        + a4 * b2
        + a5_2 * b1
        + a6 * b0
        + a7_2 * b9_19
        + a8 * b8_19
        + a9_2 * b7_19;
    let mut c7 = a0 * b7
        + a1 * b6
        + a2 * b5
        + a3 * b4
        + a4 * b3
        + a5 * b2
        + a6 * b1
        + a7 * b0
        + a8 * b9_19
        + a9 * b8_19;
    let mut c8 = a0 * b8
        + a1_2 * b7
        + a2 * b6
        + a3_2 * b5
        + a4 * b4
        + a5_2 * b3
        + a6 * b2
        + a7_2 * b1
        + a8 * b0
        + a9_2 * b9_19;
    let mut c9 = a0 * b9
        + a1 * b8
        + a2 * b7
        + a3 * b6
        + a4 * b5
        + a5 * b4
        + a6 * b3
        + a7 * b2
        + a8 * b1
        + a9 * b0;

    let mut carry: i64;

    carry = (c0 + (1 << 25)) >> 26;
    c1 += carry;
    c0 -= carry << 26;
    carry = (c4 + (1 << 25)) >> 26;
    c5 += carry;
    c4 -= carry << 26;
    carry = (c1 + (1 << 24)) >> 25;
    c2 += carry;
    c1 -= carry << 25;
    carry = (c5 + (1 << 24)) >> 25;
    c6 += carry;
    c5 -= carry << 25;
    carry = (c2 + (1 << 25)) >> 26;
    c3 += carry;
    c2 -= carry << 26;
    carry = (c6 + (1 << 25)) >> 26;
    c7 += carry;
    c6 -= carry << 26;
    carry = (c3 + (1 << 24)) >> 25;
    c4 += carry;
    c3 -= carry << 25;
    carry = (c7 + (1 << 24)) >> 25;
    c8 += carry;
    c7 -= carry << 25;
    carry = (c4 + (1 << 25)) >> 26;
    c5 += carry;
    c4 -= carry << 26;
    carry = (c8 + (1 << 25)) >> 26;
    c9 += carry;
    c8 -= carry << 26;
    carry = (c9 + (1 << 24)) >> 25;
    c0 += carry * 19;
    c9 -= carry << 25;

    carry = (c0 + (1 << 25)) >> 26;
    c1 += carry;
    c0 -= carry << 26;

    Fe([
        c0 as i32, c1 as i32, c2 as i32, c3 as i32, c4 as i32, c5 as i32, c6 as i32, c7 as i32,
        c8 as i32, c9 as i32,
    ])
}

#[inline]
pub(crate) fn fe_sq(a: &Fe) -> Fe {
    fe_mul(a, a)
}
