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

pub(crate) fn reduce_high_limbs(a: &mut [i64; 24]) {
    a[11] += a[23] * 666643;
    a[12] += a[23] * 470296;
    a[13] += a[23] * 654183;
    a[14] -= a[23] * 997805;
    a[15] += a[23] * 136657;
    a[16] -= a[23] * 683901;

    a[10] += a[22] * 666643;
    a[11] += a[22] * 470296;
    a[12] += a[22] * 654183;
    a[13] -= a[22] * 997805;
    a[14] += a[22] * 136657;
    a[15] -= a[22] * 683901;

    a[9] += a[21] * 666643;
    a[10] += a[21] * 470296;
    a[11] += a[21] * 654183;
    a[12] -= a[21] * 997805;
    a[13] += a[21] * 136657;
    a[14] -= a[21] * 683901;

    a[8] += a[20] * 666643;
    a[9] += a[20] * 470296;
    a[10] += a[20] * 654183;
    a[11] -= a[20] * 997805;
    a[12] += a[20] * 136657;
    a[13] -= a[20] * 683901;

    a[7] += a[19] * 666643;
    a[8] += a[19] * 470296;
    a[9] += a[19] * 654183;
    a[10] -= a[19] * 997805;
    a[11] += a[19] * 136657;
    a[12] -= a[19] * 683901;

    a[6] += a[18] * 666643;
    a[7] += a[18] * 470296;
    a[8] += a[18] * 654183;
    a[9] -= a[18] * 997805;
    a[10] += a[18] * 136657;
    a[11] -= a[18] * 683901;
}

pub(crate) fn reduce_mid_limbs(a: &mut [i64; 24]) {
    a[5] += a[17] * 666643;
    a[6] += a[17] * 470296;
    a[7] += a[17] * 654183;
    a[8] -= a[17] * 997805;
    a[9] += a[17] * 136657;
    a[10] -= a[17] * 683901;

    a[4] += a[16] * 666643;
    a[5] += a[16] * 470296;
    a[6] += a[16] * 654183;
    a[7] -= a[16] * 997805;
    a[8] += a[16] * 136657;
    a[9] -= a[16] * 683901;

    a[3] += a[15] * 666643;
    a[4] += a[15] * 470296;
    a[5] += a[15] * 654183;
    a[6] -= a[15] * 997805;
    a[7] += a[15] * 136657;
    a[8] -= a[15] * 683901;

    a[2] += a[14] * 666643;
    a[3] += a[14] * 470296;
    a[4] += a[14] * 654183;
    a[5] -= a[14] * 997805;
    a[6] += a[14] * 136657;
    a[7] -= a[14] * 683901;

    a[1] += a[13] * 666643;
    a[2] += a[13] * 470296;
    a[3] += a[13] * 654183;
    a[4] -= a[13] * 997805;
    a[5] += a[13] * 136657;
    a[6] -= a[13] * 683901;

    a[0] += a[12] * 666643;
    a[1] += a[12] * 470296;
    a[2] += a[12] * 654183;
    a[3] -= a[12] * 997805;
    a[4] += a[12] * 136657;
    a[5] -= a[12] * 683901;
}
