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

//! Print the last lines of a file (default ten, -n or -<count> to change).

use alloc::vec::Vec;

use super::read_file::slurp;
use crate::command::flags::{parse, parse_usize, Spec};
use crate::command::output::Output;
use crate::term::state::State;

pub fn tail(state: &mut State, argv: &[&[u8]]) {
    let spec = Spec::new(b"tail", b"").valued(b"n").numeric(b'n');
    let parsed = match parse(&spec, &argv[1..]) {
        Ok(p) => p,
        Err(e) => return Output::new(&mut state.scrollback).writeln(&e),
    };
    let n = parsed.value(b'n').and_then(parse_usize).unwrap_or(10);
    let Some(file) = parsed.operands.last().copied() else {
        Output::new(&mut state.scrollback).writeln(b"tail: missing file");
        return;
    };
    let Some(bytes) = slurp(state, file) else { return };
    let lines: Vec<&[u8]> = bytes.split(|&b| b == b'\n').collect();
    let start = lines.len().saturating_sub(n);
    let mut out = Output::new(&mut state.scrollback);
    for line in &lines[start..] {
        out.writeln(line);
    }
}
