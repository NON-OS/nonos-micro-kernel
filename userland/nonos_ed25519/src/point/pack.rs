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

use super::ops::ge_identity;
use super::types::{GeP3, D};
use crate::field::{
    fe_add, fe_equal, fe_frombytes, fe_invert, fe_is_odd, fe_mul, fe_pow2523, fe_sq, fe_sub,
    fe_tobytes, Fe,
};

pub(crate) fn ge_pack(p: &GeP3) -> [u8; 32] {
    let z_inv = fe_invert(&p.z);
    let x = fe_mul(&p.x, &z_inv);
    let y = fe_mul(&p.y, &z_inv);
    let mut s = fe_tobytes(&y);
    let sign = (fe_is_odd(&x) as u8) & 1;
    s[31] |= sign << 7;
    s
}

pub(crate) fn ge_unpack(s: &[u8; 32]) -> Option<GeP3> {
    let y = fe_frombytes(s);
    let y2 = fe_sq(&y);
    let u = fe_sub(&y2, &Fe::one());
    let v = fe_add(&fe_mul(&D, &y2), &Fe::one());

    let v2 = fe_sq(&v);
    let v3 = fe_mul(&v2, &v);
    let v4 = fe_sq(&v2);
    let v7 = fe_mul(&v3, &v4);
    let uv3 = fe_mul(&u, &v3);
    let uv7 = fe_mul(&u, &v7);
    let mut x = fe_mul(&uv3, &fe_pow2523(&uv7));

    let x2v = fe_mul(&fe_sq(&x), &v);

    if !fe_equal(&x2v, &u) {
        let sqrtm1 = Fe([
            -32595792, -7943725, 9377950, 3500415, 12389472, -272473, -25146209, -2005654, 326686,
            11406482,
        ]);
        x = fe_mul(&x, &sqrtm1);
        let x2v = fe_mul(&fe_sq(&x), &v);
        if !fe_equal(&x2v, &u) {
            return None;
        }
    }

    let sign = (s[31] >> 7) & 1;
    if (fe_is_odd(&x) as u8) != sign {
        x = fe_sub(&Fe::zero(), &x);
    }

    Some(GeP3 { x, y, z: Fe::one(), t: fe_mul(&x, &y) })
}

pub(crate) fn ge_basepoint() -> GeP3 {
    let enc: [u8; 32] = [
        0x58, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66, 0x66,
        0x66, 0x66,
    ];
    ge_unpack(&enc).unwrap_or_else(ge_identity)
}
