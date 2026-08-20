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


use super::PaintBuffer;

impl<'a> PaintBuffer<'a> {
    pub fn sub<'b>(&'b mut self, x: u32, y: u32, w: u32, h: u32) -> PaintBuffer<'b> {
        let x = x.min(self.width);
        let y = y.min(self.height);
        let w = w.min(self.width - x);
        let h = h.min(self.height - y);
        let stride = self.stride_words;
        let len = self.pixels.len();
        let off = ((y * stride + x) as usize).min(len);
        let span = if h == 0 || w == 0 {
            0
        } else {
            (((h - 1) * stride + w) as usize).min(len - off)
        };
        let (w, h) = if span == 0 { (0, 0) } else { (w, h) };
        PaintBuffer {
            pixels: &mut self.pixels[off..off + span],
            stride_words: stride,
            width: w,
            height: h,
        }
    }
}
