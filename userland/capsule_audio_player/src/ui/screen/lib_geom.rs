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

//! Library-table geometry. The painter and every hit-test read these, so a
//! click resolves to the row that was drawn under it at the current scroll.

extern crate alloc;

use alloc::vec::Vec;

use crate::library::{Library, Queue};
use crate::ui::geometry::Rect;
use crate::ui::metrics::{line_h, BODY, PAGE, S3, S4};
use crate::ui::state::LIB_TABS;
use crate::ui::widget::{tab_at, ROW_H};

const TAB_H: i32 = 36;

pub fn tabs_rect(r: Rect) -> Rect {
    Rect::new(r.x, r.y + line_h(PAGE) + line_h(BODY) + S3 + S4, r.w, TAB_H)
}

pub fn head_rect(r: Rect) -> Rect {
    Rect::new(r.x, tabs_rect(r).bottom() + S4, r.w, 28)
}

pub fn list_rect(r: Rect) -> Rect {
    let top = head_rect(r).bottom() + S3;
    Rect::new(r.x, top, r.w, (r.bottom() - top).max(0))
}

pub fn visible(r: Rect) -> usize {
    (list_rect(r).h / ROW_H).max(0) as usize
}

pub fn row_at(r: Rect, scroll: usize, len: usize, x: i32, y: i32) -> Option<usize> {
    let l = list_rect(r);
    let i = (y - l.y) / ROW_H;
    if i < 0 || !l.contains(x, y) {
        return None;
    }
    let idx = scroll + i as usize;
    if idx < len { Some(idx) } else { None }
}

pub fn tab_hit(r: Rect, x: i32, y: i32) -> Option<usize> {
    tab_at(tabs_rect(r), &LIB_TABS, x, y)
}

pub fn rows_for(lib: &Library, queue: &Queue, tab: usize) -> Vec<usize> {
    match tab {
        2 => queue.items().to_vec(),
        _ => (0..lib.tracks.len()).collect(),
    }
}

