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

use crate::calc::actions::dispatch;
use crate::calc::buttons::GRID;
use crate::calc::layout::hit_test;
use crate::calc::state::State;
use crate::calc::ui::metrics::RAIL_W;
use crate::calc::ui::nav_geom;

pub fn on_pointer_button(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    if x < RAIL_W {
        if let Some(mode) = nav_geom::at(x, y) {
            state.set_mode(mode);
            return EventOutcome::Repaint;
        }
        return EventOutcome::Idle;
    }
    let (row, col) = match hit_test(x as u32, y as u32) {
        Some(rc) => rc,
        None => return EventOutcome::Idle,
    };
    if row as usize >= GRID.len() || col as usize >= GRID[0].len() {
        return EventOutcome::Idle;
    }
    let button = GRID[row as usize][col as usize];
    dispatch::run(state, button.action);
    EventOutcome::Repaint
}
