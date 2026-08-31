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
use nonos_toolkit::paint::mixer::lerp_argb;

use crate::snake::state::Game;
use crate::snake::theme::{SNAKE_HEAD, SNAKE_TAIL, TITLE};
use crate::snake::ui::play_geom::Board;

use super::board_cell::{cell, centre, radius};
use super::glow;

// Tail first, so the head lands on top of the segment behind it and its halo
// is never overdrawn by the body.
pub fn paint(game: &Game, fb: &mut PaintBuffer, b: &Board) {
    let len = game.body.len().max(1) as u32;
    let r = radius(b);
    for (index, seg) in game.body.iter().enumerate().rev() {
        let s = cell(b, *seg);
        let t = (index as u32 * 255) / len;
        fb.fill_round(s.0, s.1, s.2, s.3, r, lerp_argb(SNAKE_HEAD, SNAKE_TAIL, t));
    }
    if let Some(at) = game.body.first() {
        head(fb, b, *at, r);
    }
}

fn head(fb: &mut PaintBuffer, b: &Board, at: (i16, i16), r: u32) {
    let s = cell(b, at);
    fb.fill_round(s.0, s.1, s.2, s.3, r, SNAKE_HEAD);
    glow::bloom(fb, s, r);
    let (cx, cy) = centre(b, at);
    fb.circle(cx, cy, (s.2 / 5).max(1), TITLE);
}
