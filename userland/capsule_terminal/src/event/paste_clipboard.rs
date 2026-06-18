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

use nonos_app_skeleton::{clipboard_paste, EventOutcome};

use crate::term::dimensions::COLS;
use crate::term::state::State;

pub fn paste_clipboard(state: &mut State) -> EventOutcome {
    let mut buf = [0u8; COLS];
    let n = match clipboard_paste(&mut buf) {
        Ok(n) => n.min(buf.len()),
        Err(_) => return EventOutcome::Idle,
    };
    let mut changed = false;
    for &b in &buf[..n] {
        if (0x20..=0x7E).contains(&b) {
            if !state.line.insert(b) {
                break;
            }
            changed = true;
        }
    }
    if !changed {
        return EventOutcome::Idle;
    }
    state.history.reset_cursor();
    state.scrollback.jump_bottom();
    EventOutcome::Repaint
}
