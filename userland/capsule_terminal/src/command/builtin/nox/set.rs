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

// `set` with no args lists variables; `set NAME VALUE...` defines or
// updates one. Values may contain spaces (join the remaining args) or
// be quoted. Expanded later as $NAME.
pub fn run(state: &mut State, args: &[&[u8]]) {
    if args.is_empty() {
        if state.vars.is_empty() {
            return state.scrollback.push_line(b"(no variables)");
        }
        for (k, v) in &state.vars {
            let mut row = Vec::with_capacity(k.len() + v.len() + 1);
            row.extend_from_slice(k);
            row.push(b'=');
            row.extend_from_slice(v);
            state.scrollback.push_line(&row);
        }
        return;
    }
    if args.len() < 2 {
        return state.scrollback.push_error(b"usage: set <name> <value>");
    }
    let name = args[0];
    let mut value = Vec::new();
    for (i, word) in args[1..].iter().enumerate() {
        if i > 0 {
            value.push(b' ');
        }
        value.extend_from_slice(word);
    }
    if let Some(slot) = state.vars.iter_mut().find(|(k, _)| k.as_slice() == name) {
        slot.1 = value;
    } else {
        state.vars.push((name.to_vec(), value));
    }
}
