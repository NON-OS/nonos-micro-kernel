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

use super::background;
use super::bits;
use super::convert;
use super::display;
use super::grid;
use super::history;
use super::radix;
use super::rail;
use crate::calc::mode::Mode;
use crate::calc::state::State;

pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    state.view = (fb.width as i32, fb.height as i32);
    background::paint(fb);
    display::paint(state, fb);
    match state.mode {
        Mode::Programmer => {
            bits::paint(state, fb);
            radix::paint(state, fb);
        }
        Mode::Convert => convert::paint(state, fb),
        Mode::History => history::paint(state, fb),
        _ => {}
    }
    grid::paint(state, fb);
    rail::paint(state, fb);
}
