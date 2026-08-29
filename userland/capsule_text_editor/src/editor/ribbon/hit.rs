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

//! Hit-testing against the cells the painter recorded, so a click always lands
//! on the control that was drawn under the pointer.

use super::items::{pill_labels, RibbonItem};
use super::metrics::{band_top, row_h, RibbonCell};
use super::panel::panel_rect;
use crate::editor::layout::RIBBON_H;

pub(in crate::editor) enum RibbonHit {
    Cell(RibbonItem),
    Row(usize),
    Outside,
}

pub(in crate::editor) fn ribbon_hit(
    cells: &[RibbonCell],
    open: Option<usize>,
    x: i32,
    y: i32,
) -> RibbonHit {
    if x < 0 || y < 0 {
        return RibbonHit::Outside;
    }
    let (x, y) = (x as u32, y as u32);
    if let Some(p) = open {
        let (px, py, pw, ph) = panel_rect(cells, p);
        if x >= px && x < px + pw && y >= py + 1 && y < py + ph - 1 {
            let idx = ((y - py - 1) / row_h()) as usize;
            return match idx < pill_labels(p).len() {
                true => RibbonHit::Row(idx),
                false => RibbonHit::Outside,
            };
        }
    }
    if y >= band_top() && y < band_top() + RIBBON_H {
        for c in cells.iter() {
            if x >= c.x0 && x < c.x1 {
                return RibbonHit::Cell(c.item);
            }
        }
    }
    RibbonHit::Outside
}
