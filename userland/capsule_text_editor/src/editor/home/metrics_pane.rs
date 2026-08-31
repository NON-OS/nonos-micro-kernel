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

//! Right-hand pane geometry: the content column, the search field, and the
//! two-column split beneath it.

use super::metrics::{lh, pane_x, BODY, CARD_W, COL_GAP, HEAD, PANE_PAD, SEARCH_H, SUBHEAD};

pub(super) fn pane_content(w: u32) -> (u32, u32) {
    let x = pane_x() + PANE_PAD;
    (x, w.saturating_sub(x + PANE_PAD))
}

pub(super) fn search_rect(w: u32) -> (u32, u32, u32, u32) {
    let (x, cw) = pane_content(w);
    (x, PANE_PAD + lh(HEAD) + 4 + lh(BODY) + 18, cw, SEARCH_H)
}

pub(super) fn cols_y(w: u32) -> u32 {
    let (_, y, _, h) = search_rect(w);
    y + h + 26
}

pub(super) fn doc_row_h() -> u32 {
    lh(BODY) * 2 + 14
}

pub(super) fn docs_rect(w: u32) -> (u32, u32, u32) {
    let (x, cw) = pane_content(w);
    let y = cols_y(w) + lh(SUBHEAD) + 14;
    (x, y, cw.saturating_sub(CARD_W + COL_GAP))
}

pub(super) fn card_x(w: u32) -> u32 {
    let (x, cw) = pane_content(w);
    x + cw.saturating_sub(CARD_W)
}

pub(super) fn docs_list_rect(w: u32, h: u32, count: usize) -> (u32, u32, u32, u32) {
    let (x, y, cw) = docs_rect(w);
    let rh = doc_row_h().max(1);
    let room = (h.saturating_sub(y) / rh) as usize;
    (x, y, cw, count.min(room) as u32 * rh)
}
