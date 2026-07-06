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

use crate::browser::manifest::{HEIGHT, WIDTH};

use super::calc::{expr, P, V};

impl P<'_> {
    pub(super) fn skip_ws(&mut self) {
        while self.s.get(self.i).is_some_and(|b| b.is_ascii_whitespace()) {
            self.i += 1;
        }
    }
}

// One operand: a parenthesised subexpression, a nested calc(), or a number
// with an optional unit. Percent becomes per-mille of the eventual base.
pub(super) fn factor(p: &mut P) -> Option<V> {
    p.skip_ws();
    if p.s[p.i..].starts_with(b"(") {
        p.i += 1;
        let v = expr(p)?;
        p.skip_ws();
        if p.s.get(p.i) == Some(&b')') {
            p.i += 1;
            return Some(v);
        }
        return None;
    }
    let lower: [u8; 5] =
        core::array::from_fn(|k| p.s.get(p.i + k).copied().unwrap_or(0).to_ascii_lowercase());
    if lower.starts_with(b"calc(") {
        p.i += 5;
        let v = expr(p)?;
        p.skip_ws();
        if p.s.get(p.i) == Some(&b')') {
            p.i += 1;
            return Some(v);
        }
        return None;
    }
    let start = p.i;
    if p.s.get(p.i).is_some_and(|&b| b == b'+' || b == b'-') {
        p.i += 1;
    }
    let mut dot = false;
    while p.s.get(p.i).is_some_and(|&b| b.is_ascii_digit() || (b == b'.' && !dot)) {
        dot |= p.s[p.i] == b'.';
        p.i += 1;
    }
    let n: f32 = core::str::from_utf8(&p.s[start..p.i]).ok()?.parse().ok()?;
    let ustart = p.i;
    while p.s.get(p.i).is_some_and(|b| b.is_ascii_alphabetic() || *b == b'%') {
        p.i += 1;
    }
    match &p.s[ustart..p.i].to_ascii_lowercase()[..] {
        b"" => Some(V::Num(n)),
        b"%" => Some(V::Len { px: 0.0, pml: n * 10.0 }),
        b"px" => Some(V::Len { px: n, pml: 0.0 }),
        b"rem" => Some(V::Len { px: n * 16.0, pml: 0.0 }),
        b"em" => Some(V::Len { px: n * p.em as f32, pml: 0.0 }),
        b"vw" => Some(V::Len { px: n * WIDTH as f32 / 100.0, pml: 0.0 }),
        b"vh" => Some(V::Len { px: n * HEIGHT as f32 / 100.0, pml: 0.0 }),
        _ => None,
    }
}
