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

use super::shade::{darken, lighten};
use crate::wallet::theme::{LINE, PANEL};

// An elevated surface card. A panel fill, a soft hairline border, then a lit top
// edge and a shadowed bottom edge so the card reads as raised from the page
// instead of a flat wireframe box.
pub fn card(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    if w == 0 || h == 0 {
        return;
    }
    let panel = PANEL();
    fb.fill_rect(x, y, w, h, panel);
    fb.fill_rect(x, y, w, 1, LINE());
    fb.fill_rect(x, y + h - 1, w, 1, LINE());
    fb.fill_rect(x, y, 1, h, LINE());
    fb.fill_rect(x + w - 1, y, 1, h, LINE());
    // Elevation: a faint highlight just inside the top, a faint shade inside the
    // bottom. One pixel each, subtle enough to feel like depth, not a stripe.
    if h > 3 {
        fb.fill_rect(x + 1, y + 1, w.saturating_sub(2), 1, lighten(panel, 0x0C));
        fb.fill_rect(x + 1, y + h - 2, w.saturating_sub(2), 1, darken(panel, 0x06));
    }
}

// Same border on an arbitrary fill (e.g. an inset well).
pub fn bordered(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, fill: u32, e: u32) {
    fb.fill_rect(x, y, w, h, fill);
    edge(fb, x, y, w, h, e);
}

// A hairline border only, no fill (transparent controls).
pub fn edge(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32, e: u32) {
    fb.fill_rect(x, y, w, 1, e);
    fb.fill_rect(x, y + h - 1, w, 1, e);
    fb.fill_rect(x, y, 1, h, e);
    fb.fill_rect(x + w - 1, y, 1, h, e);
}
