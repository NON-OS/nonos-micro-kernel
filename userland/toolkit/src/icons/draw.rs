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


use super::id::IconId;
use super::table::mask;
use crate::paint::resample::{cover, source_span};
use crate::paint::PaintBuffer;

/// Blend `id` into `buf` at `size` square, area-averaging the mask and taking
/// colour from `tint`. The mask carries coverage only, so one asset serves
/// every accent the caller wants.
pub fn draw(buf: &mut PaintBuffer, id: IconId, x: u32, y: u32, size: u32, tint: u32) {
    draw_mask(buf, mask(id), x, y, size, tint);
}

/// The same blend for a mask the caller already holds, for icons that are not
/// part of the shared set. `src` must be square; its side is recovered from the
/// length exactly as `dim` does.
pub fn draw_mask(buf: &mut PaintBuffer, src: &[u8], x: u32, y: u32, size: u32, tint: u32) {
    let sd = (src.len() as u32).isqrt();
    if size == 0 || sd == 0 {
        return;
    }
    let rgb = tint & 0x00FF_FFFF;
    let ta = (tint >> 24) as u64;
    for row in 0..size {
        let (y0, y1) = source_span(row, sd, size);
        let sy_last = core::cmp::min((y1 - 1) >> 16, sd as u64 - 1);
        for col in 0..size {
            let (x0, x1) = source_span(col, sd, size);
            let sx_last = core::cmp::min((x1 - 1) >> 16, sd as u64 - 1);
            let (mut acc, mut wsum) = (0u64, 0u64);
            for sy in (y0 >> 16)..=sy_last {
                let wy = cover(y0, y1, sy);
                let base = (sy * sd as u64) as usize;
                for sx in (x0 >> 16)..=sx_last {
                    let w = wy * cover(x0, x1, sx);
                    acc += src[base + sx as usize] as u64 * w;
                    wsum += w;
                }
            }
            if wsum == 0 {
                continue;
            }
            let a = acc * ta / (wsum * 255);
            if a == 0 {
                continue;
            }
            buf.blend_px(x + col, y + row, ((a as u32) << 24) | rgb);
        }
    }
}
