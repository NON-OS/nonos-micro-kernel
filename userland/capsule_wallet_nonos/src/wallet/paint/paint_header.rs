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
use crate::wallet::theme::{ACCENT, FG, MUTED};

pub fn paint_header(state: &State, fb: &mut PaintBuffer) {
    super::logo::logo(fb, 32, 28, 76);
    fb.text_scaled(130, 32, b"NONOS Wallet", FG, 2);
    fb.text(134, 78, b"Ethereum mainnet rail custody", MUTED);
    if state.address_ready {
        let mut addr = [0u8; 42];
        hex_addr(&state.address, &mut addr);
        fb.text(520, 46, &addr[..22], ACCENT);
        fb.text(520, 66, &addr[22..], ACCENT);
    } else {
        fb.text(520, 56, b"Generate wallet to receive funds", MUTED);
    }
}
