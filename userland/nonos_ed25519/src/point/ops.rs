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

use super::types::{GeCached, GeP1P1, GeP2, GeP3, D2};
use crate::field::{fe_add, fe_copy, fe_mul, fe_sq, fe_sub};

#[inline]
pub(crate) fn ge_identity() -> GeP3 {
    GeP3::identity()
}

#[inline]
pub(crate) fn ge_to_cached(p: &GeP3) -> GeCached {
    GeCached {
        y_plus_x: fe_add(&p.y, &p.x),
        y_minus_x: fe_sub(&p.y, &p.x),
        z: fe_copy(&p.z),
        t2d: fe_mul(&p.t, &D2),
    }
}

pub(crate) fn ge_add(p: &GeP3, q: &GeCached) -> GeP1P1 {
    let y_plus_x = fe_add(&p.y, &p.x);
    let y_minus_x = fe_sub(&p.y, &p.x);
    let pp = fe_mul(&y_plus_x, &q.y_plus_x);
    let mm = fe_mul(&y_minus_x, &q.y_minus_x);
    let tt2d = fe_mul(&p.t, &q.t2d);
    let zz = fe_mul(&p.z, &q.z);
    let zz2 = fe_add(&zz, &zz);
    GeP1P1 {
        x: fe_sub(&pp, &mm),
        y: fe_add(&pp, &mm),
        z: fe_add(&zz2, &tt2d),
        t: fe_sub(&zz2, &tt2d),
    }
}

pub(crate) fn ge_double(p: &GeP2) -> GeP1P1 {
    let xx = fe_sq(&p.x);
    let yy = fe_sq(&p.y);
    let zz2 = fe_add(&fe_sq(&p.z), &fe_sq(&p.z));
    let x_p_y = fe_add(&p.x, &p.y);
    let x_p_y2 = fe_sq(&x_p_y);
    let yy_p_xx = fe_add(&yy, &xx);
    let yy_m_xx = fe_sub(&yy, &xx);
    let e = fe_sub(&x_p_y2, &yy_p_xx);
    let f = fe_sub(&zz2, &yy_m_xx);
    GeP1P1 { x: e, y: yy_p_xx, z: yy_m_xx, t: f }
}

#[inline]
pub(crate) fn ge_p1p1_to_p3(r: &GeP1P1) -> GeP3 {
    let x = fe_mul(&r.x, &r.t);
    let y = fe_mul(&r.y, &r.z);
    let z = fe_mul(&r.z, &r.t);
    let t = fe_mul(&r.x, &r.y);
    GeP3 { x, y, z, t }
}
