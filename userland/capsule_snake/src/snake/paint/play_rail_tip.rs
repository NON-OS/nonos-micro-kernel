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
use crate::snake::theme::{MUTED, OK};
use crate::snake::ui::bar;
use crate::snake::ui::metrics::{GAP_TIGHT, PX_BODY, PX_LABEL};
use crate::snake::ui::play_geom_rows::{rail_row, RAIL_HEADS};
use crate::snake::ui::text;

use super::num;
use super::play_rail::fact;
use super::wrap;

const POWER_HEAD: &[u8] = b"Power";
const LIVES_HEAD: &[u8] = b"Lives";

// Rows past the four heads are the same `rail_row` band the geom already
// defines, so the rail keeps one origin no matter how many facts it carries.
pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let head = rail_row(w, h, 3);
    text::left(
        fb,
        head.0,
        text::centred_top(head.1, head.3, PX_LABEL),
        RAIL_HEADS[3],
        MUTED,
        PX_LABEL,
    );
    let (first, rest) = wrap::split(game.mode.blurb(), PX_BODY, head.2);
    blurb(fb, w, h, 4, first);
    blurb(fb, w, h, 5, text::fit(rest, PX_BODY, head.2));
    if game.power_active() {
        let left = num::dec((game.power_left() / 1000) as u32 + 1);
        fact(fb, rail_row(w, h, 6), POWER_HEAD, left.as_bytes());
    }
    lives(game, fb, w, h);
}

fn blurb(fb: &mut PaintBuffer, w: u32, h: u32, index: usize, line: &[u8]) {
    let r = rail_row(w, h, index);
    text::left(fb, r.0, text::centred_top(r.1, r.3, PX_BODY), line, MUTED, PX_BODY);
}

fn lives(game: &Game, fb: &mut PaintBuffer, w: u32, h: u32) {
    let total = game.mode.lives();
    if total < 2 {
        return;
    }
    let r = rail_row(w, h, 7);
    text::left(fb, r.0, text::centred_top(r.1, r.3, PX_LABEL), LIVES_HEAD, MUTED, PX_LABEL);
    let width = r.2 / 2;
    let track = (r.0 + r.2 - width, r.1 + r.3.saturating_sub(GAP_TIGHT) / 2, width, GAP_TIGHT);
    bar::pips(fb, track, total as usize, game.lives as usize, OK);
}
