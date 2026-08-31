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

use crate::snake::state::{Game, RunRecord};
use crate::snake::theme::{BAND, LABEL, MUTED, RULE_SOFT, TITLE};
use crate::snake::ui::metrics::{PX_BODY, PX_LABEL, RADIUS_BTN, RANK_ROWS};
use crate::snake::ui::rank_geom::{head, row};
use crate::snake::ui::rank_geom_cols::{cell, COLUMNS, HEADS};
use crate::snake::ui::text;

use super::num;

const EMPTY: &[u8] = b"No runs stored yet";

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    let band = head(w, h);
    for index in 0..COLUMNS {
        let c = cell(band, w, h, index);
        text::left(fb, c.0, text::centred_top(c.1, c.3, PX_LABEL), HEADS[index], MUTED, PX_LABEL);
    }
    fb.blend_rect(band.0, band.1 + band.3, band.2, 1, RULE_SOFT);
    if game.runs.is_empty() {
        let r = row(w, h, 0);
        text::left(fb, r.0, text::centred_top(r.1, r.3, PX_BODY), EMPTY, MUTED, PX_BODY);
        return;
    }
    for index in 0..RANK_ROWS.min(game.runs.len()) {
        entry(fb, w, h, index, &game.runs[index]);
    }
}

// The rank number is the row's own position, so a table that lost a record to a
// short read still counts from one rather than from the stored sequence.
fn entry(fb: &mut PaintBuffer, w: u32, h: u32, index: usize, run: &RunRecord) {
    let band = row(w, h, index);
    if index % 2 == 1 {
        fb.fill_round(band.0, band.1, band.2, band.3, RADIUS_BTN, BAND);
    }
    let place = num::dec(index as u32 + 1);
    let score = num::dec(run.score);
    let length = num::dec(run.length as u32);
    let top = text::centred_top(band.1, band.3, PX_BODY);
    let ink = if index == 0 { TITLE } else { LABEL };
    let columns: [&[u8]; COLUMNS] =
        [place.as_bytes(), score.as_bytes(), run.mode.name(), length.as_bytes()];
    for column in 0..COLUMNS {
        let c = cell(band, w, h, column);
        text::left(fb, c.0, top, text::fit(columns[column], PX_BODY, c.2), ink, PX_BODY);
    }
}
