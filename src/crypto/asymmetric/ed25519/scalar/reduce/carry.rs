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

pub(crate) fn carry_propagate_first(a: &mut [i64; 24]) -> i64 {
    let mut carry: i64;
    carry = (a[6] + (1 << 20)) >> 21;
    a[7] += carry;
    a[6] -= carry << 21;
    carry = (a[8] + (1 << 20)) >> 21;
    a[9] += carry;
    a[8] -= carry << 21;
    carry = (a[10] + (1 << 20)) >> 21;
    a[11] += carry;
    a[10] -= carry << 21;
    carry = (a[12] + (1 << 20)) >> 21;
    a[13] += carry;
    a[12] -= carry << 21;
    carry = (a[14] + (1 << 20)) >> 21;
    a[15] += carry;
    a[14] -= carry << 21;
    carry = (a[16] + (1 << 20)) >> 21;
    a[17] += carry;
    a[16] -= carry << 21;

    carry = (a[7] + (1 << 20)) >> 21;
    a[8] += carry;
    a[7] -= carry << 21;
    carry = (a[9] + (1 << 20)) >> 21;
    a[10] += carry;
    a[9] -= carry << 21;
    carry = (a[11] + (1 << 20)) >> 21;
    a[12] += carry;
    a[11] -= carry << 21;
    carry = (a[13] + (1 << 20)) >> 21;
    a[14] += carry;
    a[13] -= carry << 21;
    carry = (a[15] + (1 << 20)) >> 21;
    a[16] += carry;
    a[15] -= carry << 21;
    carry
}

pub(crate) fn carry_propagate_second(a: &mut [i64; 24], _carry: &mut i64) {
    let mut carry: i64;
    carry = (a[0] + (1 << 20)) >> 21;
    a[1] += carry;
    a[0] -= carry << 21;
    carry = (a[2] + (1 << 20)) >> 21;
    a[3] += carry;
    a[2] -= carry << 21;
    carry = (a[4] + (1 << 20)) >> 21;
    a[5] += carry;
    a[4] -= carry << 21;
    carry = (a[6] + (1 << 20)) >> 21;
    a[7] += carry;
    a[6] -= carry << 21;
    carry = (a[8] + (1 << 20)) >> 21;
    a[9] += carry;
    a[8] -= carry << 21;
    carry = (a[10] + (1 << 20)) >> 21;
    a[11] += carry;
    a[10] -= carry << 21;

    carry = (a[1] + (1 << 20)) >> 21;
    a[2] += carry;
    a[1] -= carry << 21;
    carry = (a[3] + (1 << 20)) >> 21;
    a[4] += carry;
    a[3] -= carry << 21;
    carry = (a[5] + (1 << 20)) >> 21;
    a[6] += carry;
    a[5] -= carry << 21;
    carry = (a[7] + (1 << 20)) >> 21;
    a[8] += carry;
    a[7] -= carry << 21;
    carry = (a[9] + (1 << 20)) >> 21;
    a[10] += carry;
    a[9] -= carry << 21;
    carry = (a[11] + (1 << 20)) >> 21;
    a[12] = carry;
    a[11] -= carry << 21;

    a[0] += a[12] * 666643;
    a[1] += a[12] * 470296;
    a[2] += a[12] * 654183;
    a[3] -= a[12] * 997805;
    a[4] += a[12] * 136657;
    a[5] -= a[12] * 683901;

    carry = a[0] >> 21;
    a[1] += carry;
    a[0] -= carry << 21;
    carry = a[1] >> 21;
    a[2] += carry;
    a[1] -= carry << 21;
    carry = a[2] >> 21;
    a[3] += carry;
    a[2] -= carry << 21;
    carry = a[3] >> 21;
    a[4] += carry;
    a[3] -= carry << 21;
    carry = a[4] >> 21;
    a[5] += carry;
    a[4] -= carry << 21;
    carry = a[5] >> 21;
    a[6] += carry;
    a[5] -= carry << 21;
    carry = a[6] >> 21;
    a[7] += carry;
    a[6] -= carry << 21;
    carry = a[7] >> 21;
    a[8] += carry;
    a[7] -= carry << 21;
    carry = a[8] >> 21;
    a[9] += carry;
    a[8] -= carry << 21;
    carry = a[9] >> 21;
    a[10] += carry;
    a[9] -= carry << 21;
    carry = a[10] >> 21;
    a[11] += carry;
    a[10] -= carry << 21;
}

pub(crate) fn final_reduction(a: &mut [i64; 24]) {
    let carry = a[11] >> 21;
    if carry != 0 {
        a[0] += carry * 666643;
        a[1] += carry * 470296;
        a[2] += carry * 654183;
        a[3] -= carry * 997805;
        a[4] += carry * 136657;
        a[5] -= carry * 683901;
        a[11] -= carry << 21;

        let mut c = a[0] >> 21;
        a[1] += c;
        a[0] -= c << 21;
        c = a[1] >> 21;
        a[2] += c;
        a[1] -= c << 21;
        c = a[2] >> 21;
        a[3] += c;
        a[2] -= c << 21;
        c = a[3] >> 21;
        a[4] += c;
        a[3] -= c << 21;
        c = a[4] >> 21;
        a[5] += c;
        a[4] -= c << 21;
        c = a[5] >> 21;
        a[6] += c;
        a[5] -= c << 21;
        c = a[6] >> 21;
        a[7] += c;
        a[6] -= c << 21;
        c = a[7] >> 21;
        a[8] += c;
        a[7] -= c << 21;
        c = a[8] >> 21;
        a[9] += c;
        a[8] -= c << 21;
        c = a[9] >> 21;
        a[10] += c;
        a[9] -= c << 21;
        c = a[10] >> 21;
        a[11] += c;
        a[10] -= c << 21;
    }
}
