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
use crate::pm::theme::BACKGROUND;

use super::chrome::{self, Rect};
use super::inspector;
use super::screens;
use super::sidebar;

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
    state.alert_visible = screens::sec_geom::visible(rect.h);
    screen(state, fb, &rect);
    if state.screen.has_inspector() {
        inspector::paint(state, fb);
    }
    chrome::status_bar(fb, state);
}

fn meta(state: &State, out: &mut [u8]) -> usize {
    let n = u32_decimal(state.rows.len() as u32, out);
    let tail = b" processes";
    let end = (n + tail.len()).min(out.len());
    out[n..end].copy_from_slice(&tail[..end - n]);
    end
}

// The Security screen's keyboard scroll clamps against state.alert_visible, and a
// screen painter only holds &State. The row count is pure geometry, so the funnel
// derives it from the same function the painter and the hit test read.
// The pane is the only region a screen owns. The inspector, sidebar and status
// strip are the frame's, so a screen painter can never reach them.
fn screen(state: &State, fb: &mut PaintBuffer, rect: &Rect) {
    match state.screen {
        Screen::Overview => screens::overview::paint(state, fb, rect),
        Screen::Processes => screens::processes::paint(state, fb, rect),
        Screen::Cpu => screens::cpu::paint(state, fb, rect),
        Screen::Memory => screens::memory::paint(state, fb, rect),
        Screen::Authority => screens::authority::paint(state, fb, rect),
        Screen::Security => screens::security::paint(state, fb, rect),
    }
}
