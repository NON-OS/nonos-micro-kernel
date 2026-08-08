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
use nonos_policy_proto::{label_of, Field};

use crate::settings::schema::read_only;
use crate::settings::state::State;
use crate::settings::theme::{LABEL_FG, ROW_BG, ROW_BG_ALT, ROW_SELECTED_BG, STATUS_FG_IDLE};

use super::layout::{LABEL_LEFT, ROW_H};
use super::paint_field_value::paint_field_value;

pub fn paint_field_row(
    fb: &mut PaintBuffer,
    state: &State,
    field: Field,
    row_index: usize,
    y: u32,
    selected: bool,
) {
    let bg = if selected {
        ROW_SELECTED_BG
    } else if row_index.is_multiple_of(2) {
        ROW_BG
    } else {
        ROW_BG_ALT
    };
    fb.fill_rect(0, y, fb.width, ROW_H, bg);
    // A read-only row is dimmed and marked, so it does not read as a control
    // that is simply refusing to work.
    let label_fg = if read_only(field) { STATUS_FG_IDLE } else { LABEL_FG };
    fb.text(LABEL_LEFT, y + 5, label_of(field), label_fg);
    if read_only(field) {
        let w = fb.width;
        fb.text(w.saturating_sub(80), y + 5, b"(status)", STATUS_FG_IDLE);
    }
    paint_field_value(fb, state, field, y + 5, selected);
}
