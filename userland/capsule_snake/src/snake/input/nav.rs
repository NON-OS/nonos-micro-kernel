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

use crate::snake::state::{Game, Phase, Screen};

pub fn go(game: &mut Game, screen: Screen) -> EventOutcome {
    game.screen = screen;
    EventOutcome::Repaint
}

// A run always starts from the Setup panel's own settings, so the board, the
// lives and the pace are rebuilt before the board screen is shown.
pub fn start_run(game: &mut Game) -> EventOutcome {
    game.reset(false);
    go(game, Screen::Play)
}

// Pause and Resume move the screen and the phase together. A Ready board has
// nothing to suspend, so the modal opens over it without disturbing the phase.
pub fn pause(game: &mut Game) -> EventOutcome {
    if game.phase == Phase::Running {
        game.phase = Phase::Paused;
    }
    go(game, Screen::Pause)
}

pub fn resume(game: &mut Game) -> EventOutcome {
    if game.phase == Phase::Paused {
        game.phase = Phase::Running;
    }
    go(game, Screen::Play)
}

pub fn resumable(game: &Game) -> bool {
    matches!(game.phase, Phase::Running | Phase::Paused)
}
