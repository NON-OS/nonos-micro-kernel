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

use crate::snake::grid::{BOARD_H, BOARD_W, BOARD_X, BOARD_Y, CELL};
use crate::snake::state::Game;

const BOARD_BG: u32 = 0xFF18_2024;
const BODY: u32 = 0xFF3F_A34D;
const HEAD: u32 = 0xFF6A_D47A;
const FOOD: u32 = 0xFFE0_533D;

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    fb.fill_rect(BOARD_X, BOARD_Y, BOARD_W, BOARD_H, BOARD_BG);
    cell(fb, game.food, FOOD);
    for segment in game.body.iter().skip(1) {
        cell(fb, *segment, BODY);
    }
    cell(fb, game.body[0], HEAD);
}

fn cell(fb: &mut PaintBuffer, at: (i16, i16), argb: u32) {
    let px = BOARD_X + at.0 as u32 * CELL;
    let py = BOARD_Y + at.1 as u32 * CELL;
    fb.fill_rect(px + 1, py + 1, CELL - 2, CELL - 2, argb);
}
