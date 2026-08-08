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

use super::mul_x::mul_x;
use super::mul_x_inv::mul_x_inv;

/// POLYVAL's field operation: a * b * x^-128.
///
/// The trailing x^-128 is what separates POLYVAL from GHASH-style
/// multiplication. Omitting it yields a tag that is self-consistent and
/// authenticates nothing an interoperating peer would accept.
pub(super) fn dot(a: &mut [u64; 2], b: &[u64; 2]) {
    let mut z = [0u64; 2];
    let mut v = *b;
    for word in 0..2 {
        for bit in 0..64 {
            if (a[word] >> bit) & 1 == 1 {
                z[0] ^= v[0];
                z[1] ^= v[1];
            }
            mul_x(&mut v);
        }
    }
    for _ in 0..128 {
        mul_x_inv(&mut z);
    }
    *a = z;
}
