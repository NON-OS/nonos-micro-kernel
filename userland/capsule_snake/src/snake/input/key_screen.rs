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

use nonos_app_skeleton::{EventOutcome, KEY_ENTER, KEY_ESC};

use crate::snake::state::{Game, Phase, Screen};

use super::click_run::restart;
use super::key::{KEY_P_LOWER, KEY_P_UPPER, KEY_SPACE};
use super::nav;

pub fn home(game: &mut Game, code: u32) -> EventOutcome {
    match code {
        KEY_ENTER => nav::go(game, Screen::Setup),
        _ => EventOutcome::Idle,
    }
}

pub fn setup(game: &mut Game, code: u32) -> EventOutcome {
    match code {
        KEY_ENTER => nav::start_run(game),
        KEY_ESC => nav::go(game, Screen::Home),
        _ => EventOutcome::Idle,
    }
}

pub fn play(game: &mut Game, code: u32) -> EventOutcome {
    match code {
        KEY_ENTER if game.phase == Phase::GameOver => restart(game),
        KEY_SPACE | KEY_P_LOWER | KEY_P_UPPER | KEY_ESC => nav::pause(game),
        _ => EventOutcome::Idle,
    }
}

pub fn pause(game: &mut Game, code: u32) -> EventOutcome {
    match code {
        KEY_SPACE | KEY_P_LOWER | KEY_P_UPPER | KEY_ESC | KEY_ENTER => nav::resume(game),
        _ => EventOutcome::Idle,
    }
}

pub fn over(game: &mut Game, code: u32) -> EventOutcome {
    match code {
        KEY_ENTER | KEY_SPACE => restart(game),
        KEY_ESC => nav::go(game, Screen::Home),
        _ => EventOutcome::Idle,
    }
}

pub fn rank(game: &mut Game, code: u32) -> EventOutcome {
    match code {
        KEY_ENTER | KEY_ESC => nav::go(game, Screen::Home),
        _ => EventOutcome::Idle,
    }
}
