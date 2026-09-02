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

use super::limits::{LEFT_RAIL_MIN_W, MIN_BODY_W};
use super::types::{Chrome, Layout, Rails, Rect};

pub fn compute(w: u32, h: u32, c: &Chrome, r: Rails) -> Layout {
    let titlebar_h = c.titlebar_h.min(h);
    let tab_y = titlebar_h;
    let tabstrip_h = c.tabstrip_h.min(h.saturating_sub(tab_y));
    let content_y = (tab_y + tabstrip_h + c.body_pad_top).min(h);
    let footer_h = c.footer_h.min(h.saturating_sub(content_y));
    let content_h = h.saturating_sub(content_y + footer_h);
    let input_h = c.row_h.min(content_h);
    let body_h = content_h - input_h;

    let lw = rail_width(w, r);
    let body_w = w.saturating_sub(lw);

    Layout {
        titlebar: Rect { x: 0, y: 0, w, h: titlebar_h },
        tabstrip: Rect { x: 0, y: tab_y, w, h: tabstrip_h },
        left_rail: Rect { x: 0, y: content_y, w: lw, h: content_h },
        body: Rect { x: lw, y: content_y, w: body_w, h: body_h },
        input: Rect { x: lw, y: content_y + body_h, w: body_w, h: input_h },
        footer: Rect { x: 0, y: content_y + content_h, w, h: footer_h },
    }
}

fn rail_width(w: u32, r: Rails) -> u32 {
    let left = if w >= LEFT_RAIL_MIN_W { r.left } else { 0 };
    let floor = MIN_BODY_W.min(w);
    if w.saturating_sub(left) < floor {
        w.saturating_sub(floor).min(left)
    } else {
        left
    }
}
