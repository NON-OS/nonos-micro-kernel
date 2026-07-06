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

use nonos_app_skeleton::EventOutcome;

use super::clipboard;
use super::clipboard_paste;
use super::duplicate;
use super::perms;
use super::selection;
use super::selection_select_all::select_all;
use super::state::State;
use super::view::rebuild_view;

pub fn run_action(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        code if code == b' ' as u32 => selection::toggle(state),
        code if code == b'a' as u32 => select_all(state),
        code if code == b'c' as u32 => clipboard::yank(state, false),
        code if code == b'x' as u32 => clipboard::yank(state, true),
        code if code == b'p' as u32 => clipboard_paste::paste(state),
        code if code == b'o' as u32 => duplicate::duplicate(state),
        code if code == b'u' as u32 => perms::toggle_readonly(state),
        code if code == b's' as u32 => {
            state.sort_mode = state.sort_mode.next();
            rebuild_view(state);
        }
        _ => return None,
    }
    Some(EventOutcome::Repaint)
}
