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

use crate::pm::state::{Screen, State};
use crate::pm::theme::{MUTED, TITLE};

use super::chips;
use super::metrics::{
    BODY_PX, HEAD_H, INSPECTOR_W, PANE_PAD_TOP, PANE_PAD_X, SEARCH_META_GAP, SIDEBAR_W, STATUS_H,
    TITLE_PX,
};
use super::search;
use super::text;

pub use super::status_bar::paint as status_bar;

// The content box for the active screen: everything right of the sidebar, below
// the page head, above the status bar, and left of the inspector when one is
// docked. Every screen painter takes this rect and never recomputes it.
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub fn pane_rect(fb_w: u32, fb_h: u32, inspector: bool) -> Rect {
    let right = if inspector { fb_w.saturating_sub(INSPECTOR_W) } else { fb_w };
    Rect {
        x: SIDEBAR_W + PANE_PAD_X,
        y: PANE_PAD_TOP + HEAD_H,
        w: right.saturating_sub(SIDEBAR_W + PANE_PAD_X * 2),
        h: fb_h.saturating_sub(PANE_PAD_TOP + HEAD_H + STATUS_H + PANE_PAD_X),
    }
}

// The screen name holds the left of the head band, the search field the far
// right, and the meta line is measured back from the field's left edge, so a
// wider count shifts neither of them.
pub fn page_head(fb: &mut PaintBuffer, state: &State, meta: &[u8]) {
    let title_top = text::centred_top(PANE_PAD_TOP, HEAD_H, TITLE_PX);
    let label = state.screen.nav_label();
    text::left(fb, SIDEBAR_W + PANE_PAD_X, title_top, label, TITLE, TITLE_PX);
    let meta_top = text::centred_top(PANE_PAD_TOP, HEAD_H, BODY_PX);
    let right_x = search::rect(fb.width, state.screen).0.saturating_sub(SEARCH_META_GAP);
    if meta_fits(state.screen, right_x, text::width(fb, meta, BODY_PX)) {
        text::right(fb, right_x, meta_top, meta, MUTED, BODY_PX);
    }
    chips::paint(fb, state);
    search::paint(fb, state);
}

// The chip row and the meta line share the middle of the band. The chips carry
// state the user can act on, so the count is the one that yields on a collision.
fn meta_fits(screen: Screen, right_x: u32, meta_w: u32) -> bool {
    match chips::origin(screen) {
        None => true,
        Some(x) => x + chips::width() + SEARCH_META_GAP <= right_x.saturating_sub(meta_w),
    }
}
