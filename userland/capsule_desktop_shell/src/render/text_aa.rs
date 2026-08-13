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

//! The desktop's antialiased draw path. Placement is by line box top, and the
//! advance it returns comes from the same face metrics `measure_aa` reads, so
//! layout and paint never disagree.

use nonos_toolkit::font::ttf;

use super::ui_font::valid_str;
use crate::state::Context;

/// Draw `text` with its line box top at `top_y`. Returns the final pen x.
pub fn text_aa(ctx: &Context, x: u32, top_y: u32, text: &str, argb: u32, px: f32) -> u32 {
    let stride_words = (ctx.stride / 4) as usize;
    let words = stride_words * ctx.height as usize;
    let pixels = unsafe { core::slice::from_raw_parts_mut(ctx.backing_va as *mut u32, words) };
    let pen = ttf::draw_text(
        pixels,
        stride_words,
        ctx.width,
        ctx.height,
        x as i32,
        top_y as i32,
        text,
        argb,
        super::ui_font::scaled(px),
        false,
    );
    pen.max(0) as u32
}

/// Byte-slice form, for labels that arrive from state and IPC as `&[u8]`.
pub fn text_aa_bytes(ctx: &Context, x: u32, top_y: u32, bytes: &[u8], argb: u32, px: f32) -> u32 {
    text_aa(ctx, x, top_y, valid_str(bytes), argb, px)
}
