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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::icons::IconId;

use crate::snake::state::{difficulty, mode, Game};
use crate::snake::ui::card;
use crate::snake::ui::home_geom::{card as slot, CARD_LABELS};
use crate::snake::ui::icon_table;

use super::num;

const NO_RUNS: &[u8] = b"No runs stored yet";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    daily(game, fb, w, h);
    best(game, fb, w, h);
}

// The challenge is the day the capsule is running on, taken from the clock the
// game already advances rather than a fresh syscall in the paint path.
fn daily(game: &Game, fb: &mut PaintBuffer, w: u32, h: u32) {
    let day = (game.last_ms.max(0) / 86_400_000) as usize;
    let pick = mode::ALL[day % mode::ALL.len()];
    let level = difficulty::ALL[(day / mode::ALL.len()) % difficulty::ALL.len()];
    let mark = icon_table::mode(pick);
    card::stat(fb, slot(w, h, 0), mark, CARD_LABELS[0], pick.name(), level.name());
}

fn best(game: &Game, fb: &mut PaintBuffer, w: u32, h: u32) {
    let top = game.runs.iter().max_by_key(|run| run.score);
    let score = num::dec(top.map(|run| run.score).unwrap_or(0));
    let sub = top.map(|run| run.mode.name()).unwrap_or(NO_RUNS);
    let mark = IconId::GameTrophy;
    card::stat(fb, slot(w, h, 1), mark, CARD_LABELS[1], score.as_bytes(), sub);
}
