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

use crate::term::state::State;

pub fn basename(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&path) = args.first() else {
        state.scrollback.push_error(b"usage: basename <path>");
        return false;
    };
    let trimmed = path.strip_suffix(b"/").unwrap_or(path);
    match trimmed.iter().rposition(|&c| c == b'/') {
        Some(i) if i + 1 < trimmed.len() => state.scrollback.push_line(&trimmed[i + 1..]),
        None if !trimmed.is_empty() => state.scrollback.push_line(trimmed),
        _ => state.scrollback.push_line(b"/"),
    }
    true
}

pub fn dirname(state: &mut State, args: &[&[u8]]) -> bool {
    let Some(&path) = args.first() else {
        state.scrollback.push_error(b"usage: dirname <path>");
        return false;
    };
    let trimmed = path.strip_suffix(b"/").unwrap_or(path);
    match trimmed.iter().rposition(|&c| c == b'/') {
        Some(0) => state.scrollback.push_line(b"/"),
        Some(i) => state.scrollback.push_line(&trimmed[..i]),
        None => state.scrollback.push_line(b"."),
    }
    true
}
