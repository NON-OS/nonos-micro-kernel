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
use nonos_toolkit::paint::PaintBuffer;

/// A `PaintBuffer` over the shell's backing store, so chrome can draw with the
/// shared primitives instead of raw writes. The buffer aliases the surface, so a
/// caller must finish with one before taking another.
pub fn surface(ctx: &Context) -> PaintBuffer<'_> {
    let stride_words = (ctx.stride / 4) as usize;
    let words = stride_words * ctx.height as usize;
    let pixels = unsafe { core::slice::from_raw_parts_mut(ctx.backing_va as *mut u32, words) };
    PaintBuffer { pixels, stride_words: stride_words as u32, width: ctx.width, height: ctx.height }
}
