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

use crate::browser::paint::chrome::{constants, pill};
use crate::browser::state::State;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(0, constants::TITLEBAR, fb.width, 52, constants::TOOLBAR_BG);
    chevron(fb, constants::BACK_X as u32);
    chevron(fb, constants::FWD_X as u32);
    reload(fb, constants::RELOAD_X as u32);
    home(fb, constants::HOME_X as u32);
    pill::pill(state, fb);
    hamburger(fb);
    fb.fill_rect(0, constants::TITLEBAR + 51, fb.width, 1, constants::ACCENT);
}

fn chevron(fb: &mut PaintBuffer, x: u32) {
    fb.fill_rect(x + 4, constants::TITLEBAR + 22, 2, 8, constants::DIM);
    fb.fill_rect(x + 6, constants::TITLEBAR + 20, 2, 2, constants::DIM);
    fb.fill_rect(x + 6, constants::TITLEBAR + 30, 2, 2, constants::DIM);
    fb.fill_rect(x + 8, constants::TITLEBAR + 18, 2, 2, constants::DIM);
    fb.fill_rect(x + 8, constants::TITLEBAR + 32, 2, 2, constants::DIM);
}

fn reload(fb: &mut PaintBuffer, x: u32) {
    fb.fill_rect(x + 2, constants::TITLEBAR + 18, 12, 2, constants::FG);
    fb.fill_rect(x + 2, constants::TITLEBAR + 32, 12, 2, constants::FG);
    fb.fill_rect(x + 2, constants::TITLEBAR + 18, 2, 8, constants::FG);
    fb.fill_rect(x + 12, constants::TITLEBAR + 26, 2, 8, constants::FG);
    fb.fill_rect(x + 10, constants::TITLEBAR + 16, 2, 2, constants::FG);
    fb.fill_rect(x + 12, constants::TITLEBAR + 16, 2, 4, constants::FG);
}

fn home(fb: &mut PaintBuffer, x: u32) {
    fb.fill_rect(x + 6, constants::TITLEBAR + 16, 4, 2, constants::FG);
    fb.fill_rect(x + 4, constants::TITLEBAR + 18, 8, 2, constants::FG);
    fb.fill_rect(x + 2, constants::TITLEBAR + 20, 12, 2, constants::FG);
    fb.fill_rect(x + 4, constants::TITLEBAR + 22, 8, 12, constants::FG);
    fb.fill_rect(x + 7, constants::TITLEBAR + 28, 2, 6, constants::TOOLBAR_BG);
}

fn hamburger(fb: &mut PaintBuffer) {
    let x = fb.width.saturating_sub(36);
    fb.fill_rect(x, constants::TITLEBAR + 18, 16, 2, constants::DIM);
    fb.fill_rect(x, constants::TITLEBAR + 25, 16, 2, constants::DIM);
    fb.fill_rect(x, constants::TITLEBAR + 32, 16, 2, constants::DIM);
}
