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

use crate::calc::hit::Hit;
use crate::calc::layout::hit_test;
use crate::calc::state::State;
use crate::calc::ui::metrics::RAIL_W;
use crate::calc::ui::nav_geom;

fn hover_at(state: &State, x: i32, y: i32) -> Option<Hit> {
    if x < 0 || y < 0 {
        return None;
    }
    if x < RAIL_W {
        return nav_geom::at(x, y).map(Hit::Rail);
    }
    let (w, h) = state.view;
    hit_test(state.mode, w, h, x, y).map(|(row, col)| Hit::Key(row, col))
}

pub fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    let hover = hover_at(state, x, y);
    if hover == state.hover {
        return EventOutcome::Idle;
    }
    state.hover = hover;
    EventOutcome::Repaint
}
