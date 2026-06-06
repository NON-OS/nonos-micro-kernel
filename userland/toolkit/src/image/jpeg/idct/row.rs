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
use super::table::COS_TABLE;

pub fn idct_1d_row(coeffs: &[i32; 64], row: usize, scratch: &mut [i32; 64]) {
    let base = row * 8;
    let mut x = 0usize;
    while x < 8 {
        let mut acc: i64 = 0;
        let mut u = 0usize;
        while u < 8 {
            acc += (coeffs[base + u] as i64) * (COS_TABLE[u][x] as i64);
            u += 1;
        }
        scratch[base + x] = ((acc + 8192) >> 14) as i32;
        x += 1;
    }
}
