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

use crate::wallet::state::{
    State, VIEW_HOME, VIEW_NOX, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND, VIEW_SHIELDED,
};
use crate::wallet::theme::{ACCENT, CYAN, FG, MUTED, PANEL_2};

pub fn paint_sidebar(state: &State, fb: &mut PaintBuffer) {
    super::logo::logo(fb, 32, 36, 80);
    let _ = fb.text_ttf(128, 40, "NONOS", FG, 16.0);
    let _ = fb.text_ttf(128, 66, "WALLET", MUTED, 10.0);
    nav(fb, 32, 160, "Home", state.view == VIEW_HOME);
    nav(fb, 32, 212, "Receive", state.view == VIEW_RECEIVE);
    nav(fb, 32, 264, "Send", state.view == VIEW_SEND);
    nav(fb, 32, 316, "Proof", state.view == VIEW_PROOF);
    nav(fb, 32, 368, "Shielded", state.view == VIEW_SHIELDED);
    nav(fb, 32, 420, "NOX", state.view == VIEW_NOX);
    let ry = fb.height.saturating_sub(94);
    let _ = fb.text_ttf(32, ry as i32, "RAILS", MUTED, 10.0);
    let cy = fb.height.saturating_sub(66) as i32;
    let _ = fb.text_ttf(32, cy, "ETH", CYAN, 12.0);
    let _ = fb.text_ttf(84, cy, "NOX", ACCENT, 12.0);
    let _ = fb.text_ttf(136, cy, "PR", MUTED, 12.0);
}

fn nav(fb: &mut PaintBuffer, x: u32, y: u32, text: &str, active: bool) {
    if active {
        fb.fill_rect(x, y, 220, 38, PANEL_2);
        fb.fill_rect(x, y, 3, 38, ACCENT);
    }
    let color = if active { ACCENT } else { FG };
    let _ = fb.text_ttf((x + 16) as i32, (y + 10) as i32, text, color, 14.0);
}
