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

use crate::settings::state::{searching, view_h, State};
use crate::settings::ui::metrics::SIDEBAR_W;
use crate::settings::ui::theme::WINDOW_BG;
use crate::settings::ui::{pane, results_pane, sidebar, status_bar};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(WINDOW_BG);
    sidebar::paint(fb, state.section, state.win_h);
    let pane_w = state.win_w.saturating_sub(SIDEBAR_W);
    let pane_h = view_h(state);
    if pane_w > 0 && pane_h > 0 {
        let mut view = fb.sub(SIDEBAR_W, 0, pane_w, pane_h);
        if searching(state) {
            results_pane::paint(&mut view, state, state.search_scroll, pane_w, pane_h);
        } else {
            let scroll = state.scroll_px[state.section.index()];
            pane::paint(&mut view, state, scroll, pane_w, pane_h);
        }
    }
    status_bar::paint(fb, state);
}
