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

use crate::snake::input::hover::{self, Tag};
use crate::snake::state::Game;
use crate::snake::ui::button::{self, Style};
use crate::snake::ui::card;
use crate::snake::ui::metrics::RADIUS_PANEL;
use crate::snake::ui::rank_geom::{awards, back, table};

use super::{rank_awards, rank_rows};

const BACK: &[u8] = b"Back";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    card::panel(fb, table(w, h), RADIUS_PANEL);
    card::panel(fb, awards(w, h), RADIUS_PANEL);
    rank_rows::paint(game, fb);
    rank_awards::paint(game, fb);
    button::paint(fb, back(w, h), BACK, Style::Ghost, hover::is(Tag::RankBack, 0));
}
