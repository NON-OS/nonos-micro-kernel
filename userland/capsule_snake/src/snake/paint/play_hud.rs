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

use crate::snake::state::{level, Game, Mode};
use crate::snake::ui::card;
use crate::snake::ui::icon_table;
use crate::snake::ui::play_geom_rows::hud;

use super::num;
use super::num_clock::clock;

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let score = num::dec(game.score);
    card::stat(fb, hud(w, h, 0), icon_table::hud(0), b"Score", score.as_bytes(), game.mode.name());
    let length = num::dec(game.body.len() as u32);
    let mark = icon_table::hud(1);
    card::stat(fb, hud(w, h, 1), mark, b"Length", length.as_bytes(), b"segments");
    let step = num::dec(game.level as u32 + 1);
    let mark = icon_table::hud(2);
    card::stat(fb, hud(w, h, 2), mark, b"Level", step.as_bytes(), level::name(game.level));
    last(game, fb, w, h);
}

// The fourth card is the mode's own scarcity: lives where a crash costs one,
// the clock where the run is timed, and the food streak where neither applies.
fn last(game: &Game, fb: &mut PaintBuffer, w: u32, h: u32) {
    let r = hud(w, h, 3);
    if game.mode == Mode::TimeAttack {
        let left = clock(game.time_left());
        let mark = IconId::GameStopwatch;
        card::stat(fb, r, mark, b"Time", left.as_bytes(), b"remaining");
        return;
    }
    if game.mode.lives() > 1 {
        let lives = num::dec(game.lives as u32);
        card::stat(fb, r, icon_table::hud(3), b"Lives", lives.as_bytes(), b"remaining");
        return;
    }
    let streak = num::dec(game.streak);
    card::stat(fb, r, IconId::GameTarget, b"Food", streak.as_bytes(), b"this run");
}
