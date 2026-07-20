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

use super::ui;
use crate::wallet::state::{State, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED};
use crate::wallet::theme::{ACCENT, DIM, FG, GREEN, GREEN_INK, INK, LINE2, MUTED, SEL};

// Nav geometry shared with the pointer hit-test (event::on_pointer) so drawing
// and click-mapping cannot drift. Item i sits at NAV_Y0 + i * NAV_STEP.
pub const NAV_X: u32 = 14;
pub const NAV_W: u32 = 172;
pub const NAV_H: u32 = 38;
pub const NAV_Y0: u32 = 96;
pub const NAV_STEP: u32 = 46;

pub fn paint_sidebar(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(18, 50, 20, 20, ACCENT());
    let _ = fb.text_ttf(48, 48, "NONOS", FG(), 19.0);
    nav(fb, NAV_Y0, "Home", state.view == VIEW_HOME);
    nav(fb, NAV_Y0 + NAV_STEP, "Receive", state.view == VIEW_RECEIVE);
    nav(fb, NAV_Y0 + 2 * NAV_STEP, "Send", state.view == VIEW_SEND);
    nav(fb, NAV_Y0 + 3 * NAV_STEP, "Proof", state.view == VIEW_PROOF);
    nav(fb, NAV_Y0 + 4 * NAV_STEP, "Shielded", state.view == VIEW_SHIELDED);
    nav(fb, NAV_Y0 + 5 * NAV_STEP, "NOX", state.view == VIEW_NOX);

    let _ = fb.text_ttf(22, 700, "RAILS", DIM(), 10.0);
    ui::chip(fb, 22, 722, b"ETH", ACCENT(), INK());
    ui::chip(fb, 70, 722, b"NOX", GREEN(), GREEN_INK());
    ui::chip(fb, 118, 722, b"PR", LINE2(), MUTED());
}

fn nav(fb: &mut PaintBuffer, y: u32, label: &str, active: bool) {
    let bullet = if active { ACCENT() } else { LINE2() };
    if active {
        fb.fill_rect(NAV_X, y, NAV_W, NAV_H, SEL());
        fb.fill_rect(NAV_X, y, 3, NAV_H, ACCENT());
    }
    fb.fill_rect(NAV_X + 16, y + 13, 12, 12, bullet);
    let color = if active { FG() } else { MUTED() };
    let _ = fb.text_ttf((NAV_X + 38) as i32, (y + 10) as i32, label, color, 15.0);
}
