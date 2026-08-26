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

use super::layout::{Rect, EDGE, ROW_H, TOP};

pub const HEADER_H: u32 = 44;

pub fn list_top() -> u32 {
    TOP + HEADER_H
}

pub fn visible_rows(h: u32) -> usize {
    (h.saturating_sub(list_top()).saturating_sub(EDGE) / ROW_H) as usize
}

pub fn row_rect(w: u32, slot: usize) -> Rect {
    Rect {
        x: EDGE,
        y: list_top() + slot as u32 * ROW_H,
        w: w.saturating_sub(EDGE * 2),
        h: ROW_H,
    }
}

pub fn row_at(w: u32, h: u32, scroll: usize, count: usize, x: i32, y: i32) -> Option<usize> {
    if x < EDGE as i32 || x >= w.saturating_sub(EDGE) as i32 {
        return None;
    }
    if y < list_top() as i32 {
        return None;
    }
    let slot = ((y as u32 - list_top()) / ROW_H) as usize;
    if slot >= visible_rows(h) {
        return None;
    }
    let index = scroll.checked_add(slot)?;
    if index < count {
        Some(index)
    } else {
        None
    }
}

pub fn scroll_for(sel: usize, scroll: usize, h: u32) -> usize {
    let vis = visible_rows(h).max(1);
    if sel < scroll {
        sel
    } else if sel >= scroll + vis {
        sel + 1 - vis
    } else {
        scroll
    }
}
