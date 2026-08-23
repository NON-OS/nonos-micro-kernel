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

use super::super::state::Sort;
use super::metrics::{COL_AUTH_W, COL_CPU_W, COL_MEM_W, COL_PID_W, COL_STATE_W, COL_UPTIME_W};

// Everything measured from these columns (widths, offsets, the row band and the
// click inversions) is a child module, so each file stays inside the 75-line cap
// while the painter and the hit test keep a single import path through here.
#[path = "table_span.rs"]
mod table_span;

pub use table_span::{col_w, col_x, index_at, max_scroll, name_w, row_y, sort_at_x, visible_rows};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Col {
    Name,
    Pid,
    State,
    Cpu,
    Mem,
    Uptime,
    Auth,
}

pub const COLS_OVERVIEW: [Col; 5] = [Col::Name, Col::Pid, Col::Cpu, Col::Mem, Col::Auth];
pub const COLS_FULL: [Col; 7] =
    [Col::Name, Col::Pid, Col::State, Col::Cpu, Col::Mem, Col::Uptime, Col::Auth];

// Name is the one flex column, so it declares no budget of its own and takes
// whatever the fixed columns leave.
pub fn fixed_w(col: Col) -> u32 {
    match col {
        Col::Name => 0,
        Col::Pid => COL_PID_W,
        Col::State => COL_STATE_W,
        Col::Cpu => COL_CPU_W,
        Col::Mem => COL_MEM_W,
        Col::Uptime => COL_UPTIME_W,
        Col::Auth => COL_AUTH_W,
    }
}

// Only four of the seven columns have a sort in refresh(); clicking any other
// header is a no-op rather than a silently wrong reorder.
pub fn sort_for(col: Col) -> Option<Sort> {
    match col {
        Col::Name => Some(Sort::Name),
        Col::Pid => Some(Sort::Pid),
        Col::Cpu => Some(Sort::Cpu),
        Col::Mem => Some(Sort::Mem),
        _ => None,
    }
}
