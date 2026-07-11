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

use super::{EntryOp, SbEntry};
use crate::editor::layout::{ACTIVITY_W, CHROME_PX, ROW_H, SIDEBAR_W};
use crate::editor::shell::pane_y;
use crate::editor::theme;

// The input bar at the top of the sidebar: a short prompt for the operation,
// the name typed so far, and a caret mark.
pub(in crate::editor) fn paint_entry(fb: &mut PaintBuffer, entry: &SbEntry) {
    let th = theme::active();
    let y = pane_y();
    fb.fill_rect(ACTIVITY_W, y, SIDEBAR_W, ROW_H, th.tab_inactive_bg);
    fb.fill_rect(ACTIVITY_W, y + ROW_H - 1, SIDEBAR_W, 1, th.accent);
    let prompt = match entry.op {
        EntryOp::NewFile => "file: ",
        EntryOp::NewFolder => "dir: ",
        EntryOp::Rename => "name: ",
    };
    let px = (ACTIVITY_W + 10) as i32;
    let ty = (y + 5) as i32;
    let _ = fb.text_ttf(px, ty, prompt, th.muted, CHROME_PX);
    let pw = fb.measure_ttf(prompt, CHROME_PX);
    let _ = fb.text_ttf(px + pw, ty, &entry.buf, th.foreground, CHROME_PX);
    let bw = fb.measure_ttf(&entry.buf, CHROME_PX);
    fb.fill_rect((px + pw + bw) as u32 + 1, y + 5, 2, ROW_H - 10, th.accent);
}
