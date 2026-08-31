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

use super::super::metrics::{CELL_PAD_X, NAME_MIN_W, ROW_H, TBL_HEAD_H};
use super::{fixed_w, Col};

// The name column absorbs whatever the fixed columns leave, down to a floor that
// keeps a truncated name readable rather than a lone ellipsis.
pub fn name_w(cols: &[Col], table_w: u32) -> u32 {
    let fixed: u32 = cols.iter().map(|c| fixed_w(*c)).sum();
    table_w.saturating_sub(fixed + CELL_PAD_X * 2).max(NAME_MIN_W)
}

pub fn col_w(cols: &[Col], table_w: u32, col: Col) -> u32 {
    match col {
        Col::Name => name_w(cols, table_w),
        _ => fixed_w(col),
    }
}

// Columns tile left to right from the cell pad; the painter and the hit test walk
// this same sequence, so a click cannot land on a column other than the drawn one.
pub fn col_x(cols: &[Col], table_w: u32, col: Col) -> u32 {
    let mut x = CELL_PAD_X;
    for c in cols.iter().take_while(|c| **c != col) {
        x += col_w(cols, table_w, *c);
    }
    x
}

pub fn row_y(index: usize) -> u32 {
    TBL_HEAD_H + index as u32 * ROW_H
}

pub fn visible_rows(table_h: u32) -> usize {
    (table_h.saturating_sub(TBL_HEAD_H) / ROW_H).max(1) as usize
}

pub fn max_scroll(total: usize, table_h: u32) -> usize {
    total.saturating_sub(visible_rows(table_h))
}

// Which visible row a click at table-local `y` lands on. The header band and
// anything past the last drawn row return None, so a click on empty table space
// selects nothing rather than the nearest row.
pub fn index_at(y: i32, scroll: usize, total: usize, table_h: u32) -> Option<usize> {
    if y < TBL_HEAD_H as i32 {
        return None;
    }
    let slot = ((y - TBL_HEAD_H as i32) as u32 / ROW_H) as usize;
    let index = scroll + slot;
    (slot < visible_rows(table_h) && index < total).then_some(index)
}

pub fn sort_at_x(cols: &[Col], table_w: u32, x: i32) -> Option<Col> {
    let x = x.max(0) as u32;
    cols.iter().copied().find(|c| {
        let cx = col_x(cols, table_w, *c);
        x >= cx && x < cx + col_w(cols, table_w, *c)
    })
}
