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

use crate::term::util::format_u64;

// Expand `$NAME` shell-variable references and `$?` (last exit status) in
// a command line before it is tokenized. Expansion is suppressed inside
// single quotes (POSIX behavior); quote bytes are preserved so the
// tokenizer still strips them. An undefined variable expands to nothing.
pub fn expand(line: &[u8], vars: &[(Vec<u8>, Vec<u8>)], last_status: i32) -> Vec<u8> {
    let mut out = Vec::with_capacity(line.len());
    let mut in_single = false;
    let mut i = 0;
    while i < line.len() {
        let c = line[i];
        if c == b'\'' {
            in_single = !in_single;
            out.push(c);
            i += 1;
            continue;
        }
        if !in_single && c == b'$' && i + 1 < line.len() && line[i + 1] == b'?' {
            push_status(&mut out, last_status);
            i += 2;
            continue;
        }
        let starts_name =
            i + 1 < line.len() && (line[i + 1].is_ascii_alphabetic() || line[i + 1] == b'_');
        if !in_single && c == b'$' && starts_name {
            let start = i + 1;
            let mut j = start;
            while j < line.len() && (line[j].is_ascii_alphanumeric() || line[j] == b'_') {
                j += 1;
            }
            if let Some((_, v)) = vars.iter().find(|(k, _)| k.as_slice() == &line[start..j]) {
                out.extend_from_slice(v);
            }
            i = j;
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}

fn push_status(out: &mut Vec<u8>, status: i32) {
    if status < 0 {
        out.push(b'-');
    }
    let mut buf = [0u8; 20];
    let n = format_u64(status.unsigned_abs() as u64, &mut buf);
    out.extend_from_slice(&buf[..n]);
}
