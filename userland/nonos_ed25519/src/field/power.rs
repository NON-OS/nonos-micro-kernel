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

use super::arithmetic::{fe_mul, fe_sq};
use super::types::Fe;

pub(crate) fn fe_invert(z: &Fe) -> Fe {
    let z1 = *z;
    let z2 = fe_sq(&z1);
    let z4 = fe_sq(&z2);
    let z8 = fe_sq(&z4);
    let z9 = fe_mul(&z8, &z1);
    let z11 = fe_mul(&z9, &z2);
    let z22 = fe_sq(&z11);
    let z_5_0 = fe_mul(&z22, &z9);

    let mut t = fe_sq(&z_5_0);
    for _ in 1..5 {
        t = fe_sq(&t);
    }
    let z_10_5 = fe_mul(&t, &z_5_0);

    t = fe_sq(&z_10_5);
    for _ in 1..10 {
        t = fe_sq(&t);
    }
    let z_20_10 = fe_mul(&t, &z_10_5);

    t = fe_sq(&z_20_10);
    for _ in 1..20 {
        t = fe_sq(&t);
    }
    let z_40_20 = fe_mul(&t, &z_20_10);

    t = fe_sq(&z_40_20);
    for _ in 1..10 {
        t = fe_sq(&t);
    }
    let z_50_10 = fe_mul(&t, &z_10_5);

    t = fe_sq(&z_50_10);
    for _ in 1..50 {
        t = fe_sq(&t);
    }
    let z_100_50 = fe_mul(&t, &z_50_10);

    t = fe_sq(&z_100_50);
    for _ in 1..100 {
        t = fe_sq(&t);
    }
    let z_200_100 = fe_mul(&t, &z_100_50);

    t = fe_sq(&z_200_100);
    for _ in 1..50 {
        t = fe_sq(&t);
    }
    let z_250_50 = fe_mul(&t, &z_50_10);

    t = fe_sq(&z_250_50);
    for _ in 1..5 {
        t = fe_sq(&t);
    }
    fe_mul(&t, &z11)
}

pub(crate) fn fe_pow2523(z: &Fe) -> Fe {
    let z1 = *z;
    let z2 = fe_sq(&z1);
    let z4 = fe_sq(&z2);
    let z8 = fe_sq(&z4);
    let z9 = fe_mul(&z8, &z1);
    let z11 = fe_mul(&z9, &z2);
    let z22 = fe_sq(&z11);
    let z_5_0 = fe_mul(&z22, &z9);

    let mut t = fe_sq(&z_5_0);
    for _ in 1..5 {
        t = fe_sq(&t);
    }
    let z_10_5 = fe_mul(&t, &z_5_0);

    t = fe_sq(&z_10_5);
    for _ in 1..10 {
        t = fe_sq(&t);
    }
    let z_20_10 = fe_mul(&t, &z_10_5);

    t = fe_sq(&z_20_10);
    for _ in 1..20 {
        t = fe_sq(&t);
    }
    let z_40_20 = fe_mul(&t, &z_20_10);

    t = fe_sq(&z_40_20);
    for _ in 1..10 {
        t = fe_sq(&t);
    }
    let z_50_10 = fe_mul(&t, &z_10_5);

    t = fe_sq(&z_50_10);
    for _ in 1..50 {
        t = fe_sq(&t);
    }
    let z_100_50 = fe_mul(&t, &z_50_10);

    t = fe_sq(&z_100_50);
    for _ in 1..100 {
        t = fe_sq(&t);
    }
    let z_200_100 = fe_mul(&t, &z_100_50);

    t = fe_sq(&z_200_100);
    for _ in 1..50 {
        t = fe_sq(&t);
    }
    let z_250_50 = fe_mul(&t, &z_50_10);

    t = fe_sq(&z_250_50);
    t = fe_sq(&t);
    fe_mul(&t, &z1)
}
