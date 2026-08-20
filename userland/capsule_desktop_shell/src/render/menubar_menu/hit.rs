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

//! Which title, and which row of the open drop-down, sits under a point.

use super::items::{rows, TITLE_COUNT};
use super::metrics::{pad_y, panel_w, row_h, title_w, title_x};
use super::origin::origin;
use crate::render::layout::menubar_height;
use crate::render::topbar::brand_right;
use crate::state::Context;

pub fn title_hit(ctx: &Context, px: u32, py: u32) -> Option<usize> {
    if py >= menubar_height() || px < brand_right() {
        return None;
    }
    (0..TITLE_COUNT).find(|&i| {
        let x = title_x(ctx, i);
        px >= x && px < x + title_w(ctx, i)
    })
}

pub fn row_hit(ctx: &Context, px: u32, py: u32) -> Option<usize> {
    let index = ctx.menubar.open?;
    let (ox, oy) = origin(ctx, index);
    if px < ox || px >= ox + panel_w(ctx, index) {
        return None;
    }
    (0..rows(ctx, index).len()).find(|&i| {
        let top = oy + pad_y() + i as u32 * row_h();
        py >= top && py < top + row_h()
    })
}
