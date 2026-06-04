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

use crate::geometry::{clamp_to_display, Rect};
use crate::state::Context;
use crate::window::Kind;

use super::collides::collides;
use super::constants::{PLACEMENT_LEFT, PLACEMENT_STEP, PLACEMENT_TOP};
use super::fallback_slot::fallback_slot;

pub(super) fn place(ctx: &Context, kind: Kind, requested: Rect) -> Rect {
    let requested = clamp_to_display(requested, ctx.display_width, ctx.display_height);
    if kind != Kind::Normal || !collides(ctx, requested) {
        return requested;
    }
    let max_x = ctx.display_width.saturating_sub(requested.width);
    let max_y = ctx.display_height.saturating_sub(requested.height);
    let mut y = PLACEMENT_TOP.min(max_y);
    while y <= max_y {
        let mut x = PLACEMENT_LEFT.min(max_x);
        while x <= max_x {
            let candidate = Rect { x, y, width: requested.width, height: requested.height };
            if !collides(ctx, candidate) {
                return candidate;
            }
            if max_x - x < PLACEMENT_STEP {
                break;
            }
            x += PLACEMENT_STEP;
        }
        if max_y - y < PLACEMENT_STEP {
            break;
        }
        y += PLACEMENT_STEP;
    }
    fallback_slot(ctx, requested, max_x, max_y)
}
