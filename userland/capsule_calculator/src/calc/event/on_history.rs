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

use crate::calc::mode::Mode;
use crate::calc::state::State;
use crate::calc::ui::history_geom;

pub fn click(state: &mut State, x: i32, y: i32) -> bool {
    let (w, h) = state.view;
    let index = match history_geom::at(w, h, x, y) {
        Some(found) => found,
        None => return false,
    };
    let value = match state.history.get(index) {
        Some(entry) => entry.value,
        None => return false,
    };
    state.set_mode(Mode::Basic);
    state.display = value;
    true
}
