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

use crate::pm::format::u32_decimal;
use crate::pm::state::{Screen, State};
use crate::pm::theme::{BACKGROUND, CARD_BG, CARD_BORDER, MUTED};

use super::chrome::{self, Rect};
use super::metrics::{BODY_PX, PANEL_RADIUS};
use super::sidebar;
use super::text;

// The single frame funnel: ground, sidebar, page head, the active screen's pane,
// then the status strip over the full width. Order is load-bearing, since the
// window is opaque and every later layer paints over the one before it.
pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    let (w, h) = (fb.width, fb.height);
    sidebar::paint(fb, state.screen, h, state.refreshes);
    let rect = chrome::pane_rect(w, h, state.screen.has_inspector());
    let mut buf = [0u8; 24];
    let n = meta(state, &mut buf);
    chrome::page_head(fb, state.screen, &buf[..n]);
    pane(fb, &rect, state.screen);
    chrome::status_bar(fb, state);
}

fn meta(state: &State, out: &mut [u8]) -> usize {
    let n = u32_decimal(state.rows.len() as u32, out);
    let tail = b" processes";
    let end = (n + tail.len()).min(out.len());
    out[n..end].copy_from_slice(&tail[..end - n]);
    end
}

// Phase 3 replaces this with one painter per screen. Until then every screen
// draws its own empty pane, so the chrome is verifiable on its own terms and no
// screen silently borrows another's layout.
fn pane(fb: &mut PaintBuffer, rect: &Rect, screen: Screen) {
    fb.fill_round(rect.x, rect.y, rect.w, rect.h, PANEL_RADIUS, CARD_BG);
    fb.stroke_round(rect.x, rect.y, rect.w, rect.h, PANEL_RADIUS, 1, CARD_BORDER);
    let label = screen.nav_label();
    let top = text::centred_top(rect.y, rect.h, BODY_PX);
    let x = rect.x + rect.w.saturating_sub(text::width(fb, label, BODY_PX)) / 2;
    text::left(fb, x, top, label, MUTED, BODY_PX);
}
