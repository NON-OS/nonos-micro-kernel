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

//! The `#!` line.

use alloc::vec::Vec;

/// Linux's BINPRM_BUF_SIZE. A line longer than this is truncated at the
/// limit rather than refused, which is also what Linux does.
const MAX_LINE: usize = 127;

pub struct Interp {
    pub path: Vec<u8>,
    pub arg: Option<Vec<u8>>,
}

pub fn parse(image: &[u8]) -> Option<Interp> {
    let head = image.get(..MAX_LINE.min(image.len()))?;
    let body = head.strip_prefix(b"#!")?;
    let line = &body[..body.iter().position(|b| *b == b'\n').unwrap_or(body.len())];
    let line = trim(line);
    let cut = line.iter().position(|b| *b == b' ' || *b == b'\t').unwrap_or(line.len());
    let path = &line[..cut];
    if path.is_empty() {
        return None;
    }
    let rest = trim(&line[cut..]);
    Some(Interp {
        path: path.to_vec(),
        arg: (!rest.is_empty()).then(|| rest.to_vec()),
    })
}

fn trim(s: &[u8]) -> &[u8] {
    let start = s.iter().position(|b| *b != b' ' && *b != b'\t').unwrap_or(s.len());
    let end = s.iter().rposition(|b| *b != b' ' && *b != b'\t' && *b != b'\r');
    match end {
        Some(e) => &s[start..=e.max(start)],
        None => &s[..0],
    }
}
