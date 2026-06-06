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

use crate::render::layout::Rect;
use crate::state::Context;

pub fn paint_cursor(ctx: &Context) {
    if !ctx.pointer_visible {
        return;
    }
    let x = ctx.pointer_x.min(ctx.width.saturating_sub(super::constants::CURSOR_SIZE));
    let y = ctx.pointer_y.min(ctx.height.saturating_sub(super::constants::CURSOR_SIZE));
    super::paint_rect::paint_rect(
        ctx,
        Rect { x: x + 1, y, width: 2, height: super::constants::CURSOR_SIZE },
        super::constants::CURSOR_SHADOW_ARGB,
    );
    super::paint_rect::paint_rect(
        ctx,
        Rect { x, y: y + 1, width: super::constants::CURSOR_SIZE, height: 2 },
        super::constants::CURSOR_SHADOW_ARGB,
    );
    super::paint_rect::paint_rect(
        ctx,
        Rect { x, y, width: 2, height: super::constants::CURSOR_SIZE },
        super::constants::CURSOR_ARGB,
    );
    super::paint_rect::paint_rect(
        ctx,
        Rect { x, y, width: super::constants::CURSOR_SIZE, height: 2 },
        super::constants::CURSOR_ARGB,
    );
}
