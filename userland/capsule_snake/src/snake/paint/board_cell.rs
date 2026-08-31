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

use crate::snake::ui::metrics::RADIUS_CELL;
use crate::snake::ui::play_geom::Board;
use crate::snake::ui::rect::Rect;

// Every piece on the board is placed through here, so the snake, the walls and
// the food cannot disagree about where a cell sits.
pub fn cell(b: &Board, at: (i16, i16)) -> Rect {
    let pad = b.inset();
    let x = b.x + at.0.max(0) as u32 * b.cell + pad;
    let y = b.y + at.1.max(0) as u32 * b.cell + pad;
    let span = b.cell.saturating_sub(pad * 2).max(1);
    (x, y, span, span)
}

pub fn centre(b: &Board, at: (i16, i16)) -> (u32, u32) {
    let r = cell(b, at);
    (r.0 + r.2 / 2, r.1 + r.3 / 2)
}

pub fn radius(b: &Board) -> u32 {
    RADIUS_CELL.min(b.cell / 3).max(1)
}
