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

//! The dropdown panel for the open title, painted with the overlays so it sits
//! above the pane and the status bar.

use nonos_app_skeleton::PaintBuffer;

use super::items::{rows, MenuCmd};
use super::metrics::{panel_rect, row_h, text_top, TitleSpan, DROP_PAD_X};
use crate::editor::layout::CHROME_PX;
use crate::editor::theme;

pub(in crate::editor) fn paint_dropdown(fb: &mut PaintBuffer, spans: &[TitleSpan], open: usize) {
    let th = theme::active();
    let (x, y, w, h) = panel_rect(spans, open);
    fb.fill_rect(x, y, w, h, th.tab_inactive_bg);
    fb.fill_rect(x, y, w, 1, th.line);
    fb.fill_rect(x, y + h - 1, w, 1, th.line);
    fb.fill_rect(x, y, 1, h, th.line);
    fb.fill_rect(x + w - 1, y, 1, h, th.line);

    let rh = row_h();
    let ty = text_top(rh);
    for (i, (label, cmd)) in rows(open).iter().enumerate() {
        let top = y + 1 + i as u32 * rh;
        let fg = if *cmd == MenuCmd::Todo { th.muted } else { th.foreground };
        let _ = fb.text_ttf((x + DROP_PAD_X) as i32, (top + ty) as i32, label, fg, CHROME_PX);
    }
}
