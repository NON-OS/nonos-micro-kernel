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

use crate::snake::state::{level, Game};
use crate::snake::theme::{ACCENT, LABEL, MUTED, PANEL_BORDER, RAIL_BG};
use crate::snake::ui::bar;
use crate::snake::ui::metrics::{GAP_TIGHT, PX_BODY, PX_LABEL, RADIUS_PANEL};
use crate::snake::ui::play_geom::rail;
use crate::snake::ui::play_geom_rows::{rail_row, RAIL_HEADS};
use crate::snake::ui::rect::Rect;
use crate::snake::ui::text;

use super::play_rail_tip;

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let r = rail(w, h);
    fb.fill_round(r.0, r.1, r.2, r.3, RADIUS_PANEL, RAIL_BG);
    fb.stroke_round(r.0, r.1, r.2, r.3, RADIUS_PANEL, 1, PANEL_BORDER);
    fact(fb, rail_row(w, h, 0), RAIL_HEADS[0], game.mode.name());
    fact(fb, rail_row(w, h, 1), RAIL_HEADS[1], level::name(game.level));
    progress(game, fb, rail_row(w, h, 2));
    play_rail_tip::paint(game, fb);
}

pub fn fact(fb: &mut PaintBuffer, r: Rect, head: &[u8], value: &[u8]) {
    text::left(fb, r.0, text::centred_top(r.1, r.3, PX_LABEL), head, MUTED, PX_LABEL);
    let cut = text::fit(value, PX_BODY, r.2 / 2);
    text::right(fb, r.0 + r.2, text::centred_top(r.1, r.3, PX_BODY), cut, LABEL, PX_BODY);
}

// Progress is the score climbing between this level's threshold and the next,
// and the last level has nowhere left to climb, so the bar reads full.
fn progress(game: &Game, fb: &mut PaintBuffer, r: Rect) {
    text::left(fb, r.0, text::centred_top(r.1, r.3, PX_LABEL), RAIL_HEADS[2], MUTED, PX_LABEL);
    let width = r.2 / 2;
    let track = (r.0 + r.2 - width, r.1 + r.3.saturating_sub(GAP_TIGHT) / 2, width, GAP_TIGHT);
    let base = level::threshold(game.level);
    let (num, den) = match level::next_threshold(game.level) {
        Some(next) => (game.score.saturating_sub(base) as u64, next.saturating_sub(base) as u64),
        None => (1, 1),
    };
    bar::meter(fb, track, num, den, ACCENT);
}
