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

use nonos_toolkit::font::render::draw_text;

use crate::state::Context;

pub fn draw_overlay_text(ctx: &Context, x: u32, y: u32, text: &[u8], color: u32) {
    let words = (ctx.stride as usize * ctx.height as usize) / 4;
    let pixels = unsafe { core::slice::from_raw_parts_mut(ctx.backing_va as *mut u32, words) };
    draw_text(pixels, (ctx.stride / 4) as usize, ctx.width, ctx.height, x, y, text, color);
}
