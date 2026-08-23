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

use crate::pm::state::{Screen, State};

use super::super::chrome::Rect;
use super::super::matrix_geom;
use super::super::screens::sec_geom;
use super::super::table_geom::{self, Col};
use super::Target;

// How many rows of the active list fit the pane. Navigation clamps against this,
// so it has to come from whichever geometry module actually drew the rows.
pub fn rows_visible(screen: Screen, pane_h: u32) -> usize {
    match screen {
        Screen::Authority => matrix_geom::visible_rows(pane_h),
        _ => table_geom::visible_rows(pane_h),
    }
}

// The header band sorts, the body selects. Both tests read the same table-local
// origin, which is why a click just under the header cannot resolve as a sort.
pub fn table(state: &State, r: &Rect, cols: &[Col], x: i32, y: i32) -> Option<Target> {
    if table_geom::in_head(y) {
        return table_geom::sort_at_x(cols, r.w, x).map(Target::Sort);
    }
    let total = state.filtered().len();
    table_geom::index_at(y, state.scroll, total, r.h).map(Target::Row)
}

// A click left of the first capability column lands on the process name, which
// selects the row without naming a capability. Both branches take the pane height
// so the legend strip under the grid can never resolve as a row.
pub fn matrix(state: &State, r: &Rect, x: i32, y: i32) -> Option<Target> {
    let total = state.filtered().len();
    match matrix_geom::cell_at(r.w, x, y, r.h, state.scroll, total) {
        Some((row, _)) => Some(Target::Matrix(row)),
        None => matrix_geom::row_at(y, r.h, state.scroll, total).map(Target::Row),
    }
}

pub fn finding(state: &State, r: &Rect, y: i32) -> Option<Target> {
    sec_geom::row_at(r.h, y, state.alert_scroll, state.alerts.len()).map(Target::Finding)
}
