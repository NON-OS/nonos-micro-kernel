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

use crate::app::state::VideoApp;
use crate::catalog::entry::parent_dir;
use crate::ui::fit::{fit, right_x};
use crate::ui::format::{duration, resolution, size};
use crate::ui::icon;
use crate::ui::layout::Rect;
use crate::ui::text::{BODY_PX, HEAD_PX};
use crate::ui::theme;
use crate::ui::widget::chip::paint_row;
use crate::ui::widget::empty::paint_empty;
use crate::ui::widget::poster::paint_poster;
use crate::ui::widget::progress::paint_bar;
use crate::ui::widget::section::paint_label;
use crate::ui::widget::tabs::{paint_tabs, TABS_H};

pub const TABS: [&str; 2] = ["Now Playing", "Media Details"];

const POSTER_W: u32 = 260;
const POSTER_H: u32 = 150;
const ROW_H: u32 = 30;

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, outer: Rect) {
    paint_tabs(fb, outer.x, outer.y, outer.w, &TABS, 1);
    let body = Rect { y: outer.y + TABS_H + 12, h: outer.h.saturating_sub(TABS_H + 12), ..outer };
    let item = match app.browse.selected() {
        Some(item) => item,
        None => {
            paint_empty(
                fb,
                body,
                icon::ui::info,
                "Nothing selected",
                "Pick a video in your library to see its details",
            );
            return;
        }
    };

    let poster = Rect { x: body.x, y: body.y, w: POSTER_W, h: POSTER_H };
    paint_poster(fb, poster, item.kind);
    let bar = Rect { x: body.x, y: poster.y + poster.h + 14, w: POSTER_W, h: 6 };
    paint_bar(fb, bar, item.permille(), theme::ACCENT);
    let resume = duration(item.resume_ms);
    let total = duration(item.duration_ms);
    let line_y = (bar.y + bar.h + 10) as i32;
    fb.text_ttf(body.x as i32, line_y, &resume, theme::TEXT_DIM, BODY_PX);
    let rx = right_x(&total, body.x + POSTER_W, BODY_PX);
    fb.text_ttf(rx, line_y, &total, theme::TEXT_MUTED, BODY_PX);

    let col_x = body.x + POSTER_W + 24;
    let col_w = (body.x + body.w).saturating_sub(col_x);
    fb.text_ttf(col_x as i32, body.y as i32, item.title(), theme::TEXT, HEAD_PX);
    let tags_y = body.y + 34;
    let kind_label = item.kind.label();
    let state_label = if item.decodable() { "Playable" } else { "Not decodable" };
    let after_chips = paint_row(fb, col_x, tags_y, col_w, &[kind_label, state_label]);

    let info_y = after_chips + 20;
    paint_label(fb, col_x, info_y, "FILE INFORMATION");
    let size_text = size(item.size);
    let resolution_text = resolution(item.width, item.height);
    let rows: [(&str, &str); 4] = [
        ("Name", &item.name),
        ("Location", parent_dir(&item.path)),
        ("Size", &size_text),
        ("Resolution", &resolution_text),
    ];
    let value_x = col_x + 150;
    let value_w = (col_x + col_w).saturating_sub(value_x);
    for (i, (key, value)) in rows.iter().enumerate() {
        let y = (info_y + 26 + i as u32 * ROW_H) as i32;
        fb.text_ttf(col_x as i32, y, key, theme::LABEL, BODY_PX);
        fb.text_ttf(value_x as i32, y, fit(value, value_w, BODY_PX), theme::TEXT, BODY_PX);
    }
}
