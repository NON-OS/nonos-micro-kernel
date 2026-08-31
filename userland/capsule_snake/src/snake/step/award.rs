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

use crate::snake::state::{level, Game};

// Ordinals into `paint::rank_awards::LABELS` and the ids `awards.dat` stores.
// The two tables are hand-synced: a new award needs an entry in both.
const FIRST_HUNDRED: u16 = 0;
const THOUSAND_CLUB: u16 = 1;
const FULL_LATTICE: u16 = 2;
const NO_WALLS_NEEDED: u16 = 3;
const NINETY_SECONDS: u16 = 4;
const DEEP_STACK: u16 = 5;

pub const COUNT: u16 = 6;

// `level::TABLE` names index 2 "The Lattice" and index 4 "Deep Stack"; the two
// level awards are those names, so they move if the level table is reordered.
const LATTICE_LEVEL: usize = 2;
const DEEP_STACK_LEVEL: usize = 4;
const NINETY_SECONDS_MS: i64 = 90_000;
const NEON_GRID_LEVEL: usize = 1;

fn earned(game: &Game, id: u16) -> bool {
    match id {
        FIRST_HUNDRED => game.score >= 100,
        THOUSAND_CLUB => game.score >= 1_000,
        FULL_LATTICE => game.level >= LATTICE_LEVEL,
        NO_WALLS_NEEDED => !game.opts.obstacles && game.score >= level::threshold(NEON_GRID_LEVEL),
        NINETY_SECONDS => game.elapsed >= NINETY_SECONDS_MS,
        DEEP_STACK => game.level >= DEEP_STACK_LEVEL,
        _ => false,
    }
}

// Called from `death::finish` before the run is filed, so a fresh unlock rides
// the same store write as the score that earned it.
pub fn grant(game: &mut Game) {
    for id in 0..COUNT {
        if earned(game, id) && !game.awards.contains(&id) {
            game.awards.push(id);
        }
    }
    game.awards.sort_unstable();
}
