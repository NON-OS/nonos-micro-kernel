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

use super::entries::Entry;
use super::file_color::color;
use super::file_kind::kind_of;
use super::icon;
use super::layout::ROW_H;
use super::theme::PANEL;

// The tile the filetype glyph sits on. It is its own opaque ground, so the
// glyph -- which hollows itself by overpainting with `bg` -- renders against a
// known colour whether or not the row underneath is lit.
const TILE: u32 = 30;
const TILE_X: u32 = 8;
const GLYPH: u32 = 18;

/// Draws the rounded filetype tile at the head of a row and returns the x the
/// text column starts at, so no caller repeats the icon arithmetic.
pub fn row_tile(fb: &mut PaintBuffer, entry: &Entry, y: u32, left: u32) -> u32 {
    let tile_y = y + (ROW_H - TILE) / 2;
    fb.fill_round(left + TILE_X, tile_y, TILE, TILE, 8, PANEL);
    let tint = color(kind_of(entry));
    let gx = left + TILE_X + (TILE - GLYPH) / 2;
    let gy = tile_y + (TILE - GLYPH) / 2;
    if entry.is_dir {
        icon::folder(fb, gx, gy, GLYPH, tint, PANEL);
    } else {
        icon::file(fb, gx, gy, GLYPH, tint, PANEL);
    }
    left + TILE_X + TILE + 12
}
