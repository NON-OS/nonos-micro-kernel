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
    let dst = Surface {
        base_va: ctx.backing_va,
        stride: ctx.stride,
        width: ctx.width,
        height: ctx.height,
        byte_len: ctx.backing_len,
    };
    sw_blitter::fill_rect(dst, rect, BACKGROUND_ARGB);
    debug::marker(b"fill ok");
    let (layers, count) = ctx.scene.z_sorted_snapshot();
    let mut tag = [b'l', b'a', b'y', b'e', b'r', b's', b'=', b'0'];
    tag[7] = b'0' + (count.min(9) as u8);
    debug::marker(&tag);
    let mut composited: u32 = 0;
    for layer in layers.iter().take(count) {
        if let Some(src) = ctx.attach.get_or_attach(layer.surface_handle) {
            let first: u32 = unsafe { core::ptr::read_volatile(src.base_va as *const u32) };
            let mut tag = [b'p', b'x', b'=', b'0', b'0', b'0', b'0', b'0', b'0', b'0', b'0'];
            for i in 0..8u32 {
                let nyb = ((first >> ((7 - i) * 4)) & 0xF) as u8;
                tag[3 + i as usize] = if nyb < 10 { b'0' + nyb } else { b'a' + nyb - 10 };
            }
            debug::marker(&tag);
            sw_blitter::composite_layer(
                dst,
                src,
                layer.x,
                layer.y,
                layer.width,
                layer.height,
                rect,
            );
            composited += 1;
        }
    }
    let mut done = [b'd', b'o', b'n', b'e', b'=', b'0'];
    done[5] = b'0' + (composited.min(9) as u8);
    debug::marker(&done);
}
