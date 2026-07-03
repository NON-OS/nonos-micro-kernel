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

use super::calc::{P, V};
use super::calc_factor::factor;

// One term: factors joined by * and /. A number scales a length; length
// times length has no meaning and drops the declaration.
pub(super) fn term(p: &mut P) -> Option<V> {
    let mut acc = factor(p)?;
    loop {
        p.skip_ws();
        let op = match p.s.get(p.i) {
            Some(b'*') => b'*',
            Some(b'/') => b'/',
            _ => return Some(acc),
        };
        p.i += 1;
        let rhs = factor(p)?;
        acc = mul_div(acc, rhs, op)?;
    }
}

fn mul_div(a: V, b: V, op: u8) -> Option<V> {
    match (a, b, op) {
        (V::Num(x), V::Num(y), b'*') => Some(V::Num(x * y)),
        (V::Num(x), V::Num(y), b'/') if y != 0.0 => Some(V::Num(x / y)),
        (V::Len { px, pml }, V::Num(n), b'*') | (V::Num(n), V::Len { px, pml }, b'*') => {
            Some(V::Len { px: px * n, pml: pml * n })
        }
        (V::Len { px, pml }, V::Num(n), b'/') if n != 0.0 => {
            Some(V::Len { px: px / n, pml: pml / n })
        }
        _ => None,
    }
}
