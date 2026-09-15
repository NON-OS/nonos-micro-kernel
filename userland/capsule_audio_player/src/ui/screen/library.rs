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

//! Library: a tab bar over a scrolling track table.

use nonos_app_skeleton::PaintBuffer;

use crate::library::{Library, Queue};
use crate::ui::geometry::Rect;
use crate::ui::icon::Icons;
use crate::ui::state::LIB_TABS;
use crate::ui::text::count;
use crate::ui::widget::{page_header, row, table_header, tab_bar, Flags, ROW_H};

use super::lib_geom::{head_rect, list_rect, tabs_rect, visible};

pub fn paint(
    fb: &mut PaintBuffer,
    icons: &Icons,
    r: Rect,
    lib: &Library,
    queue: &Queue,
    rows: &[usize],
    tab: usize,
    scroll: usize,
    playing: Option<usize>,
    hover: Option<usize>,
    step: u32,
) {
    let sub = count(rows.len(), "track", "tracks");
    page_header(fb, r, "Library", &sub, "");
    tab_bar(fb, tabs_rect(r), &LIB_TABS, tab);
    table_header(fb, head_rect(r));
    let l = list_rect(r);
    for slot in 0..visible(r) {
        let Some(&idx) = rows.get(scroll + slot) else { break };
        let Some(t) = lib.get(idx) else { continue };
        let rr = Rect::new(l.x, l.y + slot as i32 * ROW_H, l.w, ROW_H);
        let f = Flags {
            playing: playing == Some(idx),
            queued: queue.contains(idx),
            hover: hover == Some(idx),
        };
        row(fb, icons, rr, t, idx + 1, &f, step);
    }
}
