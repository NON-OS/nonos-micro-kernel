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

use crate::snake::state::Game;
use crate::snake::theme::{FOOD, FOOD_RING, POWER, POWER_RING, WALL, WALL_EDGE};
use crate::snake::ui::play_geom::Board;

use super::board_cell::{cell, centre, radius};
use super::diamond;

pub fn paint(game: &Game, fb: &mut PaintBuffer, b: &Board) {
    let r = radius(b);
    for wall in game.walls.iter() {
        let block = cell(b, *wall);
        fb.fill_round(block.0, block.1, block.2, block.3, r, WALL);
        fb.stroke_round(block.0, block.1, block.2, block.3, r, 1, WALL_EDGE);
    }
    food(fb, b, game.food);
    if let Some(at) = game.power {
        power(fb, b, at);
    }
}

fn food(fb: &mut PaintBuffer, b: &Board, at: (i16, i16)) {
    let (cx, cy) = centre(b, at);
    let span = cell(b, at).2 / 2;
    let thick = (span / 3).max(1);
    fb.ring(cx, cy, span, thick, FOOD_RING);
    fb.circle(cx, cy, span.saturating_sub(thick + 1).max(1), FOOD);
}

fn power(fb: &mut PaintBuffer, b: &Board, at: (i16, i16)) {
    let (cx, cy) = centre(b, at);
    let span = cell(b, at).2 / 2;
    diamond::ring(fb, cx, cy, span, POWER_RING);
    diamond::fill(fb, cx, cy, span.saturating_sub(2), POWER);
}
