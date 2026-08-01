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

use super::row::composite_row;
use super::Surface;
use crate::state::damage::Rect;

pub fn composite_layer(
    dst: Surface,
    src: Surface,
    at_x: u32,
    at_y: u32,
    layer_w: u32,
    layer_h: u32,
    clip: Rect,
) {
    let span_w = layer_w.min(src.width);
    let span_h = layer_h.min(src.height);
    if span_w == 0 || span_h == 0 {
        return;
    }
    let clip_x1 = clip.x.saturating_add(clip.width);
    let clip_y1 = clip.y.saturating_add(clip.height);
    let x0 = at_x.max(clip.x);
    let y0 = at_y.max(clip.y);
    let x1 = at_x.saturating_add(span_w).min(dst.width).min(clip_x1);
    let y1 = at_y.saturating_add(span_h).min(dst.height).min(clip_y1);
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    let copy_w = (x1 - x0) as usize;
    let src_x0 = (x0 - at_x) as usize;
    for y in y0..y1 {
        let src_row = (y - at_y) as usize;
        let Some(dst_row_va) = dst.row_start(y, x0, x1 - x0) else {
            break;
        };
        let Some(src_row_va) = src.row_start(src_row as u32, src_x0 as u32, x1 - x0) else {
            break;
        };
        let dst_ptr = dst_row_va as *mut u32;
        let src_ptr = src_row_va as *const u32;
        // SAFETY: `row_start` bounds checked both rows for `copy_w` pixels,
        // and a layer never composites onto itself, so the two do not overlap.
        // Plain slices rather than per-pixel volatile: nothing here depends on
        // the timing of a single access, and the copy inside has to be able to
        // become a memcpy to be worth anything.
        let src_row: &[u32] = unsafe { core::slice::from_raw_parts(src_ptr, copy_w) };
        let dst_row: &mut [u32] = unsafe { core::slice::from_raw_parts_mut(dst_ptr, copy_w) };
        composite_row(src_row, dst_row);
    }
}
