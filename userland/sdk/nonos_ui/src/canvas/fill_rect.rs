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

use super::types::Canvas;
use crate::color::Color;
use crate::rect::Rect;

impl Canvas<'_> {
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let bits = color.bits();
        let x2 = rect.x.saturating_add(rect.w).min(self.width);
        let y2 = rect.y.saturating_add(rect.h).min(self.height);
        for y in rect.y.min(self.height)..y2 {
            let row = (y * self.width) as usize;
            for x in rect.x.min(self.width)..x2 {
                let index = row + x as usize;
                if index < self.buf.len() {
                    self.buf[index] = bits;
                }
            }
        }
    }
}
