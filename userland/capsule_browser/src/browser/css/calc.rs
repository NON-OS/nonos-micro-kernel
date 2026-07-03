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

use super::calc_term::term;

// A calc() value: numbers scale, lengths add. A length is a pixel part plus
// a per-mille-of-base part, so "100% - 80px" survives to layout intact.
#[derive(Clone, Copy)]
pub(super) enum V {
    Num(f32),
    Len { px: f32, pml: f32 },
}

pub(super) struct P<'a> {
    pub(super) s: &'a [u8],
    pub(super) i: usize,
    pub(super) em: u32,
}

// Evaluate the body of calc(...) to (px, permille). None on any form the
// grammar does not cover; the declaration then drops, never guesses.
pub(super) fn eval_calc(inner: &str, em: u32) -> Option<(f32, f32)> {
    let mut p = P { s: inner.as_bytes(), i: 0, em };
    let v = expr(&mut p)?;
    p.skip_ws();
    if p.i != p.s.len() {
        return None;
    }
    match v {
        V::Len { px, pml } => Some((px, pml)),
        // A bare number is not a length.
        V::Num(_) => None,
    }
}

pub(super) fn expr(p: &mut P) -> Option<V> {
    let mut acc = term(p)?;
    loop {
        p.skip_ws();
        let op = match p.s.get(p.i) {
            Some(b'+') => b'+',
            Some(b'-') => b'-',
            _ => return Some(acc),
        };
        p.i += 1;
        let rhs = term(p)?;
        acc = match (acc, rhs) {
            (V::Len { px: a, pml: b }, V::Len { px: c, pml: d }) => {
                let s = if op == b'+' { 1.0 } else { -1.0 };
                V::Len { px: a + s * c, pml: b + s * d }
            }
            (V::Num(a), V::Num(c)) => V::Num(if op == b'+' { a + c } else { a - c }),
            _ => return None,
        };
    }
}
