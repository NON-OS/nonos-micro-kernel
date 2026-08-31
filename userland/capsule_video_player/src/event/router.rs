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

use super::action::Action;
use super::hit;
use super::key::{from_key, from_library_key};
use super::pointer::from_click;
use crate::app::state::VideoApp;
use crate::ui::frame::region;
use crate::ui::layout::{layout, Rect};
use crate::ui::screen::Route;
use crate::ui::view::{details, prefs_geom};
use crate::ui::widget::tabs::tab_hit;

fn settings_click(app: &VideoApp, body: Rect, x: i32, y: i32) -> Action {
    if let Some(index) = prefs_geom::section_at(body, x, y) {
        return Action::SetSection(index);
    }
    if let Some(index) = prefs_geom::toggle_at(body, app.prefs.section, x, y) {
        return Action::TogglePref(index);
    }
    if prefs_geom::reset_button(body).contains(x, y) {
        return Action::ResetPrefs;
    }
    if prefs_geom::cancel_button(body).contains(x, y) {
        return Action::Back;
    }
    Action::None
}

fn chrome_click(app: &VideoApp, x: i32, y: i32) -> Action {
    let (w, h) = app.dims;
    if let Some(route) = hit::nav(w, x, y) {
        return Action::Goto(route);
    }
    let body = region::body(w, h);
    if app.route() == Route::Settings {
        return settings_click(app, body, x, y);
    }
    if app.route() == Route::Details {
        return match tab_hit(body.x, body.y, &details::TABS, x, y) {
            Some(0) => Action::Goto(Route::Player),
            _ => Action::None,
        };
    }
    if let Some(grid) = hit::view_mode(w, h, x, y) {
        if grid != app.browse.grid {
            return Action::ToggleGrid;
        }
        return Action::None;
    }
    match hit::item(&app.browse, body, x, y) {
        Some(index) => Action::OpenIndex(index),
        None => Action::None,
    }
}

fn action_for(app: &VideoApp, event: InputEvent) -> Action {
    let chrome = app.route() != Route::Player;
    match event.kind {
        InputKind::KeyDown if chrome => from_library_key(event.code),
        InputKind::KeyDown => from_key(event.code),
        InputKind::ButtonDown if chrome => chrome_click(app, event.x, event.y),
        InputKind::ButtonDown => {
            let l = layout(app.dims.0, app.dims.1);
            from_click(&l, app.dims.0, event.x, event.y)
        }
        InputKind::Wheel if chrome => Action::MoveSel(-event.delta_y.signum()),
        _ => Action::None,
    }
}

pub fn on_event(app: &mut VideoApp, event: InputEvent) -> EventOutcome {
    let action = action_for(app, event);
    super::apply::apply(app, action)
}
