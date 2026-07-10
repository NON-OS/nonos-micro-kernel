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

//! Print the last lines of a file (default ten).

use alloc::vec::Vec;

use super::read_file::slurp;
use crate::command::output::Output;
use crate::term::state::State;

pub fn tail(state: &mut State, argv: &[&[u8]]) {
    let (n, file) = super::head_tail_args(argv, 10);
    let Some(file) = file else {
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
