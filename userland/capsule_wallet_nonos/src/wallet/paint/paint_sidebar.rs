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
    fb.text_scaled(128, 42, b"NONOS", FG, 2);
    fb.text(132, 88, b"Wallet", MUTED);
    nav(fb, 32, 160, b"Home", state.view == VIEW_HOME);
    nav(fb, 32, 212, b"Receive", state.view == VIEW_RECEIVE);
    nav(fb, 32, 264, b"Send", state.view == VIEW_SEND);
    nav(fb, 32, 316, b"Proof", state.view == VIEW_PROOF);
    nav(fb, 32, 368, b"Shielded", state.view == VIEW_SHIELDED);
    nav(fb, 32, 420, b"NOX", state.view == VIEW_NOX);
    fb.text(32, fb.height.saturating_sub(92), b"Rails", MUTED);
    fb.text(32, fb.height.saturating_sub(62), b"ETH", CYAN);
    fb.text(92, fb.height.saturating_sub(62), b"NOX", ACCENT);
    fb.text(154, fb.height.saturating_sub(62), b"PR", MUTED);
}

fn nav(fb: &mut PaintBuffer, x: u32, y: u32, text: &[u8], active: bool) {
    fb.fill_rect(x, y, 220, 38, PANEL_2);
    if active {
        fb.fill_rect(x, y, 5, 38, ACCENT);
    }
    fb.text(x + 24, y + 13, text, FG);
}
