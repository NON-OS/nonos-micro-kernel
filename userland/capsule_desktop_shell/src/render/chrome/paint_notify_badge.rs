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

use crate::render::layout::{menubar_rect, Rect};
use crate::state::Context;

pub fn paint_notify_badge(ctx: &Context) {
    let Some(level) = ctx.last_notify_level else {
        return;
    };
    let bar = menubar_rect(ctx.width);
    if bar.width < super::constants::NOTIFY_BADGE + super::constants::NOTIFY_RIGHT_INSET {
        return;
    }
    let x = bar.x + bar.width - super::constants::NOTIFY_RIGHT_INSET;
    let y = bar.y + (bar.height.saturating_sub(super::constants::NOTIFY_BADGE)) / 2;
    super::paint_rect::paint_rect(
        ctx,
        Rect {
            x: x.saturating_sub(super::constants::NOTIFY_BADGE),
            y,
            width: super::constants::NOTIFY_BADGE,
            height: super::constants::NOTIFY_BADGE,
        },
        level.tint(),
    );
}
