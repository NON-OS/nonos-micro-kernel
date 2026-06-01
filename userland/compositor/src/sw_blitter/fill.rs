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

use super::Surface;
use crate::state::damage::Rect;

pub fn fill_rect(surface: Surface, rect: Rect, argb: u32) {
    if rect.width == 0 || rect.height == 0 {
        return;
    }
    let x1 = rect.x.saturating_add(rect.width).min(surface.width);
    let y1 = rect.y.saturating_add(rect.height).min(surface.height);
    if rect.x >= x1 || rect.y >= y1 {
        return;
    }
    let row_width = (x1 - rect.x) as usize;
    for y in rect.y..y1 {
        let Some(row_va) = surface.row_start(y, rect.x, x1 - rect.x) else {
            break;
        };
        let row_ptr = row_va as *mut u32;
        unsafe { core::slice::from_raw_parts_mut(row_ptr, row_width).fill(argb) };
    }
}
