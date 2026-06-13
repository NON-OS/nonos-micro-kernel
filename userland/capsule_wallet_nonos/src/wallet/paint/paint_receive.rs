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
use crate::wallet::theme::{ACCENT, CYAN, FG, MUTED, PANEL_2};

pub fn paint_receive(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width.saturating_sub(368);
    super::panel::panel(fb, 336, 128, w - 16, fb.height.saturating_sub(220));
    fb.text(368, 164, b"Receive funds", MUTED);
    if !state.address_ready {
        fb.text_scaled(368, 220, b"No wallet generated", FG, 2);
        fb.text(368, 276, b"Press G to create a NONOS keyring wallet", MUTED);
        return;
    }
    let mut addr = [0u8; 42];
    hex_addr(&state.address, &mut addr);
    fb.text_scaled(368, 220, b"Ethereum address", FG, 2);
    fb.fill_rect(368, 292, w.saturating_sub(96), 96, PANEL_2);
    fb.text_scaled(404, 318, &addr[..22], ACCENT, 2);
    fb.text_scaled(404, 356, &addr[22..], ACCENT, 2);
    fb.text(368, 436, b"Accept ETH and configured ERC-20 rails on Ethereum mainnet.", FG);
    fb.text(368, 472, b"Do not send SAL here; Salvium uses the separate native wallet.", CYAN);
}
