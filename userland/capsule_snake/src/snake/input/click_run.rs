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

use crate::snake::state::{Game, Screen};

use super::nav;

// Footer: Pause, Restart, Sound, Quit. Sound has no mixer route from a
// windowed app and no state to carry, so index 2 is deliberately unclaimed.
pub fn foot(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => nav::pause(game),
        1 => restart(game),
        3 => nav::go(game, Screen::Home),
        _ => EventOutcome::Idle,
    }
}

pub fn pause_action(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => nav::resume(game),
        1 => restart(game),
        2 => nav::go(game, Screen::Setup),
        _ => nav::go(game, Screen::Home),
    }
}

pub fn over_action(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => restart(game),
        1 => nav::go(game, Screen::Rank),
        _ => nav::go(game, Screen::Home),
    }
}

pub fn restart(game: &mut Game) -> EventOutcome {
    game.reset(false);
    nav::go(game, Screen::Play)
}
