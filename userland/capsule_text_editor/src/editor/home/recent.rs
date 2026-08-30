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

//! The recent-documents column. There is no document store behind these rows
//! yet, so the whole list and its "View all" link are painted sunk and a click
//! on them opens nothing.

use nonos_app_skeleton::PaintBuffer;

use crate::editor::widget::{paint_docrow, DocRowStyle};

use super::metrics::{lh, BODY, DOC_ICON, SUBHEAD};
use super::metrics_pane::{cols_y, doc_row_h, docs_rect};
use super::palette::{dim, ACCENT, ICON_BG, LABEL, MUTED, TITLE};

pub(super) const DOCS: [(&str, &str); 5] = [
    ("Project Proposal.docx", "Edited 2m ago"),
    ("System Architecture.docx", "Edited 1h ago"),
    ("Meeting Notes 2026-05-24.docx", "Edited 5h ago"),
    ("Research Paper Draft.docx", "Edited yesterday"),
    ("NØNOS Roadmap.docx", "Edited 3 days ago"),
];

pub(super) fn paint_recent(fb: &mut PaintBuffer) {
    let (x, y, w) = docs_rect(fb.width);
    let head_y = cols_y(fb.width);
    let _ = fb.text_ttf(x as i32, head_y as i32, "Recent Documents", TITLE, SUBHEAD);
    paint_view_all(fb, x, head_y, w);
    let st = DocRowStyle {
        icon_bg: dim(ICON_BG),
        icon_mark: dim(LABEL),
        icon_radius: 6,
        title: dim(TITLE),
        subtitle: dim(MUTED),
        gap: 14,
    };
    let rh = doc_row_h();
    for (i, (title, sub)) in DOCS.iter().enumerate() {
        let rect = (x, y + i as u32 * rh, w, rh);
        paint_docrow(fb, rect, DOC_ICON, (title, sub), (BODY, BODY), &st);
    }
}

fn paint_view_all(fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    let label = "View all";
    let lw = fb.measure_ttf(label, BODY).max(0) as u32;
    let lx = x + w.saturating_sub(lw);
    let ly = y + lh(SUBHEAD).saturating_sub(lh(BODY)) / 2;
    let _ = fb.text_ttf(lx as i32, ly as i32, label, dim(ACCENT), BODY);
}
