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
use crate::wallet::theme::{ACCENT, BG, CYAN, FG, MUTED, NEUTRAL_800};

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
        let _ = fb.text_ttf((X + 108) as i32, 236, "No wallet yet", FG, 18.0);
        let _ = fb.text_ttf((X + 108) as i32, 268, "Self-custody Ethereum account", MUTED, 13.0);
        let _ = fb.text_ttf((X + 28) as i32, 322, "Keys are generated and sealed inside the NONOS keyring.", MUTED, 13.0);
        ui::primary(fb, GEN_BTN_X, GEN_BTN_Y, GEN_BTN_W, b"Generate wallet");
        let _ = fb.text_ttf((GEN_BTN_X + GEN_BTN_W + 20) as i32, (GEN_BTN_Y + 13) as i32, "or press G", MUTED, 12.0);
        return;
    }

    let mut addr = [0u8; 42];
    hex_addr(&state.address, &mut addr);
    ui::card(fb, X, 196, w, 200);
    let _ = fb.text_ttf((X + 28) as i32, 218, "Your Ethereum address", MUTED, 12.0);
    let chw = w.saturating_sub(56);
    fb.fill_rect(X + 28, 248, chw, 44, BG);
    fb.fill_rect(X + 28, 248, chw, 1, NEUTRAL_800);
    fb.fill_rect(X + 28, 291, chw, 1, NEUTRAL_800);
    fb.fill_rect(X + 28, 248, 1, 44, NEUTRAL_800);
    fb.fill_rect(X + 28 + chw - 1, 248, 1, 44, NEUTRAL_800);
    let s = core::str::from_utf8(&addr).unwrap_or("");
    let _ = fb.text_ttf_mono((X + 40) as i32, 260, s, ACCENT, 15.0);
    let _ = fb.text_ttf((X + 28) as i32, 312, "Accepts ETH and configured ERC-20 rails on Ethereum mainnet.", FG, 13.0);
    let _ = fb.text_ttf((X + 28) as i32, 338, "Do not send SAL here; Salvium uses its own native wallet.", CYAN, 13.0);
}
