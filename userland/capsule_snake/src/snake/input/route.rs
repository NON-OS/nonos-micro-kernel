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

use crate::snake::state::Game;

use super::{click, hover, key};

pub fn on_event(game: &mut Game, event: InputEvent) -> EventOutcome {
    match event.kind {
        InputKind::KeyDown => key::on_key(game, event.code),
        InputKind::PointerAbs => motion(game, event.x, event.y),
        InputKind::ButtonDown => click::on_click(game, event.x, event.y),
        _ => EventOutcome::Idle,
    }
}

// A repaint only when the lit shape actually changed; the pointer reports far
// more often than the chrome has anything new to say.
fn motion(game: &Game, x: i32, y: i32) -> EventOutcome {
    if hover::update(game.screen, x, y) {
        return EventOutcome::Repaint;
    }
    EventOutcome::Idle
}
