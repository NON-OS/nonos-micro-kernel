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

use crate::about::section::Section;
use crate::about::state::State;
use crate::about::theme::BACKGROUND;

use super::chrome::{self, Rect};
use super::screens;
use super::sidebar;

// The single frame funnel: ground, sidebar, page head, the active section's
// pane, then the status strip over the full width. Order is load-bearing, since
// the window is opaque and every later layer paints over the one before it.
// The surface dimensions land in the state on the way through, because this is
// the only place the pane rect and the section's content height are both known,
// and the scroll clamp needs the pair.
pub fn paint(state: &mut State, fb: &mut PaintBuffer) {
    fb.clear(BACKGROUND);
    let (w, h) = (fb.width, fb.height);
    state.fb_w = w;
    state.fb_h = h;
    let rect = chrome::pane_rect(w, h);
    state.record_extent(rect.h, screens::content_h(state, &rect));
    sidebar::paint(fb, state.section, h);
    chrome::page_head(fb, state);
    section(state, fb, &rect);
    chrome::status_bar(fb, state);
}

// The pane is the only region a section owns. The sidebar, head band and status
// strip are the frame's, so a section painter can never reach them.
fn section(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    match state.section {
        Section::Overview => screens::overview::paint(state, fb, rect),
        Section::System => screens::system::paint(state, fb, rect),
        Section::Trust => screens::trust::paint(state, fb, rect),
        Section::Display => screens::display::paint(state, fb, rect),
        Section::Licenses => screens::licenses::paint(state, fb, rect),
    }
}
