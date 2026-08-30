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

//! Home screen content. The activity strip is painted by the dispatcher on top
//! of this surface, so the layout starts where that strip ends.

use nonos_app_skeleton::PaintBuffer;

use super::super::app::Editor;
use super::super::theme;
use super::create::paint_create;
use super::metrics::{pane_x, CARD_W, COL_GAP, PANE_PAD};
use super::pane::paint_pane_head;
use super::rail::paint_rail;
use super::recent::paint_recent;
use super::state::HomeState;

const MIN_COLS_W: u32 = 200;

pub(crate) fn paint_home(_ed: &mut Editor, fb: &mut PaintBuffer) {
    let (w, h) = (fb.width, fb.height);
    fb.fill_rect(0, 0, w, h, theme::active().background);
    HomeState::note_width(w);
    if h == 0 || w <= pane_x() + PANE_PAD * 2 {
        return;
    }
    let st = HomeState::load();
    paint_rail(fb, &st);
    paint_pane_head(fb);
    if w >= pane_x() + PANE_PAD * 2 + CARD_W + COL_GAP + MIN_COLS_W {
        paint_recent(fb);
        paint_create(fb);
    }
}
