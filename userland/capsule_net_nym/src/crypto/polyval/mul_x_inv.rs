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

/// Divide by the indeterminate.
///
/// From the same polynomial, x^-1 = x^127 + x^126 + x^125 + x^120, so an odd
/// element folds that constant in as it shifts down. POLYVAL needs this
/// because its product carries an x^-128 factor that plain multiplication
/// does not remove.
pub(super) fn mul_x_inv(v: &mut [u64; 2]) {
    let odd = v[0] & 1;
    v[0] = (v[0] >> 1) | (v[1] << 63);
    v[1] >>= 1;
    if odd == 1 {
        v[1] ^= 0xe100_0000_0000_0000;
    }
}
