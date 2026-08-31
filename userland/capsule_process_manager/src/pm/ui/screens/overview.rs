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

use crate::pm::state::State;

use super::super::chrome::Rect;
use super::super::metrics::{CARD_GAP, CARD_H};
use super::super::table;
use super::super::table_geom::COLS_OVERVIEW;
use super::{ovw_cards, ovw_counts};

// Four stat cards across the top, then the five-column table taking the rest.
// The card width is divided out of r.w rather than fixed, because this screen
// docks the inspector and the pane narrows with it.
pub fn paint(state: &State, fb: &mut PaintBuffer, r: &Rect) {
    let w = r.w.saturating_sub(CARD_GAP * 3) / 4;
    let step = w + CARD_GAP;
    ovw_cards::cpu(state, fb, r.x, r.y, w);
    ovw_cards::memory(state, fb, r.x + step, r.y, w);
    ovw_counts::processes(state, fb, r.x + step * 2, r.y, w);
    ovw_counts::authority(state, fb, r.x + step * 3, r.y, w);
    let below = CARD_H + CARD_GAP;
    let rect = Rect { x: r.x, y: r.y + below, w: r.w, h: r.h.saturating_sub(below) };
    table::paint(state, fb, &rect, &COLS_OVERVIEW);
}
