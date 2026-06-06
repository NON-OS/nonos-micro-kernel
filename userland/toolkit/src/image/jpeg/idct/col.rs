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

pub fn idct_1d_col(scratch: &[i32; 64], col: usize, out: &mut [i32; 64]) {
    let mut y = 0usize;
    while y < 8 {
        let mut acc: i64 = 0;
        let mut v = 0usize;
        while v < 8 {
            acc += (scratch[v * 8 + col] as i64) * (COS_TABLE[v][y] as i64);
            v += 1;
        }
        out[y * 8 + col] = ((acc + (1 << 15)) >> 16) as i32;
        y += 1;
    }
}
