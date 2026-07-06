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

use crate::clock::manifest::WIDTH;
use crate::clock::state::State;
use crate::clock::tabs;

pub fn on_click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if let Some(t) = tabs::hit(WIDTH as i32, x, y) {
        state.tab = t;
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}
