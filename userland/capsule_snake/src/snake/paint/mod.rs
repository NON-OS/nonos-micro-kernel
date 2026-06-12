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

mod board;
mod header;
mod overlay;

use nonos_app_skeleton::PaintBuffer;

use super::state::{Game, Phase};

const BG: u32 = 0xFF10_1418;

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    fb.clear(BG);
    header::paint(game, fb);
    board::paint(game, fb);
    match game.phase {
        Phase::Ready => overlay::ready(fb),
        Phase::Paused => overlay::paused(fb),
        Phase::GameOver => overlay::game_over(fb),
        Phase::Running => {}
    }
}
