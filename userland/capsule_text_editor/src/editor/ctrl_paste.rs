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

use super::follow_caret::follow_caret;
use super::state::State;

pub(super) fn ctrl_paste(state: &mut State) -> EventOutcome {
    let mut scratch = [0u8; 512];
    match clipboard_paste(&mut scratch) {
        Ok(n)
            if core::str::from_utf8(&scratch[..n]).is_ok() && {
                // Paste replaces the selection when there is one.
                state.delete_sel();
                state.insert(&scratch[..n])
            } =>
        {
            // Named /notes.txt whatever document was actually open.
            state.status = b"pasted";
            let rows = state.visible_rows;
            follow_caret(state, rows);
            EventOutcome::Repaint
        }
        Ok(_) => {
            state.status = b"paste rejected";
            EventOutcome::Repaint
        }
        Err(_) => {
            state.status = b"clipboard unavailable";
            EventOutcome::Repaint
        }
    }
}
