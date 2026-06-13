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

use crate::wallet::state::{State, VIEW_HOME, VIEW_PROOF, VIEW_RECEIVE, VIEW_SEND};
use crate::wallet::theme::{ACCENT, FG, PANEL_2};

pub fn paint_tabs(state: &State, fb: &mut PaintBuffer) {
    paint_tab(fb, 32, b"Home", state.view == VIEW_HOME);
    paint_tab(fb, 140, b"Receive", state.view == VIEW_RECEIVE);
    paint_tab(fb, 276, b"Send", state.view == VIEW_SEND);
    paint_tab(fb, 384, b"Proof", state.view == VIEW_PROOF);
}

fn paint_tab(fb: &mut PaintBuffer, x: u32, text: &[u8], active: bool) {
    fb.fill_rect(x, 118, 96, 26, PANEL_2);
    if active {
        fb.fill_rect(x, 142, 96, 3, ACCENT);
    }
    fb.text(x + 18, 126, text, FG);
}
