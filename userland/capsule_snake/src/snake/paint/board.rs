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

use super::layout::Layout;
use crate::snake::state::Game;

const BOARD_BG: u32 = 0xFF18_2024;
const BODY: u32 = 0xFF3F_A34D;
const HEAD: u32 = 0xFF6A_D47A;
const FOOD: u32 = 0xFFE0_533D;

pub fn paint(game: &Game, layout: &Layout, fb: &mut PaintBuffer) {
    fb.fill_rect(layout.x, layout.y, layout.w, layout.h, BOARD_BG);
    cell(fb, layout, game.food, FOOD);
    for segment in game.body.iter().skip(1) {
        cell(fb, layout, *segment, BODY);
    }
    cell(fb, layout, game.body[0], HEAD);
}

fn cell(fb: &mut PaintBuffer, layout: &Layout, at: (i16, i16), argb: u32) {
    let inset = layout.inset();
    let px = layout.x + at.0 as u32 * layout.cell + inset;
    let py = layout.y + at.1 as u32 * layout.cell + inset;
    fb.fill_rect(px, py, layout.cell - 2 * inset, layout.cell - 2 * inset, argb);
}
