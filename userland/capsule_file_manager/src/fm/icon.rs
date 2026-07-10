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

// Small vector-ish glyphs built from filled rects. Crisp at any size and
// on-brand with the flat UI, so the list reads as files/folders at a glance
// instead of a wall of text.

/// Folder: a body with a raised tab on the left, hollowed so it reads as an
/// outline rather than a solid block.
pub fn folder(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, c: u32, bg: u32) {
    let tab_h = (s / 4).max(2);
    let tab_w = s / 2;
    fb.fill_rect(x, y + tab_h, s, s.saturating_sub(tab_h), c);
    fb.fill_rect(x, y + tab_h / 2, tab_w, tab_h, c);
    // hollow the body so it looks like an icon, not a filled square
    let b = 2u32;
    fb.fill_rect(
        x + b,
        y + tab_h + b,
        s.saturating_sub(2 * b),
        s.saturating_sub(tab_h + 2 * b),
        bg,
    );
}

/// File: a page with a folded top-right corner.
pub fn file(fb: &mut PaintBuffer, x: u32, y: u32, s: u32, c: u32, bg: u32) {
    let w = (s * 3 / 4).max(3);
    let ox = x + (s - w) / 2;
    fb.fill_rect(ox, y, w, s, c);
    // hollow interior
    let b = 2u32;
    fb.fill_rect(ox + b, y + b, w.saturating_sub(2 * b), s.saturating_sub(2 * b), bg);
    // folded corner (cut the top-right, then a small mark)
    let fold = (s / 3).max(2);
    fb.fill_rect(ox + w - fold, y, fold, fold, bg);
    fb.fill_rect(ox + w - fold, y + fold - 1, fold, 1, c);
    fb.fill_rect(ox + w - 1, y, 1, fold, c);
}
