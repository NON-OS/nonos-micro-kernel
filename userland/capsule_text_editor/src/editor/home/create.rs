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

//! The "Create New" card. No template engine is wired up yet, so every row is
//! painted sunk and inert.

use nonos_app_skeleton::PaintBuffer;

use super::metrics::{lh, BODY, CARD_PAD, CARD_W, SUBHEAD};
use super::metrics_pane::{card_x, cols_y};
use super::palette::{dim, ACCENT, CARD_BG, LABEL, RAIL_LINE, TITLE};

const ROWS: [&str; 4] = ["Report", "Letter", "Resume", "Project Plan"];
const ROW_GAP: u32 = 12;

pub(super) fn card_h() -> u32 {
    let rows = ROWS.len() as u32 * (lh(BODY) + ROW_GAP);
    CARD_PAD * 2 + lh(SUBHEAD) + ROW_GAP + rows + 10 + lh(BODY)
}

pub(super) fn paint_create(fb: &mut PaintBuffer) {
    let x = card_x(fb.width);
    let y = cols_y(fb.width);
    fb.panel(x, y, CARD_W, card_h(), 14, CARD_BG, RAIL_LINE);
    let tx = (x + CARD_PAD) as i32;
    let mut ty = y + CARD_PAD;
    let _ = fb.text_ttf(tx, ty as i32, "Create New", TITLE, SUBHEAD);
    ty += lh(SUBHEAD) + ROW_GAP;
    for row in ROWS.iter() {
        let _ = fb.text_ttf(tx, ty as i32, row, dim(LABEL), BODY);
        ty += lh(BODY) + ROW_GAP;
    }
    ty += 10;
    let _ = fb.text_ttf(tx, ty as i32, "More templates →", dim(ACCENT), BODY);
}
