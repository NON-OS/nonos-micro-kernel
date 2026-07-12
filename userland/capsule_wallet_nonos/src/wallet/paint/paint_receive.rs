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
use crate::wallet::hex::hex_addr;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, CYAN, FG, MUTED, PANEL_2};

// The content column starts just right of the sidebar. Keep the Generate
// button rect in lockstep with `on_pointer` (GENERATE_HIT) so a click and the
// painted target never drift apart.
const X: u32 = 368;
pub const GEN_BTN_X: u32 = 396;
pub const GEN_BTN_Y: u32 = 384;
pub const GEN_BTN_W: u32 = 220;
pub const GEN_BTN_H: u32 = 44;

pub fn paint_receive(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width.saturating_sub(X + 32);
    ui::title(fb, X, 118, b"RECEIVE", "Receive funds");

    if !state.address_ready {
        ui::card(fb, X, 196, w, 268);
        super::logo::logo(fb, X + 28, 228, 64);
        fb.text_scaled(X + 108, 244, b"No wallet yet", FG, 2);
        fb.text(X + 108, 288, b"Self-custody Ethereum account", MUTED);
        fb.text(X + 28, 336, b"Keys are generated and sealed inside the NONOS keyring.", MUTED);
        ui::primary(fb, GEN_BTN_X, GEN_BTN_Y, GEN_BTN_W, b"Generate wallet");
        fb.text(GEN_BTN_X + GEN_BTN_W + 20, GEN_BTN_Y + 15, b"or press G", MUTED);
        return;
    }

    let mut addr = [0u8; 42];
    hex_addr(&state.address, &mut addr);
    ui::card(fb, X, 196, w, 214);
    fb.text(X + 28, 220, b"Your Ethereum address", MUTED);
    fb.fill_rect(X + 28, 248, w.saturating_sub(56), 84, PANEL_2);
    fb.text_scaled(X + 48, 266, &addr[..21], ACCENT, 2);
    fb.text_scaled(X + 48, 298, &addr[21..], ACCENT, 2);
    fb.text(X + 28, 356, b"Accepts ETH and configured ERC-20 rails on Ethereum mainnet.", FG);
    fb.text(X + 28, 382, b"Do not send SAL here; Salvium uses its own native wallet.", CYAN);
}
