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

use super::types::{Argv, MAX_ARGS};
use crate::term::util::is_space;

pub fn parse(input: &[u8]) -> Argv<'_> {
    let mut out = Argv { argv: [b""; MAX_ARGS], argc: 0 };
    let mut i = 0;
    while i < input.len() && out.argc < MAX_ARGS {
        while i < input.len() && is_space(input[i]) {
            i += 1;
        }
        if i >= input.len() {
            break;
        }
        // A token wrapped in matching single or double quotes is taken
        // verbatim between the quotes, so arguments may contain spaces
        // (`write notes.txt "hello world"`). The slice still points into
        // the input; only the quote bytes are excluded. An unterminated
        // quote runs to end of line.
        if input[i] == b'"' || input[i] == b'\'' {
            let quote = input[i];
            let start = i + 1;
            let mut j = start;
            while j < input.len() && input[j] != quote {
                j += 1;
            }
            out.argv[out.argc] = &input[start..j];
            i = if j < input.len() { j + 1 } else { j };
        } else {
            let start = i;
            while i < input.len() && !is_space(input[i]) && input[i] != b'"' && input[i] != b'\'' {
                i += 1;
            }
            out.argv[out.argc] = &input[start..i];
        }
        out.argc += 1;
    }
    out
}
