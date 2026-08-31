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

use alloc::format;

use nonos_app_skeleton::paint::PaintBuffer;

use crate::app::state::VideoApp;
use crate::ui::format::{count, duration};
use crate::ui::icon;
use crate::ui::layout::Rect;
use crate::ui::paint::rrect;
use crate::ui::text::BODY_PX;
use crate::ui::theme;
use crate::ui::widget::button::{paint_lead_button, Tone};
use crate::ui::widget::empty::paint_empty;
use crate::ui::widget::scrollbar;
use crate::ui::widget::section;
use crate::ui::widget::table::{self, Column};

const RAIL_W: u32 = 230;
const GAP: u32 = 20;
const QUEUE_COLS: [Column; 3] = [
    Column { title: "#", weight: 6, min: 44 },
    Column { title: "TITLE", weight: 74, min: 200 },
    Column { title: "DURATION", weight: 20, min: 96 },
];

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    let rail = Rect { x: body.x, y: body.y, w: RAIL_W, h: body.h };
    section::paint_label(fb, rail.x, rail.y, "PLAYLISTS");
    let row = Rect { x: rail.x, y: rail.y + 28, w: rail.w, h: 54 };
    rrect::panel(fb, row.x, row.y, row.w, row.h, 10, theme::SELECT, theme::ACCENT_DIM);
    fb.text_ttf((row.x + 14) as i32, (row.y + 10) as i32, "All Videos", theme::TEXT, BODY_PX);
    let tally = count(app.browse.items.len(), "video", "videos");
    fb.text_ttf((row.x + 14) as i32, (row.y + 30) as i32, &tally, theme::TEXT_MUTED, BODY_PX);
    let new_btn = Rect { x: rail.x, y: row.y + row.h + 12, w: rail.w, h: 38 };
    paint_lead_button(fb, new_btn, icon::ui::plus, "New Playlist", Tone::Quiet);

    let right_x = rail.x + rail.w + GAP;
    let right_w = body.w.saturating_sub(rail.w + GAP);
    section::paint_head(fb, right_x, body.y, right_w, "Queue", "");
    let qy = body.y + section::HEAD_H;
    let queue = Rect { x: right_x, y: qy, w: right_w, h: body.h.saturating_sub(section::HEAD_H) };
    if app.browse.items.is_empty() {
        let note = "Open a video from your library to build a queue";
        paint_empty(fb, queue, icon::nav::playlist, "The queue is empty", note);
        return;
    }

    let w = queue.w.saturating_sub(12);
    table::paint_head(fb, queue.x, queue.y, w, &QUEUE_COLS);
    let ry = queue.y + table::HEAD_H + 4;
    let rows = Rect { x: queue.x, y: ry, w, h: queue.h.saturating_sub(table::HEAD_H + 4) };
    let visible = table::rows_visible(rows.h);
    for i in 0..visible {
        let index = app.browse.scroll + i;
        let Some(item) = app.browse.get(index) else { break };
        let r = table::row_rect(rows, i);
        table::paint_row_bg(fb, r, index == app.browse.sel);
        let ink = if index == app.browse.sel { theme::TEXT } else { theme::TEXT_DIM };
        let n = format!("{}", index + 1);
        table::paint_cell(fb, r, &QUEUE_COLS, 0, &n, ink);
        table::paint_cell(fb, r, &QUEUE_COLS, 1, item.title(), theme::TEXT);
        table::paint_cell(fb, r, &QUEUE_COLS, 2, &duration(item.duration_ms), ink);
    }
    let total = app.browse.len();
    scrollbar::paint_scrollbar(fb, scrollbar::track(queue), app.browse.scroll, visible, total);
}
