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
use nonos_toolkit::icons::draw;

use crate::snake::state::Game;
use crate::snake::theme::{LABEL, MUTED, TITLE};
use crate::snake::ui::icon_table;
use crate::snake::ui::metrics::{GAP_TIGHT, ICON_MD, PAD_TIGHT, PX_LABEL, ROW_H, TABLE_HEAD_H};
use crate::snake::ui::rank_geom::{award_row, awards, AWARD_ROWS};
use crate::snake::ui::rect;
use crate::snake::ui::text;

const HEADING: &[u8] = b"Awards";

// Ordinal against `rank_geom::AWARD_ROWS`; the id is the index, which is what
// `awards.dat` stores, so an unlocked mark survives a reload.
const LABELS: [&[u8]; AWARD_ROWS] = [
    b"First hundred",
    b"Thousand club",
    b"Full lattice",
    b"No walls needed",
    b"Ninety seconds",
    b"Deep stack",
];

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let inner = rect::inset(awards(w, h), PAD_TIGHT);
    let band = (inner.0, inner.1, inner.2, TABLE_HEAD_H);
    text::left(fb, band.0, text::centred_top(band.1, band.3, PX_LABEL), HEADING, MUTED, PX_LABEL);
    for index in 0..AWARD_ROWS {
        entry(game, fb, award_row(w, h, index), index);
    }
}

fn entry(game: &Game, fb: &mut PaintBuffer, r: rect::Rect, index: usize) {
    let unlocked = game.awards.iter().any(|id| *id as usize == index);
    let ink = if unlocked { TITLE } else { MUTED };
    let mark_y = r.1 + ROW_H.saturating_sub(ICON_MD) / 2;
    draw(fb, icon_table::award(unlocked), r.0, mark_y, ICON_MD, ink);
    let x = r.0 + ICON_MD + GAP_TIGHT;
    let cut = text::fit(LABELS[index], PX_LABEL, r.2.saturating_sub(ICON_MD + GAP_TIGHT));
    let body = if unlocked { LABEL } else { MUTED };
    text::left(fb, x, text::centred_top(r.1, r.3, PX_LABEL), cut, body, PX_LABEL);
}
