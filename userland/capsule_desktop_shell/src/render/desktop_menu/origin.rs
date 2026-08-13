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

//! Clamp the stored anchor so the whole panel stays on screen.

use super::height::height;
use super::metrics::width;
use crate::state::Context;

pub(super) fn origin(ctx: &Context) -> (u32, u32) {
    let (rx, ry) = ctx.desktop_menu.unwrap_or((0, 0));
    let x = rx.min(ctx.width.saturating_sub(width(ctx)));
    let y = ry.min(ctx.height.saturating_sub(height(ctx)));
    (x, y)
}
