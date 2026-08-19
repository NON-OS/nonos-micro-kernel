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

//! Top-left corner of an open drop-down, clamped so a title near the right
//! edge still shows its whole panel.

use super::metrics::{panel_w, title_x};
use crate::render::layout::menubar_height;
use crate::state::Context;

pub(super) fn origin(ctx: &Context, index: usize) -> (u32, u32) {
    let x = title_x(ctx, index).min(ctx.width.saturating_sub(panel_w(ctx, index)));
    (x, menubar_height())
}
