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

//! A small dot in the last notification's colour, at the left of the cluster.

use super::metrics::dot;
use crate::render::fill::fill_rect;
use crate::state::Context;

pub(super) fn notify_dot(ctx: &Context, x: u32, y: u32) {
    if let Some(level) = ctx.last_notify_level {
        let d = dot();
        fill_rect(ctx.backing_va, ctx.stride, ctx.width, ctx.height, x, y, d, d, level.tint());
    }
}
