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
use nonos_toolkit::font::ttf;

use super::registry::with_face;

// Bold cuts store under the family key with this bit set, so one registry
// holds both weights without a second key namespace.
pub const BOLD_KEY: u32 = 1 << 31;

// One text run's face and geometry, grouped so calls stay readable.
pub struct TextRun {
    pub key: u32,
    pub mono: bool,
    pub bold: bool,
    pub x: i32,
    pub top_y: i32,
    pub px: f32,
    pub spacing: f32,
}

// Advance width for the run, resolving the face the same way the draw will:
// the family's bold cut, then its regular one, then the built-in weight.
pub fn measure_text(key: u32, mono: bool, bold: bool, text: &str, px: f32, spacing: f32) -> i32 {
    let custom = if bold {
        with_face(key | BOLD_KEY, |f| ttf::measure_tracked(f, text, px, spacing))
            .or_else(|| with_face(key, |f| ttf::measure_tracked(f, text, px, spacing)))
    } else {
        with_face(key, |f| ttf::measure_tracked(f, text, px, spacing))
    };
    custom.unwrap_or_else(|| {
        ttf::builtin_face(mono, bold)
            .map(|f| ttf::measure_tracked(f, text, px, spacing))
            .unwrap_or(0)
    })
}

// Draw the run with the resolved face. Returns true when a true bold cut was
// used, so the caller knows the fake thickening is not needed.
pub fn draw_text(fb: &mut PaintBuffer, run: TextRun, text: &str, argb: u32) -> bool {
    let stride = fb.stride_words as usize;
    let (w, h) = (fb.width, fb.height);
    let TextRun { key, mono, bold, x, top_y, px, spacing } = run;
    if bold {
        let drew = with_face(key | BOLD_KEY, |f| {
            ttf::draw_text_tracked(f, fb.pixels, stride, w, h, x, top_y, text, argb, px, spacing);
        });
        if drew.is_some() {
            return true;
        }
    }
    let drew = with_face(key, |f| {
        ttf::draw_text_tracked(f, fb.pixels, stride, w, h, x, top_y, text, argb, px, spacing);
    });
    if drew.is_some() {
        return false;
    }
    if let Some(f) = ttf::builtin_face(mono, bold) {
        ttf::draw_text_tracked(f, fb.pixels, stride, w, h, x, top_y, text, argb, px, spacing);
        return bold && !mono;
    }
    false
}
