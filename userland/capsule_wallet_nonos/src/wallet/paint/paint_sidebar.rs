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

pub fn paint_sidebar(state: &State, fb: &mut PaintBuffer) {
    fb.fill_rect(18, 50, 20, 20, ACCENT);
    let _ = fb.text_ttf(48, 48, "NONOS", FG, 19.0);
    nav(fb, 96, "Home", state.view == VIEW_HOME);
    nav(fb, 142, "Receive", state.view == VIEW_RECEIVE);
    nav(fb, 188, "Send", state.view == VIEW_SEND);
    nav(fb, 234, "Proof", state.view == VIEW_PROOF);
    nav(fb, 280, "Shielded", state.view == VIEW_SHIELDED);
    nav(fb, 326, "NOX", state.view == VIEW_NOX);

    let _ = fb.text_ttf(22, 700, "RAILS", DIM, 10.0);
    ui::chip(fb, 22, 722, b"ETH", ACCENT, INK);
    ui::chip(fb, 70, 722, b"NOX", GREEN, GREEN_INK);
    ui::chip(fb, 118, 722, b"PR", LINE2, MUTED);
}

fn nav(fb: &mut PaintBuffer, y: u32, label: &str, active: bool) {
    let bullet = if active { ACCENT } else { LINE2 };
    if active {
        fb.fill_rect(14, y, 172, 38, SEL);
        fb.fill_rect(14, y, 3, 38, ACCENT);
    }
    fb.fill_rect(30, y + 13, 12, 12, bullet);
    let color = if active { FG } else { MUTED };
    let _ = fb.text_ttf(52, (y + 10) as i32, label, color, 15.0);
}
