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

use crate::browser::paint::chrome::constants;
use crate::browser::state::State;

pub fn pill(state: &State, fb: &mut PaintBuffer) {
    let l = constants::PILL_L as u32;
    let r = fb.width.saturating_sub(52).max(l);
    let w = r.saturating_sub(l);
    let edge = if state.address_focused { constants::ACCENT } else { constants::BORDER };
    fb.fill_rect(l, constants::TITLEBAR + 10, w, 32, edge);
    fb.fill_rect(l + 1, constants::TITLEBAR + 11, w.saturating_sub(2), 30, constants::FIELD_BG);
    fb.fill_rect(l, constants::TITLEBAR + 10, 2, 2, constants::TOOLBAR_BG);
    fb.fill_rect(l + w.saturating_sub(2), constants::TITLEBAR + 10, 2, 2, constants::TOOLBAR_BG);
    fb.fill_rect(l, constants::TITLEBAR + 40, 2, 2, constants::TOOLBAR_BG);
    fb.fill_rect(l + w.saturating_sub(2), constants::TITLEBAR + 40, 2, 2, constants::TOOLBAR_BG);
    fb.fill_rect(l + 10, constants::TITLEBAR + 22, 8, 8, constants::DIM);
    fb.fill_rect(l + 12, constants::TITLEBAR + 24, 4, 4, constants::FIELD_BG);
    let ty = (constants::TITLEBAR + 15) as i32;
    if state.address.is_empty() {
        fb.text_ttf(l as i32 + 30, ty, "Search or enter address", constants::DIM, 15.0);
    } else {
        fb.text_ttf(l as i32 + 30, ty, &state.address, constants::FG, 15.0);
    }
}
