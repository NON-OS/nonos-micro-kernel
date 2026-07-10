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

//! Place a newly opened window. Normal windows cascade down-right from a fixed
//! base by the count of open normal windows, so each successive titlebar sits
//! below the last and stays clickable: with reachable titlebars, clicking one
//! is a reliable way to switch windows even when they overlap. Non-normal
//! windows (dialogs, tooltips) keep their requested position.

use crate::geometry::{clamp_to_display, Rect};
use crate::state::Context;
use crate::window::{Kind, Visibility};

use super::constants::{PLACEMENT_GAP, PLACEMENT_LEFT, PLACEMENT_STEP, PLACEMENT_TOP};

// Cascade resets after this many windows so they never march off-screen.
const CASCADE_WRAP: u32 = 5;

pub(super) fn place(ctx: &Context, kind: Kind, requested: Rect) -> Rect {
    let requested = clamp_to_display(requested, ctx.display_width, ctx.display_height);
    if kind != Kind::Normal {
        return requested;
    }
    let open = ctx
        .windows
        .windows()
        .filter(|w| w.kind == Kind::Normal && w.visibility == Visibility::Visible)
        .count() as u32;
    let step = PLACEMENT_STEP + PLACEMENT_GAP;
    let slot = open % CASCADE_WRAP;
    let max_x = ctx.display_width.saturating_sub(requested.width);
    let max_y = ctx.display_height.saturating_sub(requested.height);
    Rect {
        x: PLACEMENT_LEFT.saturating_add(slot * step).min(max_x),
        y: PLACEMENT_TOP.saturating_add(slot * step).min(max_y),
        width: requested.width,
        height: requested.height,
    }
}
