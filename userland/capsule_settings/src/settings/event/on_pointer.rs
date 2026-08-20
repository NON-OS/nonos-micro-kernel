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

use nonos_app_skeleton::EventOutcome;

use crate::settings::state::{search_clear, searching, set_section, track_scroll, view_h, State};
use crate::settings::ui::bytes::as_str;
use crate::settings::ui::hit::{at, Hit};
use crate::settings::ui::metrics::SIDEBAR_W;
use crate::settings::ui::nav_geom;
use crate::settings::ui::results;
use crate::settings::ui::results_geom::index_at;

use super::on_search_key::open_selected;
use super::pointer_row::activate;

pub(super) fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let was_focused = core::mem::replace(&mut state.search_focused, false);
    let outcome = route(state, x, y);
    if was_focused && outcome == EventOutcome::Idle {
        return EventOutcome::Repaint;
    }
    outcome
}

fn route(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < SIDEBAR_W as i32 {
        let Some(section) = nav_geom::at(x, y) else { return EventOutcome::Idle };
        search_clear(state);
        set_section(state, section);
        return EventOutcome::Repaint;
    }
    if y >= view_h(state) as i32 {
        return EventOutcome::Idle;
    }
    if searching(state) {
        return results_click(state, y);
    }
    let pane_w = state.win_w.saturating_sub(SIDEBAR_W);
    let scroll = state.scroll_px[state.section.index()];
    match at(state, x - SIDEBAR_W as i32, y, scroll, pane_w) {
        Hit::Field { index, control } => {
            state.cursor[state.section.index()] = index;
            track_scroll(state);
            activate(state, control)
        }
        Hit::Network(i) => {
            state.wifi_cursor = i;
            EventOutcome::Repaint
        }
        Hit::None => EventOutcome::Idle,
    }
}

fn results_click(state: &mut State, y: i32) -> EventOutcome {
    let n = results::count(as_str(state.search.as_slice()));
    let Some(i) = index_at(y, state.search_scroll, n) else { return EventOutcome::Idle };
    state.search_cursor = i;
    open_selected(state)
}
