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

//! Which menu row, if any, sits under a point.

use super::metrics::{items, pad_y, row_h, width};
use super::origin::origin;
use crate::state::Context;

pub fn hit(ctx: &Context, px: u32, py: u32) -> Option<usize> {
    ctx.desktop_menu?;
    let (ox, oy) = origin(ctx);
    if px < ox || px >= ox + width(ctx) {
        return None;
    }
    for i in 0..items(ctx).len() {
        let top = oy + pad_y() + i as u32 * row_h();
        if py >= top && py < top + row_h() {
            return Some(i);
        }
    }
    None
}
