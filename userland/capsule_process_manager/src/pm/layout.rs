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

//! Column geometry for the process table, shared by the painter that draws the
//! header and the event handler that turns a header click into a sort. One
//! definition so a click always lands on the column it looks like it hits.

use super::state::Sort;

// Height of the top bar, and the first row / row pitch of the security panel's
// findings list. Shared so a click resolves to the row it visually hits.
pub const HEADER_H: u32 = 42;
pub const SEC_LIST_TOP: u32 = 96;
pub const SEC_ROW_H: u32 = 23;

pub const COL_NAME: u32 = 18;
pub const COL_PID: u32 = 240;
pub const COL_STATE: u32 = 320;
pub const COL_CPU: u32 = 430;
pub const COL_MEM: u32 = 560;
pub const COL_UPTIME: u32 = 690;
pub const COL_CAPS: u32 = 810;

// The sortable column a header click at `x` falls in, if any. State, uptime and
// caps have no meaningful sort order here, so they yield None.
pub fn sort_at_x(x: i32) -> Option<Sort> {
    let x = x.max(0) as u32;
    if x >= COL_UPTIME {
        None
    } else if x >= COL_MEM {
        Some(Sort::Mem)
    } else if x >= COL_CPU {
        Some(Sort::Cpu)
    } else if x >= COL_STATE {
        None
    } else if x >= COL_PID {
        Some(Sort::Pid)
    } else {
        Some(Sort::Name)
    }
}
