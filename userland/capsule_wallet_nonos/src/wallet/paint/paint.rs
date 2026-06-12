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

use crate::wallet::hex::hex_addr;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, BG, FG, MUTED, PANEL_2};

pub fn paint(state: &State, fb: &mut PaintBuffer) {
    fb.clear(BG);
    super::logo::logo(fb, 32, 28, 72);
    fb.text_scaled(124, 32, b"NONOS Wallet", FG, 2);
    fb.text(128, 78, b"self-custody NONOS rails", MUTED);
    super::panel::panel(fb, 32, 124, 260, 232);
    super::panel::panel(fb, 320, 124, 260, 232);
    fb.text(52, 150, b"Wallet", MUTED);
    if state.wallet_id == 0 {
        fb.text(52, 184, b"no wallet generated", FG);
    } else {
        fb.text(52, 184, b"wallet id", MUTED);
        let mut id = [0u8; 10];
        let n = super::format_u32::format_u32(state.wallet_id, &mut id);
        fb.text(148, 184, &id[..n], FG);
    }
    if state.address_ready {
        let mut addr = [0u8; 42];
        hex_addr(&state.address, &mut addr);
        fb.text(52, 318, &addr, ACCENT);
    } else {
        fb.text(52, 318, b"No address", ACCENT);
    }
    super::paint_rails::paint_rails(state, fb);
    fb.text(340, 150, b"Status", MUTED);
    fb.fill_rect(340, 180, 204, 34, PANEL_2);
    fb.text(354, 193, state.status, FG);
    fb.text(340, 256, b"Generate", FG);
    fb.text(340, 288, b"Refresh", FG);
    fb.text(428, 256, b"Sign ETH", FG);
    fb.text(428, 288, b"Sign NOX", FG);
    super::paint_tx::paint_tx(state, fb);
}
