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

//! Count lines, words and bytes in a file; -l, -w and -c select columns.

use super::read_file::slurp;
use crate::command::flags::{parse, Spec};
use crate::command::output::Output;
use crate::term::state::State;
use crate::term::util::format_u64;

pub fn wc(state: &mut State, argv: &[&[u8]]) {
    let parsed = match parse(&Spec::new(b"wc", b"lwc"), &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    let Some(file) = parsed.operands.first().copied() else {
        Output::new(&mut state.scrollback).writeln(b"wc: missing file");
        return;
    };
    let picked = parsed.has(b'l') || parsed.has(b'w') || parsed.has(b'c');
    let Some(bytes) = slurp(state, file) else { return };
    let lines = bytes.iter().filter(|&&b| b == b'\n').count() as u64;
    let words = bytes.split(|b| b.is_ascii_whitespace()).filter(|w| !w.is_empty()).count() as u64;
    let mut line = alloc::vec::Vec::new();
    for (want, label, v) in [
        (parsed.has(b'l'), b"lines " as &[u8], lines),
        (parsed.has(b'w'), b"words ", words),
        (parsed.has(b'c'), b"bytes ", bytes.len() as u64),
    ] {
        if picked && !want {
            continue;
        }
        if !line.is_empty() {
            line.extend_from_slice(b"  ");
        }
        line.extend_from_slice(label);
        let mut num = [0u8; 20];
        let n = format_u64(v, &mut num);
        line.extend_from_slice(&num[..n]);
    }
    Output::new(&mut state.scrollback).writeln(&line);
}
