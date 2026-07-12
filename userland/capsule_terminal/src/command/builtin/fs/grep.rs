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

//! Print the lines of a file that contain a substring.

use super::read_file::slurp;
use crate::command::output::Output;
use crate::term::state::State;

pub fn grep(state: &mut State, argv: &[&[u8]]) {
    if argv.len() < 3 {
        Output::new(&mut state.scrollback).writeln(b"usage: grep <pattern> <file>");
        return;
    }
    let pat = argv[1];
    let Some(bytes) = slurp(state, argv[2]) else { return };
    let mut out = Output::new(&mut state.scrollback);
    for line in bytes.split(|&b| b == b'\n') {
        if contains(line, pat) {
            out.writeln(line);
        }
    }
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || needle.len() > hay.len() {
        return needle.is_empty();
    }
    hay.windows(needle.len()).any(|w| w == needle)
}
