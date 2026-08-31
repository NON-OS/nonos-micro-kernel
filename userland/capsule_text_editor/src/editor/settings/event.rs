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

//! Input routing for the Settings screen. Presses land on the rail rows, on the
//! four General switches, and on the switches of the selected section; the
//! dropdowns are drawn dimmed, so a click that misses a live control stays idle.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::super::app::Editor;
use super::card::{control_box, DROP_VALUES, ROWS, TOGGLE_H, TOGGLE_W};
use super::geom::{nav_rect, NAV_LABELS, NAV_PX};
use super::sect::section;
use super::sect_event::section_press;
use super::state::{flip_switch, select_nav, state, width};
use crate::editor::widget::{navlist_hit, toggle_hit};

pub(crate) fn settings_event(_ed: &mut Editor, event: InputEvent) -> EventOutcome {
    if !matches!(event.kind, InputKind::ButtonDown) {
        return EventOutcome::Idle;
    }
    if let Some(i) = navlist_hit(nav_rect(), NAV_LABELS.len(), NAV_PX, event.x, event.y) {
        return if select_nav(i) { EventOutcome::Repaint } else { EventOutcome::Idle };
    }
    let nav = state().nav;
    if nav == 0 {
        return switch_press(event.x, event.y);
    }
    match section(nav) {
        Some(sec) => section_press(nav, sec, event.x, event.y),
        None => EventOutcome::Idle,
    }
}

fn switch_press(mx: i32, my: i32) -> EventOutcome {
    let w = width();
    if w == 0 {
        return EventOutcome::Idle;
    }
    for row in DROP_VALUES.len()..ROWS {
        let rect = control_box(w, row, TOGGLE_W, TOGGLE_H);
        if toggle_hit(rect, mx, my) {
            flip_switch(row - DROP_VALUES.len());
            return EventOutcome::Repaint;
        }
    }
    EventOutcome::Idle
}
