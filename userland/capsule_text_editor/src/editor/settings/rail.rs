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

//! The left navigation rail: the screen title over a label list. Every section
//! now has a panel behind it, so the list is painted live in one pass.

use nonos_app_skeleton::PaintBuffer;

use super::geom::{nav_rect, title_top, NAV_LABELS, NAV_PX, RAIL_PAD, RAIL_W, RAIL_X, TITLE_PX};
use super::style::{nav_live, HAIRLINE, RAIL_BG, TEXT};
use crate::editor::widget::paint_navlist;

pub(super) fn paint_rail(fb: &mut PaintBuffer, selected: usize) {
    let h = fb.height;
    fb.fill_rect(RAIL_X, 0, RAIL_W, h, RAIL_BG);
    fb.fill_rect(RAIL_X + RAIL_W - 1, 0, 1, h, HAIRLINE);
    let tx = (RAIL_X + RAIL_PAD + 12) as i32;
    let _ = fb.text_ttf(tx, title_top() as i32, "Docs Settings", TEXT, TITLE_PX);

    let (nx, ny, nw) = nav_rect();
    paint_navlist(fb, (nx, ny, nw), &NAV_LABELS, selected, NAV_PX, &nav_live());
}
