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

use crate::snake::state::Game;
use crate::snake::theme::{BAND, MUTED, TITLE};
use crate::snake::ui::metrics::{PX_BODY, PX_LABEL, RADIUS_BTN};
use crate::snake::ui::over_geom_rows::{summary_row, HEADS, SUMMARY_ROWS};
use crate::snake::ui::text;

use super::num::Digits;
use super::num_clock::{clock, hex};
use super::{num, receipt};

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    for index in 0..SUMMARY_ROWS {
        let r = summary_row(w, h, index);
        if index % 2 == 1 {
            fb.fill_round(r.0, r.1, r.2, r.3, RADIUS_BTN, BAND);
        }
        let head = text::fit(HEADS[index], PX_LABEL, r.2 / 2);
        text::left(fb, r.0, text::centred_top(r.1, r.3, PX_LABEL), head, MUTED, PX_LABEL);
        let value = value(game, index);
        let top = text::centred_top(r.1, r.3, PX_BODY);
        text::mono_right(fb, r.0 + r.2, top, value.as_bytes(), TITLE, PX_BODY);
    }
}

// Ordinal against `over_geom_rows::HEADS`. The receipt is the run's own short
// hash, which is what this OS hands back instead of a coin balance.
fn value(game: &Game, index: usize) -> Digits {
    match index {
        0 => num::dec(game.score),
        1 => num::dec(game.body.len() as u32),
        2 => num::dec(game.level as u32 + 1),
        3 => num::dec(game.streak),
        4 => clock(game.elapsed),
        _ => hex(receipt::of(game)),
    }
}
