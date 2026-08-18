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

use nonos_app_skeleton::paint::PaintBuffer;

use crate::ui::fit::width;
use crate::ui::layout::Rect;
use crate::ui::paint::shape;
use crate::ui::text::{center_y, BODY_PX};
use crate::ui::theme;

pub const TABS_H: u32 = 42;
const PAD: u32 = 16;
const RULE: u32 = 3;

pub fn tab_rect(x: u32, y: u32, labels: &[&str], index: usize) -> Rect {
    let mut pen = x;
    for label in labels.iter().take(index) {
        pen += width(label, BODY_PX) + PAD * 2;
    }
    let w = labels.get(index).map(|l| width(l, BODY_PX) + PAD * 2).unwrap_or(0);
    Rect { x: pen, y, w, h: TABS_H }
}

pub fn tab_hit(x: u32, y: u32, labels: &[&str], px: i32, py: i32) -> Option<usize> {
    (0..labels.len()).find(|&i| tab_rect(x, y, labels, i).contains(px, py))
}

pub fn paint_tabs(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, labels: &[&str], active: usize) {
    shape::hline(fb, x, y + TABS_H, w, theme::BORDER);
    for (i, label) in labels.iter().enumerate() {
        let r = tab_rect(x, y, labels, i);
        let ink = if i == active { theme::TEXT } else { theme::TEXT_MUTED };
        fb.text_ttf((r.x + PAD) as i32, center_y(r.y, TABS_H), label, ink, BODY_PX);
        if i == active {
            fb.fill_rect(r.x, y + TABS_H + 1 - RULE, r.w, RULE, theme::ACCENT);
        }
    }
}
