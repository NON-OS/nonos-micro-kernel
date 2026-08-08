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
use nonos_policy_proto::{kind_of, Category, KIND_I8, KIND_U8};

// The stepper draws "<" at VALUE_LEFT and ">" 64px along, so the split sits
// between them. Left of it decrements, right of it increments.
const STEP_ZONE: u32 = 41;

use crate::settings::paint::paint_tabs::tab_width;
use crate::settings::paint::{
    layout::{BODY_TOP, HEADER_H, ROW_H, STATUS_H, TAB_H, VALUE_LEFT},
    visible_rows::visible_rows,
};
use crate::settings::state::{
    current_field::current_field, focused_count::focused_count, refresh_wifi::enter_wifi,
    set_category::set_category, track_scroll::track_scroll, State,
};

use super::adjust::adjust;
use super::toggle_or_inc::toggle_or_inc;

pub(super) fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let (x, y) = (x as u32, y as u32);
    if (HEADER_H..HEADER_H + TAB_H).contains(&y) {
        // Same width the tabs are drawn at, so a widened window does not leave
        // a strip past the last tab that selects Wi-Fi.
        let tab_w = tab_width(state.win_w).max(1);
        match core::cmp::min(x / tab_w, 3) {
            0 => set_category(state, Category::User),
            1 => set_category(state, Category::Identity),
            2 => set_category(state, Category::Kernel),
            _ => enter_wifi(state),
        }
        return EventOutcome::Repaint;
    }
    // Clicking a network selects it. The panel used to drop every body click,
    // so networks could only be reached with the arrow keys plus `c`, which
    // the panel does not mention anywhere.
    if state.wifi_active {
        // While the passphrase editor is open the body is not a list.
        if state.wifi_pass_active || state.wifi_network_count == 0 {
            return EventOutcome::Idle;
        }
        // Mirrors paint_wifi: the heading row, then the adapter row, then a
        // 4px gap, then one row per network.
        let net_top = BODY_TOP + 2 * ROW_H + 4;
        if y < net_top {
            return EventOutcome::Idle;
        }
        let row = ((y - net_top) / ROW_H) as usize;
        if row >= state.wifi_network_count {
            return EventOutcome::Idle;
        }
        state.wifi_cursor = row;
        return EventOutcome::Repaint;
    }
    // Against the manifest height, a click below the starting height read as
    // the status bar and was dropped, so a taller window had a dead lower half.
    if y < BODY_TOP || y >= state.win_h.saturating_sub(STATUS_H) {
        return EventOutcome::Idle;
    }
    let row = ((y - BODY_TOP) / ROW_H) as usize;
    let cat = state.category as usize;
    let idx = state.scroll_top[cat] + row;
    if row >= visible_rows(state.win_h) || idx >= focused_count(state.category) {
        return EventOutcome::Idle;
    }
    state.cursor[cat] = idx;
    track_scroll(state);
    // Clicking the label selects the row without changing it.
    if x < VALUE_LEFT {
        return EventOutcome::Repaint;
    }
    // Dispatch on the field's kind. The zones below only mean anything for a
    // stepped value: sending a click on a switch to `adjust` did nothing at
    // all, because adjust ignores booleans, so the switch was dead to the
    // mouse and only the empty space beside it toggled the setting.
    let Some(field) = current_field(state) else {
        return EventOutcome::Repaint;
    };
    match kind_of(field) {
        KIND_U8 | KIND_I8 => {
            if x < VALUE_LEFT + STEP_ZONE {
                adjust(state, -1);
            } else {
                adjust(state, 1);
            }
        }
        // A switch or a text field has one action wherever it is clicked.
        _ => toggle_or_inc(state),
    }
    EventOutcome::Repaint
}
