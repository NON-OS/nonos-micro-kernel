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

use super::metrics::TBL_HEAD_H;

use super::super::state::Sort;

pub use super::table_cols::{fixed_w, Col, COLS_FULL, COLS_OVERVIEW};

// Everything measured from these columns (widths, offsets, the row band and the
// click inversions) is a child module, so each file stays inside the 75-line cap
// while the painter and the hit test keep a single import path through here.
#[path = "table_span.rs"]
mod table_span;

pub use table_span::{col_w, col_x, index_at, max_scroll, name_w, row_y, sort_at_x, visible_rows};

// The header band, in table-local coordinates. `index_at` rejects the same y,
// so the sort test and the row test share one edge rather than two.
pub fn in_head(y: i32) -> bool {
    y >= 0 && y < TBL_HEAD_H as i32
}

// Only the columns refresh() can order by sort on a click; any other header
// is a no-op rather than a silently wrong reorder.
pub fn sort_for(col: Col) -> Option<Sort> {
    match col {
        Col::Name => Some(Sort::Name),
        Col::Pid => Some(Sort::Pid),
        Col::Cpu => Some(Sort::Cpu),
        Col::Mem => Some(Sort::Mem),
        Col::Ipc => Some(Sort::Ipc),
        Col::Sysc => Some(Sort::Sysc),
        _ => None,
    }
}
