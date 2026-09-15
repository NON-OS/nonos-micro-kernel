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

use super::ops::{ge_add, ge_double, ge_identity, ge_p1p1_to_p3, ge_to_cached};
use super::pack::ge_basepoint;
use super::types::GeP3;
use crate::crypto::asymmetric::ed25519::field::{fe_cmov, fe_equal, fe_is_zero};

pub(crate) fn ge_scalarmult_base_ct(a: &[u8; 32]) -> GeP3 {
    let base = ge_basepoint();
    ge_scalarmult_ct(&base, a)
}

pub(crate) fn ge_scalarmult_ct(p: &GeP3, scalar: &[u8; 32]) -> GeP3 {
    let mut result = ge_identity();
    let mut temp = *p;

    for i in 0..256 {
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        let bit = (scalar[byte_idx] >> bit_idx) & 1;

        let sum = ge_add(&result, &ge_to_cached(&temp));
        let sum_p3 = ge_p1p1_to_p3(&sum);

        let mask = ct_byte_mask(bit);
        result = ge_cmov(&result, &sum_p3, mask);

        let doubled = ge_double(&temp.to_p2());
        temp = ge_p1p1_to_p3(&doubled);
    }

    result
}

#[inline]
pub(crate) fn ct_byte_mask(bit: u8) -> u8 {
    0u8.wrapping_sub(bit)
}

#[inline]
pub(crate) fn ge_cmov(a: &GeP3, b: &GeP3, mask: u8) -> GeP3 {
    GeP3 {
        x: fe_cmov(&a.x, &b.x, mask),
        y: fe_cmov(&a.y, &b.y, mask),
        z: fe_cmov(&a.z, &b.z, mask),
        t: fe_cmov(&a.t, &b.t, mask),
    }
}

pub(crate) fn ge_scalarmult_vartime(p: &GeP3, scalar: &[u8; 32]) -> GeP3 {
    let mut result = ge_identity();
    let mut temp = *p;

    for i in 0..256 {
        let byte_idx = i / 8;
        let bit_idx = i % 8;
        let bit = (scalar[byte_idx] >> bit_idx) & 1;

        if bit == 1 {
            let sum = ge_add(&result, &ge_to_cached(&temp));
            result = ge_p1p1_to_p3(&sum);
        }

        let doubled = ge_double(&temp.to_p2());
        temp = ge_p1p1_to_p3(&doubled);
    }

    result
}

pub(crate) fn ge_has_large_order(p: &GeP3) -> bool {
    let p2 = ge_p1p1_to_p3(&ge_double(&p.to_p2()));
    let p4 = ge_p1p1_to_p3(&ge_double(&p2.to_p2()));
    let p8 = ge_p1p1_to_p3(&ge_double(&p4.to_p2()));

    let x_is_zero = fe_is_zero(&p8.x);
    let y_eq_z = fe_equal(&p8.y, &p8.z);

    !(x_is_zero && y_eq_z)
}
