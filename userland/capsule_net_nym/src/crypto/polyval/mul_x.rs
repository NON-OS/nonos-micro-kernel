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

/// Multiply by the indeterminate, reducing modulo
/// x^128 + x^127 + x^126 + x^121 + 1.
///
/// When the shift pushes a term past x^127 it comes back as all four lower
/// terms of the polynomial, so the fold touches both words: the `+1` lands in
/// word 0 and x^121, x^126, x^127 in word 1.
pub(super) fn mul_x(v: &mut [u64; 2]) {
    let overflow = v[1] >> 63;
    v[1] = (v[1] << 1) | (v[0] >> 63);
    v[0] <<= 1;
    if overflow == 1 {
        v[0] ^= 1;
        v[1] ^= 0xc200_0000_0000_0000;
    }
}
