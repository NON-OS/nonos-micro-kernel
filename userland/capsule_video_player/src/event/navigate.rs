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

use nonos_app_skeleton::app::EventOutcome;

use super::action::Action;
use crate::app::VideoApp;
use crate::ui::rows::scroll_for;
use crate::ui::screen::Screen;

pub fn navigate(app: &mut VideoApp, action: Action) -> Option<EventOutcome> {
    match action {
        Action::ShowLibrary => {
            app.screen = Screen::Library;
            app.playing = false;
            Some(EventOutcome::Repaint)
        }
        Action::MoveSel(delta) => Some(move_sel(app, delta)),
        Action::OpenSelected => Some(open(app, app.sel)),
        Action::OpenIndex(index) => Some(open(app, index)),
        _ => None,
    }
}

fn move_sel(app: &mut VideoApp, delta: i32) -> EventOutcome {
    let count = app.items.len();
    if count == 0 {
        return EventOutcome::Idle;
    }
    let sel = (app.sel as i64 + delta as i64).clamp(0, count as i64 - 1) as usize;
    app.sel = sel;
    app.scroll = scroll_for(sel, app.scroll, app.dims.1);
    EventOutcome::Repaint
}

fn open(app: &mut VideoApp, index: usize) -> EventOutcome {
    if app.open_index(index) {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
