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

//! The title strip. Painted after the activity bar and tab strip so it owns the
//! full-width reserved band at the top, and it records the spans it drew.

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;

use super::items::TITLES;
use super::metrics::{text_top, title_spans, TitleSpan, TITLE_PAD};
use crate::editor::layout::{CHROME_PX, TITLEBAR_H};
use crate::editor::theme;

pub(in crate::editor) fn paint_menubar(
    fb: &mut PaintBuffer,
    width: u32,
    open: Option<usize>,
) -> Vec<TitleSpan> {
    let th = theme::active();
    let band = TITLEBAR_H.saturating_sub(1);
    fb.fill_rect(0, 0, width, band, th.tabbar_bg);
    fb.fill_rect(0, band, width, 1, th.line);

    let spans = title_spans();
    let ty = text_top(band) as i32;
    for (i, span) in spans.iter().enumerate() {
        if span.x1 > width {
            break;
        }
        let hot = open == Some(i);
        if hot {
            fb.fill_rect(span.x0, 0, span.x1 - span.x0, band, th.tab_active_bg);
        }
        let fg = if hot { th.title } else { th.foreground };
        let _ = fb.text_ttf((span.x0 + TITLE_PAD) as i32, ty, TITLES[i], fg, CHROME_PX);
    }
    spans
}
