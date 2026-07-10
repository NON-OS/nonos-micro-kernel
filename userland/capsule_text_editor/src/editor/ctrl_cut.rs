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

//! Cut: copy the selection to the clipboard, then delete it.

use nonos_app_skeleton::{clipboard_copy, EventOutcome};

use super::state::State;

pub(super) fn ctrl_cut(state: &mut State) -> EventOutcome {
    if let Some((s, e)) = state.sel_range() {
        if clipboard_copy(&state.buf[s..e]).is_ok() {
            state.delete_sel();
            state.status = b"cut";
        } else {
            state.status = b"clipboard unavailable";
        }
    }
    EventOutcome::Repaint
}
