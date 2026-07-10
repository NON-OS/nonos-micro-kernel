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

use crate::wallet::theme::{ACCENT, BG, FG, LINE, MUTED, PANEL_2};

const BTN_H: u32 = 44;

// Primary action: filled with an accent underline. Use one per view.
pub fn primary(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &[u8]) {
    fb.fill_rect(x, y, w, BTN_H, PANEL_2);
    fb.fill_rect(x, y + BTN_H - 2, w, 2, ACCENT);
    corners(fb, x, y, w);
    fb.text(x + 22, y + 15, text, FG);
}

// Secondary action: outlined, muted. For cancel / less-important choices.
pub fn ghost(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &[u8]) {
    fb.fill_rect(x, y, w, 1, LINE);
    fb.fill_rect(x, y + BTN_H - 1, w, 1, LINE);
    fb.fill_rect(x, y, 1, BTN_H, LINE);
    fb.fill_rect(x + w - 1, y, 1, BTN_H, LINE);
    corners(fb, x, y, w);
    fb.text(x + 22, y + 15, text, MUTED);
}

// A disabled action: rendered dim, no accent. Pairs with the clear-sign gate.
pub fn disabled(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, text: &[u8]) {
    fb.fill_rect(x, y, w, BTN_H, BG);
    fb.fill_rect(x, y, w, 1, LINE);
    fb.fill_rect(x, y + BTN_H - 1, w, 1, LINE);
    corners(fb, x, y, w);
    fb.text(x + 22, y + 15, text, MUTED);
}

fn corners(fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let rx = x + w - 1;
    let ry = y + BTN_H - 1;
    fb.fill_rect(x, y, 1, 1, BG);
    fb.fill_rect(rx, y, 1, 1, BG);
    fb.fill_rect(x, ry, 1, 1, BG);
    fb.fill_rect(rx, ry, 1, 1, BG);
}
