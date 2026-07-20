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

use crate::state::attach::MAX_ATTACH;
use crate::state::damage::Rect;
use crate::state::Context;
use crate::sw_blitter::{self, Surface};

pub const BACKGROUND_ARGB: u32 = 0xFF10_1620;

const REAP_THRESHOLD: u16 = 60;

pub fn paint(ctx: &mut Context, rect: Rect) {
    let dst = Surface {
        base_va: ctx.backing_va,
        stride: ctx.stride,
        width: ctx.width,
        height: ctx.height,
        byte_len: ctx.backing_len,
    };
    sw_blitter::fill_rect(dst, rect, BACKGROUND_ARGB);
    let focused = ctx.focus.focused();
    let (layers, count) = ctx.scene.z_sorted_snapshot(focused);
    let mut attached = [0u64; MAX_ATTACH];
    let mut n_attached = 0;
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
            if n_attached < attached.len() {
                attached[n_attached] = layer.surface_handle;
                n_attached += 1;
            }
        }
    }
    let mut dropped = [0u64; MAX_ATTACH];
    let n_dropped =
        ctx.scene.reap_unattachable(&attached[..n_attached], REAP_THRESHOLD, &mut dropped);
    for &handle in dropped.iter().take(n_dropped) {
        let _ = ctx.attach.forget(handle);
    }
    let cursor = ctx.cursor.current();
    if cursor.visible {
        super::cursor::blit(
            ctx.backing_va,
            ctx.stride,
            ctx.width,
            ctx.height,
            cursor.x,
            cursor.y,
            rect,
        );
    }
}
