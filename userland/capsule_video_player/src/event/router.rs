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

use super::apply::apply;
use super::key::{self, from_key};
use super::pointer::from_click;
use crate::app::VideoApp;
use crate::ui::layout::layout;

const _: () = assert!(key::KEY_ESC == input::KEY_ESC);
const _: () = assert!(key::KEY_UP == input::KEY_UP);
const _: () = assert!(key::KEY_DOWN == input::KEY_DOWN);
const _: () = assert!(key::KEY_LEFT == input::KEY_LEFT);
const _: () = assert!(key::KEY_RIGHT == input::KEY_RIGHT);

pub fn on_event(app: &mut VideoApp, event: InputEvent) -> EventOutcome {
    let l = layout(app.dims.0, app.dims.1);
    let action = match event.kind {
        InputKind::ButtonDown => from_click(&l, event.x, event.y),
        InputKind::KeyDown => from_key(event.code),
        _ => return EventOutcome::Idle,
    };
    apply(app, action)
}
