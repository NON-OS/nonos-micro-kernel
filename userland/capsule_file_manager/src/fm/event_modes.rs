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

use super::state::{Mode, State};

pub fn open_mode(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        code if code == b'/' as u32 => {
            state.mode = Mode::Filter;
            state.status = b"filter: ";
        }
        code if code == b'?' as u32 => {
            state.mode = Mode::Help;
        }
        _ => return None,
    }
    Some(EventOutcome::Repaint)
}
