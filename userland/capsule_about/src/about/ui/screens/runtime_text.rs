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

use crate::about::data::runtime::Runtime;
use crate::about::format::{kib, load_q11};

use super::tile_text::push;

// Used and total in one cell, so the pair is read as a fraction rather than as two
// unrelated numbers a reader has to divide themselves.
pub fn memory<'a>(r: &Runtime, dst: &'a mut [u8; 64]) -> &'a [u8] {
    let mut used = [0u8; 24];
    let mut total = [0u8; 24];
    let mut n = push(dst, 0, kib(r.mem_used_kb, &mut used));
    n = push(dst, n, b" / ");
    n = push(dst, n, kib(r.mem_total_kb, &mut total));
    &dst[..n]
}

pub fn loads<'a>(r: &Runtime, dst: &'a mut [u8; 48]) -> &'a [u8] {
    let mut n = 0usize;
    for (i, q) in r.load.iter().enumerate() {
        let mut one = [0u8; 12];
        if i > 0 {
            n = push(dst, n, b"  ");
        }
        n = push(dst, n, load_q11(*q, &mut one));
    }
    &dst[..n]
}
