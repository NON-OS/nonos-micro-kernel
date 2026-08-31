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

use crate::snake::state::run::MAX_RUNS;
use crate::snake::state::{Game, Phase, RunRecord, Screen};

// A crash spends a life and rebuilds the board around the surviving score.
pub fn crash(game: &mut Game) {
    game.lives = game.lives.saturating_sub(1);
    if game.lives == 0 {
        finish(game);
        return;
    }
    game.reset(true);
}

// The one place a run is filed: at the transition into Over, never per tick.
pub fn finish(game: &mut Game) {
    game.phase = Phase::GameOver;
    game.screen = Screen::Over;
    let seq = game.runs.len() as u32 + 1;
    let length = game.body.len() as u16;
    game.runs.push(RunRecord::new(game.score, game.mode, length, seq));
    game.runs.sort_by(|a, b| b.score.cmp(&a.score));
    game.runs.truncate(MAX_RUNS);
    super::award::grant(game);
    crate::snake::store::save_from(game);
}
