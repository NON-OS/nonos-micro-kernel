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

use crate::wallet::theme::{ACCENT, FG, INK, LINE2};

const BTN_H: u32 = 42;

// Primary action: solid cyan fill with dark ink label.
pub fn primary(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &[u8]) {
    fb.fill_rect(x, y, w, BTN_H, ACCENT());
    label(fb, x, y, w, text, INK());
}

// Secondary action: outlined, light label.
pub fn outline(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &[u8]) {
    border(fb, x, y, w, LINE2());
    label(fb, x, y, w, text, FG());
}

fn label(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &[u8], color: u32) {
    let s = core::str::from_utf8(text).unwrap_or("");
    let tw = fb.measure_ttf(s, 14.0).max(0) as u32;
    let tx = x + w.saturating_sub(tw) / 2;
    let _ = fb.text_ttf(tx as i32, (y + 12) as i32, s, color, 14.0);
}

fn border(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, c: u32) {
    fb.fill_rect(x, y, w, 1, c);
    fb.fill_rect(x, y + BTN_H - 1, w, 1, c);
    fb.fill_rect(x, y, 1, BTN_H, c);
    fb.fill_rect(x + w - 1, y, 1, BTN_H, c);
}
