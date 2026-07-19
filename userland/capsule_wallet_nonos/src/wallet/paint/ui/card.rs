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

use crate::wallet::theme::{LINE, PANEL};

// A flat surface card: panel fill with a single hairline border on all edges.
pub fn card(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, h: u32) {
    if w == 0 || h == 0 {
        return;
    }
    fb.fill_rect(x, y, w, h, PANEL);
    fb.fill_rect(x, y, w, 1, LINE);
    fb.fill_rect(x, y + h - 1, w, 1, LINE);
    fb.fill_rect(x, y, 1, h, LINE);
    fb.fill_rect(x + w - 1, y, 1, h, LINE);
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
