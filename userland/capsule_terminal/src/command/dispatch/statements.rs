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

use alloc::vec::Vec;

// How a statement is gated on the previous statement's exit status:
// `;` runs always, `&&` runs only after success, `||` only after failure.
// A single `|` is left untouched here so the pipeline stage can split it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Conn {
    Always,
    And,
    Or,
}

pub struct Stmt<'a> {
    pub conn: Conn,
    pub body: &'a [u8],
    pub background: bool,
}

pub fn split_program(line: &[u8]) -> Vec<Stmt<'_>> {
    let mut out: Vec<Stmt<'_>> = Vec::new();
    let (mut start, mut conn) = (0usize, Conn::Always);
    let (mut sq, mut dq) = (false, false);
    let mut i = 0;
    while i < line.len() {
        let c = line[i];
        if c == b'\'' && !dq {
            sq = !sq;
        } else if c == b'"' && !sq {
            dq = !dq;
        } else if !sq && !dq && c == b';' {
            push(&mut out, conn, &line[start..i], false);
            conn = Conn::Always;
            start = i + 1;
        } else if !sq && !dq && c == b'&' && line.get(i + 1) == Some(&b'&') {
            push(&mut out, conn, &line[start..i], false);
            conn = Conn::And;
            start = i + 2;
            i += 2;
            continue;
        } else if !sq && !dq && c == b'|' && line.get(i + 1) == Some(&b'|') {
            push(&mut out, conn, &line[start..i], false);
            conn = Conn::Or;
            start = i + 2;
            i += 2;
            continue;
        } else if !sq && !dq && c == b'&' {
            push(&mut out, conn, &line[start..i], true);
            conn = Conn::Always;
            start = i + 1;
        }
        i += 1;
    }
    push(&mut out, conn, &line[start..], false);
    out
}

fn push<'a>(out: &mut Vec<Stmt<'a>>, conn: Conn, seg: &'a [u8], background: bool) {
    let mut a = 0;
    let mut b = seg.len();
    while a < b && seg[a] == b' ' {
        a += 1;
    }
    while b > a && seg[b - 1] == b' ' {
        b -= 1;
    }
    if a < b {
        out.push(Stmt { conn, body: &seg[a..b], background });
    }
}
