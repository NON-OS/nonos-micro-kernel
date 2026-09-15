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

//! Stat tiles and the storage meter.

use nonos_app_skeleton::PaintBuffer;

use crate::ui::geometry::Rect;
use crate::ui::icon::{Glyph, Icons};
use crate::ui::metrics::{line_h, DATA, LABEL, R_TILE, S3, S4};
use crate::ui::paint::{fill, panel, text, text_right};
use crate::ui::text::upper;
use crate::ui::theme::{alpha, rgb, AMBER, CYAN, CYAN_DIM, DEEP, EDGE, GREEN, INK, MUTE, RAISED};

pub const TILE_H: i32 = 96;

pub fn stat_tile(fb: &mut PaintBuffer, icons: &Icons, r: Rect, value: &str, label: &str, g: Glyph) {
    panel(fb, r, R_TILE, RAISED, EDGE);
    let inner = r.pad(S4, (r.h - 20).max(0) / 2);
    icons.draw(fb, Rect::new(inner.x, inner.y, 20, 20), g, alpha(rgb(CYAN), 0xCC));
    let lx = inner.x + 20 + S3;
    text(fb, lx, inner.y + (20 - line_h(LABEL)) / 2, &upper(label), MUTE, LABEL);
    let vr = Rect::new(lx, inner.y + (20 - line_h(DATA)) / 2, inner.right() - lx, line_h(DATA));
    text_right(fb, vr, value, INK, DATA);
}

pub fn storage_meter(fb: &mut PaintBuffer, r: Rect, used: u32, total: u32, used_label: &str) {
    panel(fb, r, R_TILE, RAISED, EDGE);
    let inner = r.pad(S4, S4);
    text(fb, inner.x, inner.y, &upper("Storage"), MUTE, LABEL);
    text_right(fb, Rect::new(inner.x, inner.y, inner.w, line_h(LABEL)), used_label, INK, LABEL);
    let bar = Rect::new(inner.x, inner.bottom() - 10, inner.w, 8);
    fill(fb, bar, 4, DEEP);
    let pct = if total == 0 { 0 } else { (used * 100 / total).min(100) };
    let w = bar.w * pct as i32 / 100;
    let c = if pct > 80 { AMBER } else if pct > 40 { CYAN } else { GREEN };
    if w > 0 {
        fill(fb, Rect::new(bar.x, bar.y, w, bar.h), 4, c);
    }
    fill(fb, Rect::new(bar.x, bar.y - S3 - 2, inner.w, 1), 0, alpha(rgb(CYAN_DIM), 0x22));
}
