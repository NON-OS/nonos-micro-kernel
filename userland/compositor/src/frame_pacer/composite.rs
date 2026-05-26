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

use crate::state::damage::Rect;
use crate::state::Context;
use crate::debug;
use crate::sw_blitter::{self, Surface};

pub const BACKGROUND_ARGB: u32 = 0xFF10_1620;

pub fn paint(ctx: &mut Context, rect: Rect) {
    sw_blitter::fill_rect(
        ctx.backing_va,
        ctx.stride,
        rect.x,
        rect.y,
        rect.width,
        rect.height,
        BACKGROUND_ARGB,
    );
    debug::marker(b"fill ok");
    let dst = Surface {
        base_va: ctx.backing_va,
        stride: ctx.stride,
        width: ctx.width,
        height: ctx.height,
    };
    let (layers, count) = ctx.scene.z_sorted_snapshot();
    debug::marker(b"snap ok");
    for layer in layers.iter().take(count) {
        if let Some(src) = ctx.attach.get_or_attach(layer.surface_handle) {
            sw_blitter::composite_layer(
                dst,
                src,
                layer.x,
                layer.y,
                layer.width,
                layer.height,
                rect,
            );
        }
    }
    debug::marker(b"paint done");
}
