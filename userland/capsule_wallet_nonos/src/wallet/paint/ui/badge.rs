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

// A solid pill: filled background with dark ink label. Used for status badges
// and rail chips. Returns the width drawn so callers can lay out siblings.
pub fn badge(fb: &mut PaintBuffer, x: u32, y: u32, text: &[u8], bg: u32, fg: u32) -> u32 {
    let s = core::str::from_utf8(text).unwrap_or("");
    let tw = fb.measure_ttf(s, 11.0).max(0) as u32;
    let w = tw + 18;
    fb.fill_rect(x, y, w, 20, bg);
    let _ = fb.text_ttf((x + 9) as i32, (y + 4) as i32, s, fg, 11.0);
    w
}

// Alias for sidebar rail chips; identical shape to a badge.
pub fn chip(fb: &mut PaintBuffer, x: u32, y: u32, text: &[u8], bg: u32, fg: u32) -> u32 {
    badge(fb, x, y, text, bg, fg)
}
