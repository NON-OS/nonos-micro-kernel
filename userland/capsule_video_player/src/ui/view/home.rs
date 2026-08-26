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

use super::grid::{paint_grid, paint_list};
use crate::app::state::VideoApp;
use crate::ui::icon;
use crate::ui::layout::Rect;
use crate::ui::widget::empty::{paint_empty, paint_inline};
use crate::ui::widget::section::paint_head;

const HEAD: u32 = 34;
const RESUME_H: u32 = 202;

fn resumable(app: &VideoApp) -> usize {
    app.browse.items.iter().filter(|m| m.permille() > 0).count()
}

pub fn paint(fb: &mut PaintBuffer, app: &VideoApp, body: Rect) {
    if app.browse.items.is_empty() {
        paint_empty(
            fb,
            body,
            icon::nav::video,
            "No videos yet",
            "Add .mp4, .mkv, .avi or .mov files to see them here",
        );
        return;
    }
    paint_head(fb, body.x, body.y, body.w, "Continue Watching", "");
    let strip = Rect { x: body.x, y: body.y + HEAD, w: body.w, h: RESUME_H };
    if resumable(app) == 0 {
        paint_inline(fb, strip, "Nothing in progress - press Enter on a video to start watching");
    } else {
        paint_grid(fb, &app.browse, strip);
    }
    let rest_y = strip.y + strip.h + 22;
    paint_head(fb, body.x, rest_y, body.w, "Recent", "");
    let rest = Rect {
        x: body.x,
        y: rest_y + HEAD,
        w: body.w,
        h: (body.y + body.h).saturating_sub(rest_y + HEAD),
    };
    if app.browse.grid {
        paint_grid(fb, &app.browse, rest);
    } else {
        paint_list(fb, &app.browse, rest);
    }
}
