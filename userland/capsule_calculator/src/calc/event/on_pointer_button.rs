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

use super::on_convert;
use super::on_history;
use crate::calc::actions::{dispatch, prog_bit};
use crate::calc::buttons::grid;
use crate::calc::layout::hit_test;
use crate::calc::mode::Mode;
use crate::calc::state::State;
use crate::calc::ui::bits_geom;
use crate::calc::ui::metrics::RAIL_W;
use crate::calc::ui::nav_geom;

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}

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
    let (w, h) = state.view;
    if state.mode == Mode::Convert {
        return outcome(on_convert::click(state, x, y));
    }
    if state.mode == Mode::History {
        return outcome(on_history::click(state, x, y));
    }
    if state.mode == Mode::Programmer {
        if let Some(bit) = bits_geom::at(w, x, y) {
            prog_bit::run(state, bit);
            return EventOutcome::Repaint;
        }
    }
    let (row, col) = match hit_test(state.mode, w, h, x, y) {
        Some(rc) => rc,
        None => return EventOutcome::Idle,
    };
    let button = match grid(state.mode).get(row).and_then(|r| r.get(col)) {
        Some(btn) => *btn,
        None => return EventOutcome::Idle,
    };
    dispatch::run(state, button.action);
    EventOutcome::Repaint
}
