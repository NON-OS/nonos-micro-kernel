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

//! The bar fill and the hairline that separates it from the desktop.

use super::metrics::{BAR_BG, BAR_BORDER};
use crate::render::fill::fill_rect;
use crate::render::layout::menubar_rect;
use crate::state::Context;

pub(super) fn background(ctx: &Context) {
    let bar = menubar_rect(ctx.width);
    let (va, st, w, h) = (ctx.backing_va, ctx.stride, ctx.width, ctx.height);
    fill_rect(va, st, w, h, bar.x, bar.y, bar.width, bar.height, BAR_BG);
    let edge = crate::render::ui_font::scale();
    let mut fb = crate::render::surface::surface(ctx);
    fb.blend_rect(bar.x, bar.y + bar.height - edge, bar.width, edge, BAR_BORDER);
}
