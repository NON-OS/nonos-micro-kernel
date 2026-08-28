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

use crate::snake::grid::{COLS, ROWS};
use crate::snake::ui::play_geom::Board;
use crate::snake::ui::rect::Rect;

// The Game Over still is the run as it ended rather than an illustration, so it
// needs a board inside a panel rather than inside the play stage. Nothing hit
// tests against it; the play board still comes from `play_geom::board`.
pub fn fit(r: Rect) -> Board {
    let cell = (r.2 / COLS as u32).min(r.3 / ROWS as u32).max(2);
    let w = cell * COLS as u32;
    let h = cell * ROWS as u32;
    let x = r.0 + r.2.saturating_sub(w) / 2;
    let y = r.1 + r.3.saturating_sub(h) / 2;
    Board { cell, x, y, w, h }
}
