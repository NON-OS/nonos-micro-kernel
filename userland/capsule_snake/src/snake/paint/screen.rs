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

use crate::snake::input::surface;
use crate::snake::state::{Game, Screen};
use crate::snake::theme::BACKGROUND;

use super::{home, over, pause, play, rank, setup};

// One entry, one exit. Play, Pause and Over all sit over a live board, so the
// board screen paints first and the modal screens paint on top of it.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    surface::note(fb.width, fb.height);
    fb.fill_rect(0, 0, fb.width, fb.height, BACKGROUND);
    if game.screen.over_board() {
        play::paint(game, fb);
    }
    match game.screen {
        Screen::Home => home::paint(game, fb),
        Screen::Setup => setup::paint(game, fb),
        Screen::Pause => pause::paint(fb),
        Screen::Over => over::paint(game, fb),
        Screen::Rank => rank::paint(game, fb),
        Screen::Play => {}
    }
}
