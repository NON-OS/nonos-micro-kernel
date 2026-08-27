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

use nonos_app_skeleton::PaintBuffer;

use super::button;
use crate::calc::buttons::grid;
use crate::calc::hit::Hit;
use crate::calc::mode::Mode;
use crate::calc::prog::allowed;
use crate::calc::state::State;
use crate::calc::ui::keypad_geom::cell;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width as i32, fb.height as i32);
    for (row, buttons) in grid(state.mode).iter().enumerate() {
        let mut col = 0usize;
        for (idx, btn) in buttons.iter().enumerate() {
            let span = btn.span.max(1);
            let rect = cell(state.mode, w, h, row, col, span);
            let enabled = state.mode != Mode::Programmer || allowed(state.base, btn.action);
            let hover = enabled && state.hover == Some(Hit::Key(row, idx));
            button::paint(fb, btn, rect, hover, enabled);
            col += span as usize;
        }
    }
}
