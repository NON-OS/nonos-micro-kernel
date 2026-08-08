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

use crate::settings::schema::visible_for;
use crate::settings::state::State;
use crate::settings::theme::BACKGROUND;

use super::layout::{BODY_TOP, ROW_H};
use super::paint_field_row::paint_field_row;
use super::paint_header::paint_header;
use super::paint_status::{paint_status, HintMode};
use super::paint_tabs::paint_tabs;
use super::paint_wifi::paint_wifi;
use super::scroll_indicator::paint_scroll_indicator;
use super::visible_rows::visible_rows;

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    paint_header(fb);
    paint_tabs(fb, state.category, state.wifi_active, state.win_w);
    if state.wifi_active {
        paint_wifi(fb, state);
        let mode = if state.wifi_pass_active { HintMode::WifiPassphrase } else { HintMode::Wifi };
        paint_status(fb, &state.status, state.policy_ready, mode);
        return;
    }
    let fields = visible_for(state.category);
    let cat = state.category as usize;
    let cursor = state.cursor[cat];
    let top = state.scroll_top[cat];
    let rows = visible_rows(state.win_h);
    let end = core::cmp::min(top + rows, fields.len());
    let mut y = BODY_TOP;
    for (i, field) in fields[top..end].iter().enumerate() {
        let abs = top + i;
        paint_field_row(fb, state, *field, abs, y, abs == cursor);
        y += ROW_H;
    }
    paint_scroll_indicator(fb, top, rows, fields.len());
    let mode = if state.editing { HintMode::Editing } else { HintMode::Browsing };
    paint_status(fb, &state.status, state.policy_ready, mode);
}
