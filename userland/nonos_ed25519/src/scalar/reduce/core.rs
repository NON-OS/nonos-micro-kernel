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

use super::carry::{carry_propagate_first, carry_propagate_second, final_reduction};
use super::limbs::{reduce_high_limbs, reduce_mid_limbs};
use super::pack::pack_output;
use crate::field::{load3, load4};

pub(crate) fn sc_reduce_mod_l(s: &mut [u8; 64]) -> [u8; 32] {
    let mut a = [0i64; 24];
    a[0] = 2097151 & load3(&s[0..]);
    a[1] = 2097151 & (load4(&s[2..]) >> 5);
    a[2] = 2097151 & (load3(&s[5..]) >> 2);
    a[3] = 2097151 & (load4(&s[7..]) >> 7);
    a[4] = 2097151 & (load4(&s[10..]) >> 4);
    a[5] = 2097151 & (load3(&s[13..]) >> 1);
    a[6] = 2097151 & (load4(&s[15..]) >> 6);
    a[7] = 2097151 & (load3(&s[18..]) >> 3);
    a[8] = 2097151 & load3(&s[21..]);
    a[9] = 2097151 & (load4(&s[23..]) >> 5);
    a[10] = 2097151 & (load3(&s[26..]) >> 2);
    a[11] = 2097151 & (load4(&s[28..]) >> 7);
    a[12] = 2097151 & (load4(&s[31..]) >> 4);
    a[13] = 2097151 & (load3(&s[34..]) >> 1);
    a[14] = 2097151 & (load4(&s[36..]) >> 6);
    a[15] = 2097151 & (load3(&s[39..]) >> 3);
    a[16] = 2097151 & load3(&s[42..]);
    a[17] = 2097151 & (load4(&s[44..]) >> 5);
    a[18] = 2097151 & (load3(&s[47..]) >> 2);
    a[19] = 2097151 & (load4(&s[49..]) >> 7);
    a[20] = 2097151 & (load4(&s[52..]) >> 4);
    a[21] = 2097151 & (load3(&s[55..]) >> 1);
    a[22] = 2097151 & (load4(&s[57..]) >> 6);
    a[23] = load4(&s[60..]) >> 3;

    reduce_high_limbs(&mut a);
    let mut carry = carry_propagate_first(&mut a);
    reduce_mid_limbs(&mut a);
    carry_propagate_second(&mut a, &mut carry);
    final_reduction(&mut a);

    pack_output(&a)
}
