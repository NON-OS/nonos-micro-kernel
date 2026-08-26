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
use nonos_toolkit::font::ttf::line_height;

use crate::pm::format::{state_label, u32_decimal};
use crate::pm::state::{Row, State};
use crate::pm::theme::{MUTED, SIDEBAR_BG, SIDEBAR_LINE, TITLE};

use super::insp_geom::{content_w, content_x, pane_x};
use super::metrics::{BODY_PX, INSPECTOR_W, INSP_SECTION_GAP, PANE_PAD_TOP, TITLE_PX};
use super::tint::state_tint;
use super::{insp_actions, insp_chips, insp_fields, insp_spark, text};

// The docked detail pane: identity, the numbers behind the selected row, its
// trend, its full grant list, and the two actions. It owns the whole window
// height, so it paints its own ground instead of sitting inside a content rect.
pub fn paint(state: &State, fb: &mut PaintBuffer) {
    let x = pane_x(fb.width);
    fb.fill_rect(x, 0, INSPECTOR_W, fb.height, SIDEBAR_BG);
    fb.fill_rect(x, 0, 1, fb.height, SIDEBAR_LINE);
    let Some(row) = state.selected_row() else {
        return empty(fb, x);
    };
    let (left, w) = (content_x(fb.width), content_w());
    let mut y = heading(fb, left, PANE_PAD_TOP, w, row);
    y = insp_fields::block(fb, left, y, row, state.total_mem_kb) + INSP_SECTION_GAP;
    y = insp_spark::paint(fb, left, y, w, state.history.get(row.pid));
    let mut buf = [0u8; 12];
    let n = u32_decimal(row.caps.count_ones(), &mut buf);
    y = insp_fields::field(fb, left, y, b"Authority", &buf[..n], TITLE);
    insp_chips::paint(fb, left, y, w, row.caps);
    insp_actions::paint(fb);
}

fn heading(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, row: &Row) -> u32 {
    let cut = text::fit(fb, row.name(), TITLE_PX, w);
    text::left(fb, x, y, cut, TITLE, TITLE_PX);
    let next = y + line_height(TITLE_PX).max(1) as u32;
    text::left(fb, x, next, state_label(row.state), state_tint(row.state), BODY_PX);
    next + line_height(BODY_PX).max(1) as u32 + INSP_SECTION_GAP
}

fn empty(fb: &mut PaintBuffer, x: u32) {
    let label = b"No process selected";
    let top = text::centred_top(0, fb.height, BODY_PX);
    let cx = x + INSPECTOR_W.saturating_sub(text::width(fb, label, BODY_PX)) / 2;
    text::left(fb, cx, top, label, MUTED, BODY_PX);
}
