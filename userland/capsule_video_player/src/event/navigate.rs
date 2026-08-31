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
use crate::ui::screen::Route;

const MAX_VOLUME: i32 = 100;

pub fn navigate(app: &mut VideoApp, action: Action) -> Option<EventOutcome> {
    match action {
        Action::ShowLibrary => Some(goto(app, Route::Library)),
        Action::Goto(route) => Some(goto(app, route)),
        Action::Back => Some(outcome(app.nav.back())),
        Action::MoveSel(delta) => Some(move_sel(app, delta)),
        Action::OpenSelected => Some(open(app, app.browse.sel)),
        Action::OpenIndex(index) => Some(open(app, index)),
        Action::ToggleGrid => {
            app.browse.grid = !app.browse.grid;
            Some(EventOutcome::Repaint)
        }
        Action::VolumeBy(delta) => Some(set_volume(app, app.volume as i32 + delta)),
        Action::SetVolume(level) => Some(set_volume(app, level as i32)),
        Action::ToggleMute => {
            app.muted = !app.muted;
            Some(EventOutcome::Repaint)
        }
        Action::TogglePref(index) => Some(outcome(app.prefs.toggle(index))),
        Action::SetSection(index) => Some(set_section(app, index)),
        Action::ResetPrefs => {
            app.prefs.reset();
            Some(EventOutcome::Repaint)
        }
        _ => None,
    }
}

fn goto(app: &mut VideoApp, route: Route) -> EventOutcome {
    if route != Route::Player {
        app.playing = false;
    }
    outcome(app.nav.go(route))
}

fn set_volume(app: &mut VideoApp, level: i32) -> EventOutcome {
    let level = level.clamp(0, MAX_VOLUME) as u32;
    if app.volume == level {
        return EventOutcome::Idle;
    }
    app.volume = level;
    EventOutcome::Repaint
}

fn set_section(app: &mut VideoApp, index: usize) -> EventOutcome {
    if index >= crate::app::prefs::SECTIONS.len() || app.prefs.section == index {
        return EventOutcome::Idle;
    }
    app.prefs.section = index;
    EventOutcome::Repaint
}

fn move_sel(app: &mut VideoApp, delta: i32) -> EventOutcome {
    let count = app.browse.len();
    if count == 0 {
        return EventOutcome::Idle;
    }
    let sel = (app.browse.sel as i64 + delta as i64).clamp(0, count as i64 - 1) as usize;
    app.browse.sel = sel;
    app.browse.scroll = scroll_for(sel, app.browse.scroll, app.dims.1);
    EventOutcome::Repaint
}

fn open(app: &mut VideoApp, index: usize) -> EventOutcome {
    outcome(app.open_index(index))
}

fn outcome(changed: bool) -> EventOutcome {
    if changed {
        EventOutcome::Repaint
    } else {
        EventOutcome::Idle
    }
}
