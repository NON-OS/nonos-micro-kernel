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

use crate::state::Context;

use super::super::fill::blit_rgba8_scaled;

// Real brand mark, from the shared icon source at 128x139 RGBA and
// alpha-blitted so the dock icon stays crisp at any size. `color` is ignored:
// the asset already carries the brand teal.
const ICON: &[u8] = include_bytes!("../../../../assets/icons/nonos_logo.rgba");
const ICON_W: u32 = 128;
const ICON_H: u32 = 139;

pub fn paint(ctx: &Context, x: u32, y: u32, size: u32, _color: u32) {
    if size == 0 {
        return;
    }
    let dh = size;
    let dw = (size * ICON_W / ICON_H).max(1);
    let dx = x + size.saturating_sub(dw) / 2;
    blit_rgba8_scaled(
        ctx.backing_va,
        ctx.stride,
        ctx.width,
        ctx.height,
        dx,
        y,
        dw,
        dh,
        ICON,
        ICON_W,
        ICON_H,
    );
}
