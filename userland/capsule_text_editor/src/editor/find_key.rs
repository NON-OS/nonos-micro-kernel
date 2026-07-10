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

//! Key handling while the find bar is open: type to search, Enter for the next
//! match, Backspace to edit the query, Escape to close. With replace mode on,
//! typing edits the replacement instead, Enter rewrites the current match, and
//! Tab flips between the query and replacement fields.

use super::state::State;

const KEY_BACKSPACE: u32 = 0x08;
const KEY_TAB: u32 = 0x09;
const KEY_ENTER: u32 = 0x0D;
const KEY_ESC: u32 = 0x1B;
const MAX_QUERY: usize = 128;

pub(super) fn find_key(state: &mut State, code: u32) {
    if state.replace_active {
        match code {
            KEY_ESC => {
                state.replace_active = false;
                state.find_active = false;
            }
            KEY_TAB => state.replace_active = false,
            KEY_ENTER => {
                state.replace_current();
            }
            KEY_BACKSPACE => {
                state.replace_buf.pop();
            }
            c => push_char(&mut state.replace_buf, c),
        }
        return;
    }
    match code {
        KEY_ESC => state.find_active = false,
        KEY_TAB => state.replace_active = true,
        KEY_ENTER => state.find_next(true),
        KEY_BACKSPACE => {
            state.find_buf.pop();
            state.find_incremental();
        }
        c => {
            push_char(&mut state.find_buf, c);
            state.find_incremental();
        }
    }
}

fn push_char(buf: &mut alloc::string::String, code: u32) {
    if (0x20..=0x0010_FFFF).contains(&code) && buf.len() < MAX_QUERY {
        if let Some(ch) = char::from_u32(code) {
            buf.push(ch);
        }
    }
}
