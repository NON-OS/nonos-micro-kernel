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

use crate::term::state::State;

// `alias` lists definitions; `alias NAME EXPANSION...` defines one. The
// first word of a future line matching NAME is replaced by EXPANSION.
pub fn run(state: &mut State, args: &[&[u8]]) {
    if args.is_empty() {
        if state.aliases.is_empty() {
            return state.scrollback.push_line(b"(no aliases)");
        }
        for (k, v) in &state.aliases {
            let mut row = Vec::with_capacity(k.len() + v.len() + 3);
            row.extend_from_slice(k);
            row.extend_from_slice(b" = ");
            row.extend_from_slice(v);
            state.scrollback.push_line(&row);
        }
        return;
    }
    if args.len() < 2 {
        return state.scrollback.push_line(b"usage: alias <name> <expansion>");
    }
    let name = args[0];
    let mut exp = Vec::new();
    for (i, word) in args[1..].iter().enumerate() {
        if i > 0 {
            exp.push(b' ');
        }
        exp.extend_from_slice(word);
    }
    if let Some(slot) = state.aliases.iter_mut().find(|(k, _)| k.as_slice() == name) {
        slot.1 = exp;
    } else {
        state.aliases.push((name.to_vec(), exp));
    }
}
