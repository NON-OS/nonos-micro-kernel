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

//! Press routing for the six table-driven sections. Only `Ctl::Toggle` rows are
//! hit tested, because the dropdowns are drawn dimmed and have nothing to open;
//! rects come from `control_box` with the latched width, as the painter does.

use nonos_app_skeleton::EventOutcome;

use super::card::{control_box, TOGGLE_H, TOGGLE_W};
use super::sect::{Ctl, Section};
use super::sect_state::flip_sect;
use super::state::width;
use crate::editor::widget::toggle_hit;

pub(super) fn section_press(nav: usize, sec: &Section, mx: i32, my: i32) -> EventOutcome {
    let w = width();
    if w == 0 {
        return EventOutcome::Idle;
    }
    for (row, spec) in sec.rows.iter().enumerate() {
        if let Ctl::Toggle(bit) = spec.1 {
            if toggle_hit(control_box(w, row, TOGGLE_W, TOGGLE_H), mx, my) {
                flip_sect(nav, bit);
                return EventOutcome::Repaint;
            }
        }
    }
    EventOutcome::Idle
}
