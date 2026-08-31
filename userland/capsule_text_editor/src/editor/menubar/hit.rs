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

//! Hit-testing against the same measured geometry the painter used, so a click
//! always lands on the cell that was drawn under the pointer.

use super::items::rows;
use super::metrics::{panel_rect, row_h, TitleSpan};
use crate::editor::layout::TITLEBAR_H;

pub(in crate::editor) enum MenuHit {
    Title(usize),
    Row(usize),
    Outside,
}

pub(in crate::editor) fn menubar_hit(
    spans: &[TitleSpan],
    open: Option<usize>,
    x: i32,
    y: i32,
) -> MenuHit {
    if x < 0 || y < 0 {
        return MenuHit::Outside;
    }
    let (x, y) = (x as u32, y as u32);
    if let Some(o) = open {
        let (px, py, pw, ph) = panel_rect(spans, o);
        if x >= px && x < px + pw && y >= py + 1 && y < py + ph - 1 {
            let idx = ((y - py - 1) / row_h()) as usize;
            return match rows(o).get(idx) {
                Some(_) => MenuHit::Row(idx),
                None => MenuHit::Outside,
            };
        }
    }
    if y < TITLEBAR_H {
        for (i, span) in spans.iter().enumerate() {
            if x >= span.x0 && x < span.x1 {
                return MenuHit::Title(i);
            }
        }
    }
    MenuHit::Outside
}
