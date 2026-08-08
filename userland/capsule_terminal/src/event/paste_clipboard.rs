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
    // Stop at the first newline rather than skipping over it. Dropping
    // newlines glued separate commands into one line, which the next Enter
    // then ran as a single mangled command.
    let first_line = match buf[..n].iter().position(|&b| b == b'\n' || b == b'\r') {
        Some(cut) => &buf[..cut],
        None => &buf[..n],
    };
    let multiline = first_line.len() < n;

    let mut changed = false;
    let mut full = false;
    for &b in first_line {
        if (0x20..=0x7E).contains(&b) {
            if !state.line.insert(b) {
                full = true;
                break;
            }
            changed = true;
        }
    }
    // The input line holds COLS bytes, so anything longer cannot fit. Say so
    // instead of leaving a silently shortened command on the prompt.
    if multiline {
        state.scrollback.push_line(b"paste: first line only");
    } else if full {
        state.scrollback.push_line(b"paste: line full, clipboard truncated");
    }
    if !changed {
        return EventOutcome::Repaint;
    }
    state.history.reset_cursor();
    state.scrollback.jump_bottom();
    EventOutcome::Repaint
}
