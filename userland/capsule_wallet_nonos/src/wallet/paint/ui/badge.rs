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

use crate::wallet::theme::BG;

fn mix(over: u32, base: u32, bw: u32) -> u32 {
    let ow = 256 - bw;
    let c = |sh: u32| (((over >> sh) & 0xff) * ow + ((base >> sh) & 0xff) * bw) / 256;
    0xFF00_0000 | (c(16) << 16) | (c(8) << 8) | c(0)
}

// A filled status pill: dark tone-tinted fill, tone label, clipped corners.
// Returns the width drawn so callers can lay out following content.
pub fn badge(fb: &mut PaintBuffer, x: u32, y: u32, text: &[u8], tone: u32) -> u32 {
    let s = core::str::from_utf8(text).unwrap_or("");
    let tw = fb.measure_ttf(s, 11.0).max(0) as u32;
    let w = tw + 20;
    let h = 22;
    fb.fill_rect(x, y, w, h, mix(tone, BG, 210));
    fb.fill_rect(x, y, 1, 1, BG);
    fb.fill_rect(x + w - 1, y, 1, 1, BG);
    fb.fill_rect(x, y + h - 1, 1, 1, BG);
    fb.fill_rect(x + w - 1, y + h - 1, 1, 1, BG);
    let _ = fb.text_ttf((x + 10) as i32, (y + 5) as i32, s, tone, 11.0);
    w
}
