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

//! Index of the desktop item under a point, if any.

use super::cell_rect::cell_rect;
use crate::state::Context;

pub fn hit(ctx: &Context, px: u32, py: u32) -> Option<usize> {
    (0..ctx.desktop_items.len()).find(|&i| {
        let (x, y, w, h) = cell_rect(ctx, i);
        px >= x && px < x + w && py >= y && py < y + h
    })
}
