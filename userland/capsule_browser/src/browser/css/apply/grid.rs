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

use crate::browser::css::auto_repeat::parse_auto_repeat;
use crate::browser::css::computed::{Computed, MAX_GRID_COLS};
use crate::browser::css::grid_tracks::parse_grid_tracks;
use crate::browser::css::parse_px::parse_px;

const MAX_GAP_PX: u32 = 128;

// Grid column tracks and the gap shared by grid and flex containers.
pub(super) fn apply_grid(c: &mut Computed, name: &str, value: &str, fs: u32) -> bool {
    match name {
        "grid-template-columns" => {
            // auto-fill and auto-fit carry no track count, so the value has to
            // be recognised before the fixed-list parser rejects it and leaves
            // the container on its one-column default.
            if let Some((mode, min, track)) = parse_auto_repeat(value, fs) {
                c.grid_auto = Some(mode);
                c.grid_auto_min = min;
                c.grid_cols = [track; MAX_GRID_COLS];
                c.grid_col_n = 1;
            } else if let Some((cols, ncols)) = parse_grid_tracks(value, fs) {
                c.grid_auto = None;
                c.grid_cols = cols;
                c.grid_col_n = ncols;
            }
        }
        "gap" | "column-gap" | "row-gap" | "grid-column-gap" | "grid-row-gap" | "grid-gap" => {
            // The gap shorthand may give row and column; one gap serves both.
            if let Some(first) = value.split_whitespace().next() {
                if let Some(px) = parse_px(first, fs) {
                    c.gap = px.min(MAX_GAP_PX);
                }
            }
        }
        _ => return false,
    }
    true
}
