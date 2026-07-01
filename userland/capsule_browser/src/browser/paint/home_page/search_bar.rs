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

use crate::browser::paint::home_page::constants;
use crate::browser::state::State;

pub fn search_bar(state: &State, fb: &mut PaintBuffer) {
    let edge = if state.address_focused { constants::ACCENT } else { constants::BORDER };
    let x = fb.width.saturating_sub(constants::PILL_W) / 2;
    fb.fill_rect(x, constants::PILL_Y, constants::PILL_W, constants::PILL_H, edge);
    fb.fill_rect(x + 2, constants::PILL_Y + 2, constants::PILL_W - 4, constants::PILL_H - 4, constants::PILL_BG);
    globe(fb, x + 16, constants::PILL_Y + 15);
    let ty = constants::PILL_Y + (constants::PILL_H - 8) / 2;
    let tx = x + 40;
    if state.address.is_empty() {
        fb.text(tx, ty, b"Search or enter a URL", constants::DIM);
    } else {
        fb.text(tx, ty, state.address.as_bytes(), constants::FG);
    }
}

fn globe(fb: &mut PaintBuffer, x: u32, y: u32) {
    fb.fill_rect(x, y, 16, 16, constants::DIM);
    fb.fill_rect(x, y, 2, 2, constants::PILL_BG);
    fb.fill_rect(x + 14, y, 2, 2, constants::PILL_BG);
    fb.fill_rect(x, y + 14, 2, 2, constants::PILL_BG);
    fb.fill_rect(x + 14, y + 14, 2, 2, constants::PILL_BG);
    fb.fill_rect(x + 7, y, 2, 16, constants::PILL_BG);
    fb.fill_rect(x, y + 7, 16, 2, constants::PILL_BG);
}
