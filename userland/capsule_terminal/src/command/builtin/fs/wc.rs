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

//! Count lines, words and bytes in a file.

use super::read_file::slurp;
use crate::command::output::Output;
use crate::term::state::State;
use crate::term::util::format_u64;

pub fn wc(state: &mut State, argv: &[&[u8]]) {
    if argv.len() < 2 {
        Output::new(&mut state.scrollback).writeln(b"wc: missing file");
        return;
    }
    let Some(bytes) = slurp(state, argv[1]) else { return };
    let lines = bytes.iter().filter(|&&b| b == b'\n').count() as u64;
    let words = bytes.split(|b| b.is_ascii_whitespace()).filter(|w| !w.is_empty()).count() as u64;
    let mut line = alloc::vec::Vec::new();
    for (label, v) in
        [(b"lines " as &[u8], lines), (b"  words ", words), (b"  bytes ", bytes.len() as u64)]
    {
        line.extend_from_slice(label);
        let mut num = [0u8; 20];
        let n = format_u64(v, &mut num);
        line.extend_from_slice(&num[..n]);
    }
    Output::new(&mut state.scrollback).writeln(&line);
}
