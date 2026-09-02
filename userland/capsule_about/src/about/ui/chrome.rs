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

use crate::about::state::State;
use crate::about::theme::{MUTED, TITLE};

use super::metrics::{
    BODY_PX, HEAD_H, HEAD_META_GAP, PANE_PAD_TOP, PANE_PAD_X, SIDEBAR_W, STATUS_H, TITLE_PX,
};
use super::text;

pub use super::status_bar::paint as status_bar;

// The content box for the active screen: everything right of the sidebar, below
// the page head, above the status bar. Every screen painter takes this rect and
// never recomputes it.
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub fn pane_rect(fb_w: u32, fb_h: u32) -> Rect {
    Rect {
        x: SIDEBAR_W + PANE_PAD_X,
        y: PANE_PAD_TOP + HEAD_H,
        w: fb_w.saturating_sub(SIDEBAR_W + PANE_PAD_X * 2),
        h: fb_h.saturating_sub(PANE_PAD_TOP + HEAD_H + STATUS_H + PANE_PAD_X),
    }
}

// The section name holds the left of the head band and its meta line the right,
// measured back from the pane's right edge so a longer name never pushes it off
// the window. If the two would collide the meta yields: the name identifies the
// screen, the meta only describes it.
pub fn page_head(fb: &mut PaintBuffer, state: &State) {
    let title_top = text::top_of(PANE_PAD_TOP as i32, HEAD_H, TITLE_PX);
    let label = state.section.nav_label();
    text::line(fb, SIDEBAR_W + PANE_PAD_X, title_top, label, TITLE, TITLE_PX);
    let meta = state.section.head_meta();
    let meta_top = text::top_of(PANE_PAD_TOP as i32, HEAD_H, BODY_PX);
    let right_x = state.fb_w.saturating_sub(PANE_PAD_X);
    let title_end = SIDEBAR_W + PANE_PAD_X + text::width(fb, label, TITLE_PX) + HEAD_META_GAP;
    if title_end + text::width(fb, meta, BODY_PX) <= right_x {
        text::right(fb, right_x, meta_top, meta, MUTED, BODY_PX);
    }
}
