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
    pub fn shadow_round(&mut self, x: u32, y: u32, w: u32, h: u32, r: u32, spread: u32, argb: u32) {
        if spread == 0 || w == 0 || h == 0 {
            return;
        }
        let a = (argb >> 24) & 0xFF;
        let rgb = argb & 0x00FF_FFFF;
        let denom = (spread + 1) * (spread + 1);
        for i in 1..=spread {
            let step = spread + 1 - i;
            let ea = a * step * step / denom;
            if ea == 0 {
                continue;
            }
            let ox = x.saturating_sub(i);
            let oy = y.saturating_sub(i);
            let ow = w + (x - ox) + i;
            let oh = h + (y - oy) + i;
            self.stroke_round(ox, oy, ow, oh, r + i, 1, (ea << 24) | rgb);
        }
    }
}
