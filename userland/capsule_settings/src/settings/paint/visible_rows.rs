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

use super::layout::{BODY_TOP, ROW_H, STATUS_H};

/// Rows that fit a window `win_h` tall. This was fixed at the manifest height,
/// so making the window taller left the extra space empty.
pub fn visible_rows(win_h: u32) -> usize {
    let body_height = win_h.saturating_sub(BODY_TOP + STATUS_H);
    (body_height / ROW_H) as usize
}
