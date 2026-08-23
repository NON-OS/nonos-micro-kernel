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

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};

use super::state::{Screen, State, SIGKILL, SIGTERM};
use super::ui::hit::{self, Target};
use super::ui::table_geom;

#[path = "event_key.rs"]
mod event_key;
#[path = "event_scroll.rs"]
mod event_scroll;

pub fn on_event(state: &mut State, event: InputEvent) -> EventOutcome {
    if event.kind == InputKind::ButtonDown {
        return click(state, event.x, event.y);
    }
    if event.kind == InputKind::Wheel {
        let delta = if event.delta_y > 0 { -3 } else { 3 };
        if state.screen == Screen::Security {
            state.alert_scroll_by(delta);
        } else {
            state.scroll_by(delta);
        }
        return EventOutcome::Repaint;
    }
    if !event.is_key_down() {
        return EventOutcome::Idle;
    }
    event_key::key(state, event.code)
}

// The whole click path: ask the shared hit test what the frame drew under the
// pointer, then act on the answer. Not one coordinate is derived here, which is
// the only reason a click cannot land on something other than what was painted.
fn click(state: &mut State, x: i32, y: i32) -> EventOutcome {
    let (w, h) = (state.fb_w, state.fb_h);
    let Some(target) = hit::at(state, w, h, x, y) else {
        return EventOutcome::Idle;
    };
    match target {
        Target::Nav(screen) => state.set_screen(screen),
        Target::Sort(col) => {
            if let Some(sort) = table_geom::sort_for(col) {
                state.set_sort(sort);
            }
        }
        Target::Row(index) | Target::Matrix(index) => state.select_visible(index),
        Target::Finding(index) => {
            state.alert_sel = index;
            state.jump_to_alert();
        }
        Target::EndProcess => state.kill_selected(SIGTERM),
        Target::ForceQuit => state.kill_selected(SIGKILL),
    }
    EventOutcome::Repaint
}
