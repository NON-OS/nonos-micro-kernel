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

use super::{key_screen, steer};

pub const KEY_SPACE: u32 = 0x20;
pub const KEY_P_LOWER: u32 = 0x70;
pub const KEY_P_UPPER: u32 = 0x50;

// Steering is the board's own vocabulary and is offered to the board screen
// alone; every other screen sees WASD as an ordinary unbound key.
pub fn on_key(game: &mut Game, code: u32) -> EventOutcome {
    if game.screen == Screen::Play {
        if let Some(dir) = steer::direction(code) {
            return steer::steer(game, dir);
        }
    }
    match game.screen {
        Screen::Home => key_screen::home(game, code),
        Screen::Setup => key_screen::setup(game, code),
        Screen::Play => key_screen::play(game, code),
        Screen::Pause => key_screen::pause(game, code),
        Screen::Over => key_screen::over(game, code),
        Screen::Rank => key_screen::rank(game, code),
    }
}
