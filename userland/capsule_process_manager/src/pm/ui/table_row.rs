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
use nonos_toolkit::icons::{draw, IconId};

use crate::pm::critical::is_critical;
use crate::pm::state::{Row, State};
use crate::pm::theme::{ACCENT, BAND, FOREGROUND, SELECT_BG};

use super::chrome::Rect;
use super::metrics::{BODY_PX, CELL_PAD_X, ROW_H, ROW_ICON, ROW_ICON_GAP, SEL_BAR_W};
use super::table_geom::{self, Col};
use super::{table_cell, text};

// SELECT_BG carries alpha, so the selection wash has to blend: fill_rect writes
// raw and would punch the row straight through the table ground it sits on. BAND
// is opaque and is correct as a write.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect, cols: &[Col], row: &Row, slot: usize) {
    let y = r.y + table_geom::row_y(slot);
    let body_w = r.w.saturating_sub(2);
    if row.pid == state.selected_pid && state.selected_pid != 0 {
        fb.blend_rect(r.x + 1, y, body_w, ROW_H, SELECT_BG);
        fb.fill_rect(r.x + 1, y, SEL_BAR_W, ROW_H, ACCENT);
    } else if (state.scroll + slot) % 2 == 1 {
        fb.fill_rect(r.x + 1, y, body_w, ROW_H, BAND);
    }
    for col in cols {
        match col {
            Col::Name => name(fb, r, cols, row, y),
            _ => table_cell::paint(fb, r, cols, row, *col, y),
        }
    }
}

// A protected process is marked with the shared shield art and the name starts
// after it, measured against what the mark left of the cell so a long name is
// cut rather than drawn over the next column.
fn name(fb: &mut PaintBuffer, r: &Rect, cols: &[Col], row: &Row, y: u32) {
    let mut x = r.x + table_geom::col_x(cols, r.w, Col::Name);
    let mut avail = table_geom::col_w(cols, r.w, Col::Name).saturating_sub(CELL_PAD_X);
    if is_critical(row.name()) {
        draw(fb, IconId::SettingsSecurity, x, y + (ROW_H - ROW_ICON) / 2, ROW_ICON, ACCENT);
        x += ROW_ICON + ROW_ICON_GAP;
        avail = avail.saturating_sub(ROW_ICON + ROW_ICON_GAP);
    }
    let cut = text::fit(fb, row.name(), BODY_PX, avail);
    text::left(fb, x, text::centred_top(y, ROW_H, BODY_PX), cut, FOREGROUND, BODY_PX);
}
