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
use crate::components::list::render_list_item;
use crate::font::render::draw_text;

use super::fill_rect::fill_rect;
use super::style::DropdownStyle;

pub fn render_dropdown(
    buf: &mut [u32],
    stride: usize,
    w: u32,
    h: u32,
    x: u32,
    y: u32,
    rw: u32,
    row_h: u32,
    selected: &[u8],
    expanded: bool,
    options: &[&[u8]],
    index: usize,
    style: DropdownStyle,
) {
    fill_rect(buf, stride, w, h, (x, y, rw, row_h), style.bg.as_u32());
    draw_text(
        buf,
        stride,
        w,
        h,
        x.saturating_add(4),
        y.saturating_add(4),
        selected,
        style.fg.as_u32(),
    );
    if !expanded {
        return;
    }
    let mut i = 0usize;
    while i < options.len() {
        let oy = y.saturating_add(row_h.saturating_mul((i + 1) as u32));
        render_list_item(buf, stride, w, h, x, oy, rw, row_h, options[i], i == index, style.list);
        i += 1;
    }
}
