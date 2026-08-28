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

//! The bottom status bar: the active document's message on the left and its
//! caret position and language on the right. While the find bar is open it
//! shows the query and match count instead.

use alloc::format;

use nonos_app_skeleton::PaintBuffer;

use super::canvas::page_index;
use super::language::language_name;
use super::mode::Mode;
use super::layout::{FOOTER_H, PAD_X, STATUS_PX};
use super::paint::caret_position;
use super::state::State;
use super::theme;
use crate::doc::counts::word_count;

pub(super) fn paint_status(fb: &mut PaintBuffer, doc: &State, width: u32, height: u32) {
    let th = theme::active();
    let fy = height.saturating_sub(FOOTER_H);
    fb.fill_rect(0, fy, width, FOOTER_H, th.header_bg);
    fb.fill_rect(0, fy, width, 1, th.line);
    let ty = (fy + 6) as i32;

    if doc.find_active {
        // Find query, then the replacement field when replace mode is on; the
        // caret bar sits in whichever field is being edited.
        let mut x = PAD_X as i32;
        let _ = fb.text_ttf(x, ty, "Find: ", th.accent, STATUS_PX);
        x += fb.measure_ttf("Find: ", STATUS_PX);
        let _ = fb.text_ttf(x, ty, &doc.find_buf, th.foreground, STATUS_PX);
        x += fb.measure_ttf(&doc.find_buf, STATUS_PX);
        if !doc.replace_active {
            let _ = fb.text_ttf(x, ty, "|", th.accent, STATUS_PX);
        }
        if doc.replace_active || !doc.replace_buf.is_empty() {
            let _ = fb.text_ttf(x + 8, ty, "Replace: ", th.accent, STATUS_PX);
            x += 8 + fb.measure_ttf("Replace: ", STATUS_PX);
            let _ = fb.text_ttf(x, ty, &doc.replace_buf, th.foreground, STATUS_PX);
            x += fb.measure_ttf(&doc.replace_buf, STATUS_PX);
            if doc.replace_active {
                let _ = fb.text_ttf(x, ty, "|", th.accent, STATUS_PX);
            }
        }
        let info = format!("{} matches", doc.find_count());
        let iw = fb.measure_ttf(&info, STATUS_PX).max(0) as u32;
        let _ =
            fb.text_ttf(width.saturating_sub(PAD_X + iw) as i32, ty, &info, th.muted, STATUS_PX);
        return;
    }

    let status = core::str::from_utf8(doc.status).unwrap_or("");
    let _ = fb.text_ttf(PAD_X as i32, ty, status, th.muted, STATUS_PX);

    // While a path prompt is open, show what is being typed. It edits its own
    // buffer now, so the document's path on the right would not reflect it.
    if doc.prompt.is_some() {
        let typed = core::str::from_utf8(&doc.prompt_path[..doc.prompt_len]).unwrap_or("");
        let sw = fb.measure_ttf(status, STATUS_PX).max(0) as u32;
        let x = PAD_X + sw + 12;
        let _ = fb.text_ttf(x as i32, ty, typed, th.foreground, STATUS_PX);
        let tw = fb.measure_ttf(typed, STATUS_PX).max(0) as u32;
        let _ = fb.text_ttf((x + tw) as i32, ty, "|", th.accent, STATUS_PX);
        return;
    }

    if doc.mode == Mode::Document {
        let info = format!(
            "Page {} of {}    {} words    UTF-8",
            page_index(doc) + 1,
            doc.pages.len().max(1),
            word_count(&doc.doc)
        );
        let iw = fb.measure_ttf(&info, STATUS_PX).max(0) as u32;
        let _ =
            fb.text_ttf(width.saturating_sub(PAD_X + iw) as i32, ty, &info, th.muted, STATUS_PX);
        return;
    }

    let (line, col) = caret_position(doc);
    let path = core::str::from_utf8(&doc.path[..doc.path_len]).unwrap_or("");
    let info = format!("Ln {}, Col {}    {}    UTF-8", line + 1, col + 1, language_name(path));
    let iw = fb.measure_ttf(&info, STATUS_PX).max(0) as u32;
    let _ = fb.text_ttf(width.saturating_sub(PAD_X + iw) as i32, ty, &info, th.muted, STATUS_PX);
}
