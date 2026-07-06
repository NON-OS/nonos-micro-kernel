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

use super::refresh::refresh;
use super::state::State;

pub fn open_parent(state: &mut State) -> EventOutcome {
    if state.prefix == "/" {
        return EventOutcome::Idle;
    }
    let trim = state.prefix.trim_end_matches('/');
    let cut = trim.rfind('/').unwrap_or(0);
    state.prefix.truncate(if cut == 0 { 1 } else { cut + 1 });
    state.cursor = 0;
    state.scroll = 0;
    refresh(state);
    EventOutcome::Repaint
}
