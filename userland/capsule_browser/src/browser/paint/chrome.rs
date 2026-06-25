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

use crate::browser::manifest::WIDTH;
use crate::browser::state::State;

const BG: u32 = 0xFF10_1418;
const BAR: u32 = 0xFF1E_2630;
const FG: u32 = 0xFFE6_EDF3;
const ACCENT: u32 = 0xFF4C_9AFF;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(BG);
    fb.fill_rect(0, 0, WIDTH, 40, BAR);
    let caret = if state.address_focused { ACCENT } else { BAR };
    fb.fill_rect(8, 8, WIDTH - 16, 24, caret);
    fb.fill_rect(10, 10, WIDTH - 20, 20, 0xFF0C_0F12);
    fb.text(16, 14, state.address.as_bytes(), FG);
    fb.text(16, HEIGHT_STATUS_Y, state.status.as_bytes(), 0xFF8B_98A5);
}

const HEIGHT_STATUS_Y: u32 = 676;
