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
use nonos_app_skeleton::input::{self, InputEvent, InputKind};

use super::action::Action;
use super::apply::apply;
use super::key::{self, from_key, from_library_key};
use super::pointer::from_click;
use crate::app::VideoApp;
use crate::ui::layout::layout;
use crate::ui::rows::row_at;
use crate::ui::screen::Screen;

const _: () = assert!(key::KEY_ENTER == input::KEY_ENTER);
const _: () = assert!(key::KEY_ESC == input::KEY_ESC);
const _: () = assert!(key::KEY_UP == input::KEY_UP);
const _: () = assert!(key::KEY_DOWN == input::KEY_DOWN);
const _: () = assert!(key::KEY_LEFT == input::KEY_LEFT);
const _: () = assert!(key::KEY_RIGHT == input::KEY_RIGHT);

pub fn on_event(app: &mut VideoApp, event: InputEvent) -> EventOutcome {
    let action = match app.screen {
        Screen::Library => library_action(app, event),
        Screen::Player => player_action(app, event),
    };
    apply(app, action)
}

fn library_action(app: &VideoApp, event: InputEvent) -> Action {
    let (w, h) = app.dims;
    match event.kind {
        InputKind::ButtonDown => {
            match row_at(w, h, app.scroll, app.items.len(), event.x, event.y) {
                Some(index) => Action::OpenIndex(index),
                None => Action::None,
            }
        }
        InputKind::KeyDown => from_library_key(event.code),
        _ => Action::None,
    }
}

fn player_action(app: &VideoApp, event: InputEvent) -> Action {
    match event.kind {
        InputKind::ButtonDown => {
            let l = layout(app.dims.0, app.dims.1);
            from_click(&l, event.x, event.y)
        }
        InputKind::KeyDown => from_key(event.code),
        _ => Action::None,
    }
}
