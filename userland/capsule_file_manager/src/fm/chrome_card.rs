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

use nonos_app_skeleton::PaintBuffer;
use super::chrome_glow::glow_out;
use super::theme::{HAIR, R_CARD};

/// A raised plate: rounded fill, 1px hairline, optional vertical tint, optional
/// outer glow. `const` setters, so a style table can sit beside layout consts.
#[derive(Clone, Copy)]
pub struct Plate {
    fill: u32,
    line: u32,
    r: u32,
    tint: Option<(u32, u32)>,
    glow: u32,
}

impl Plate {
    pub const fn new(fill: u32) -> Self {
        Self { fill, line: HAIR, r: R_CARD, tint: None, glow: 0 }
    }

    pub const fn radius(mut self, r: u32) -> Self {
        self.r = r;
        self
    }

    pub const fn line(mut self, line: u32) -> Self {
        self.line = line;
        self
    }

    pub const fn tint(mut self, top: u32, bottom: u32) -> Self {
        self.tint = Some((top, bottom));
        self
    }

    pub const fn glow(mut self, spread: u32) -> Self {
        self.glow = spread;
        self
    }

    /// Blends throughout, so a plate is safe over live paint. The tint ramps
    /// across the straight rows only; a rect gradient would square the corners.
    pub fn draw(&self, fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
        if w == 0 || h == 0 {
            return;
        }
        if self.glow > 0 {
            glow_out(fb, x, y, w, h, self.r, self.glow);
        }
        fb.fill_round(x, y, w, h, self.r, self.fill);
        if let Some((top, bottom)) = self.tint {
            let cap = self.r.min(h / 2);
            if h > cap * 2 {
                fb.gradient_v(x, y + cap, w, h - cap * 2, top, bottom);
            }
        }
        fb.stroke_round(x, y, w, h, self.r, 1, self.line);
    }
}
