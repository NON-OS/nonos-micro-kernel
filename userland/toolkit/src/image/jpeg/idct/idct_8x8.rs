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
use super::clamp_u8::clamp_u8;
use super::col::idct_1d_col;
use super::row::idct_1d_row;

pub fn idct_8x8(coeffs: &[i32; 64], out: &mut [u8; 64]) {
    let mut scratch: [i32; 64] = [0; 64];
    let mut spatial: [i32; 64] = [0; 64];
    let mut r = 0usize;
    while r < 8 {
        idct_1d_row(coeffs, r, &mut scratch);
        r += 1;
    }
    let mut c = 0usize;
    while c < 8 {
        idct_1d_col(&scratch, c, &mut spatial);
        c += 1;
    }
    let mut i = 0usize;
    while i < 64 {
        out[i] = clamp_u8(spatial[i] + 128);
        i += 1;
    }
}
