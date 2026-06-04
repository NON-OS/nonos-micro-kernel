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

use crate::geometry::Rect;
use crate::state::Context;
use crate::window::Kind;

use super::constants::{PLACEMENT_GAP, PLACEMENT_LEFT, PLACEMENT_STEP, PLACEMENT_TOP};

pub(super) fn fallback_slot(ctx: &Context, requested: Rect, max_x: u32, max_y: u32) -> Rect {
    let open = ctx.windows.windows().filter(|w| w.kind == Kind::Normal).count() as u32;
    let step = PLACEMENT_STEP + PLACEMENT_GAP;
    Rect {
        x: PLACEMENT_LEFT.saturating_add(open.saturating_mul(step)).min(max_x),
        y: PLACEMENT_TOP.saturating_add(open.saturating_mul(step)).min(max_y),
        width: requested.width,
        height: requested.height,
    }
}
