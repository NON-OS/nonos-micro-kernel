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

use crate::calc::fixed::Fixed;
use crate::calc::format::format;
use crate::calc::op::Op;

fn symbol(op: Op) -> u8 {
    match op {
        Op::Add => b'+',
        Op::Sub => b'-',
        Op::Mul => b'x',
        Op::Div => b'/',
        Op::Pow => b'^',
        Op::None => b'=',
    }
}

fn put(out: &mut [u8], at: usize, byte: u8) -> usize {
    if at < out.len() {
        out[at] = byte;
        1
    } else {
        0
    }
}

pub fn write(lhs: Fixed, op: Op, rhs: Fixed, out: &mut [u8]) -> usize {
    let mut n = format(lhs, 0, out);
    n += put(out, n, b' ');
    n += put(out, n, symbol(op));
    n += put(out, n, b' ');
    if n < out.len() {
        n += format(rhs, 0, &mut out[n..]);
    }
    n
}
