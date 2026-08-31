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

use crate::snake::state::{difficulty, mode, Game};

// Row 0 is the mode strip and row 1 the difficulty strip, the same order
// `setup_geom_rows::chip_label` reads them out of.
pub fn chip(game: &mut Game, row: usize, index: usize) -> EventOutcome {
    if row == 0 {
        game.mode = mode::ALL[index.min(mode::ALL.len() - 1)];
    } else {
        game.diff = difficulty::ALL[index.min(difficulty::ALL.len() - 1)];
    }
    EventOutcome::Repaint
}

pub fn toggle(game: &mut Game, index: usize) -> EventOutcome {
    match index {
        0 => game.opts.obstacles = !game.opts.obstacles,
        1 => game.opts.wrap = !game.opts.wrap,
        2 => game.opts.powerups = !game.opts.powerups,
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
