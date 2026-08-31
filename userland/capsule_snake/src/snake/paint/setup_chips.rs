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
use crate::snake::state::{difficulty, mode, Game};
use crate::snake::theme::{
    ACCENT, ACCENT_BORDER, ACCENT_TINT, BTN_BG, BTN_BORDER, BTN_HOVER_BG, LABEL,
};
use crate::snake::ui::metrics::{PX_LABEL, RADIUS_PILL};
use crate::snake::ui::rect::Rect;
use crate::snake::ui::setup_geom_rows::{chip, chip_label, CHIPS, CHIP_ROWS};
use crate::snake::ui::text;

pub fn paint(game: &Game, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    for row in 0..CHIP_ROWS {
        for index in 0..CHIPS {
            let on = selected(game, row, index);
            let lit = hover::is(tag(row), index);
            face(fb, chip(w, h, row, index), chip_label(row, index), on, lit);
        }
    }
}

fn tag(row: usize) -> Tag {
    if row == 0 {
        Tag::ModeChip
    } else {
        Tag::DiffChip
    }
}

fn selected(game: &Game, row: usize, index: usize) -> bool {
    if row == 0 {
        mode::ALL[index] == game.mode
    } else {
        difficulty::ALL[index] == game.diff
    }
}

// The chip is exactly its measured label plus the pad, so centring on the same
// measurement puts the text where `chip_w` says the shape is.
fn face(fb: &mut PaintBuffer, r: Rect, label: &[u8], on: bool, lit: bool) {
    let radius = RADIUS_PILL.min(r.3 / 2);
    let (bg, border, ink) =
        if on { (ACCENT_TINT, ACCENT_BORDER, ACCENT) } else { (BTN_BG, BTN_BORDER, LABEL) };
    fb.fill_round(r.0, r.1, r.2, r.3, radius, bg);
    if lit {
        fb.fill_round(r.0, r.1, r.2, r.3, radius, BTN_HOVER_BG);
    }
    fb.stroke_round(r.0, r.1, r.2, r.3, radius, 1, border);
    let x = r.0 + r.2.saturating_sub(text::width_of(label, PX_LABEL)) / 2;
    text::left(fb, x, text::centred_top(r.1, r.3, PX_LABEL), label, ink, PX_LABEL);
}
