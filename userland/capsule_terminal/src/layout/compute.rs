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

use super::limits::{LEFT_RAIL_MIN_W, MIN_BODY_W, RIGHT_RAIL_MIN_W};
use super::types::{Chrome, Layout, Rails, Rect};

pub fn compute(w: u32, h: u32, c: &Chrome, r: Rails) -> Layout {
    let titlebar_h = c.titlebar_h.min(h);
    let tab_y = titlebar_h;
    let tabstrip_h = c.tabstrip_h.min(h.saturating_sub(tab_y));
    let content_y = tab_y + tabstrip_h;
    let footer_h = c.footer_h.min(h.saturating_sub(content_y));
    let content_h = h.saturating_sub(content_y + footer_h);
    let input_h = c.row_h.min(content_h);
    let body_h = content_h - input_h;

    let (lw, rw) = rail_widths(w, r);
    let body_w = w.saturating_sub(lw + rw);

    Layout {
        titlebar: Rect::new(0, 0, w, titlebar_h),
        tabstrip: Rect::new(0, tab_y, w, tabstrip_h),
        left_rail: Rect::new(0, content_y, lw, content_h),
        right_rail: Rect::new(lw + body_w, content_y, rw, content_h),
        body: Rect::new(lw, content_y, body_w, body_h),
        input: Rect::new(lw, content_y + body_h, body_w, input_h),
        footer: Rect::new(0, content_y + content_h, w, footer_h),
    }
}

fn rail_widths(w: u32, r: Rails) -> (u32, u32) {
    let mut left = if w >= LEFT_RAIL_MIN_W { r.left } else { 0 };
    let mut right = if w >= RIGHT_RAIL_MIN_W { r.right } else { 0 };
    let floor = MIN_BODY_W.min(w);

    if w.saturating_sub(left + right) < floor {
        right = w.saturating_sub(left + floor).min(right);
    }
    if w.saturating_sub(left + right) < floor {
        left = w.saturating_sub(right + floor).min(left);
    }
    (left, right)
}
