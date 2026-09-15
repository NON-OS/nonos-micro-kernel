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

//! Icon masks rasterised once at boot and tinted at blit time.
//!
//! The sprite rasteriser supersamples sixteen times per pixel, which is far too
//! expensive to repeat on a frame that repaints at twenty hertz. Each glyph is
//! therefore rendered once into a white mask and every draw walks the mask with
//! nearest-neighbour sampling, so a tint costs only the destination pixels.

extern crate alloc;

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use super::geometry::Rect;
use super::sprite::{
    bell, chevron, check, close, compass, download, gear, grid, heart, home, magnifier, next, note,
    pause, play, plus, prev, radio, repeat, shuffle, speaker, Sprite,
};

const SRC: u32 = 64;
const WHITE: u32 = 0x00FF_FFFF;

#[derive(Clone, Copy)]
pub enum Glyph {
    Play,
    Pause,
    Prev,
    Next,
    Shuffle,
    Repeat,
    Speaker,
    Note,
    Search,
    Home,
    Grid,
    Gear,
    Chevron,
    Download,
    Heart,
    Check,
    Close,
    Plus,
    Compass,
    Radio,
    Bell,
}

pub struct Icons {
    masks: Vec<Sprite>,
}

impl Icons {
    pub fn new() -> Icons {
        let builders: [fn(u32, u32) -> Sprite; 21] = [
            play, pause, prev, next, shuffle, repeat, speaker, note, magnifier, home, grid, gear,
            chevron, download, heart, check, close, plus, compass, radio, bell,
        ];
        Icons { masks: builders.iter().map(|f| f(SRC, WHITE)).collect() }
    }

    pub fn draw(&self, fb: &mut PaintBuffer, r: Rect, g: Glyph, argb: u32) {
        let m = &self.masks[g as usize];
        let a = (argb >> 24) & 0xFF;
        if r.w <= 0 || r.h <= 0 || a == 0 {
            return;
        }
        let rgb = argb & 0x00FF_FFFF;
        for dy in 0..r.h {
            let sy = (dy as u32 * m.h / r.h as u32).min(m.h - 1);
            for dx in 0..r.w {
                let sx = (dx as u32 * m.w / r.w as u32).min(m.w - 1);
                let sa = m.rgba[((sy * m.w + sx) * 4 + 3) as usize] as u32;
                if sa == 0 {
                    continue;
                }
                let (px, py) = (r.x + dx, r.y + dy);
                if px >= 0 && py >= 0 {
                    fb.blend_px(px as u32, py as u32, ((sa * a / 255) << 24) | rgb);
                }
            }
        }
    }

    pub fn centred(&self, fb: &mut PaintBuffer, r: Rect, size: i32, g: Glyph, argb: u32) {
        self.draw(fb, r.centred(size, size), g, argb);
    }
}
