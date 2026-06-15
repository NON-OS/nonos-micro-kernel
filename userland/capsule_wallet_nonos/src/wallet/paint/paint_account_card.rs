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

use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, FG, MUTED};

pub fn paint_account_card(state: &State, fb: &mut PaintBuffer) {
    fb.text(368, 160, b"Account", MUTED);
    fb.text_scaled(
        368,
        198,
        if state.wallet_id == 0 { b"Not created" } else { b"Active wallet" },
        FG,
        2,
    );
    fb.text(
        368,
        250,
        if state.address_ready { b"Receive address ready" } else { b"Generate wallet to start" },
        ACCENT,
    );
    fb.text(368, 286, if state.balance_ready { b"Balance live" } else { b"Balance pending" }, MUTED);
    paint_u64(fb, 520, 286, lower_u64(&state.balance_wei));
    fb.text(368, 322, if state.nonce_ready { b"Nonce" } else { b"Nonce pending" }, MUTED);
    paint_u64(fb, 520, 322, state.live_nonce);
    fb.text(368, 358, if state.fee_ready { b"Gas wei" } else { b"Gas pending" }, MUTED);
    paint_u64(fb, 520, 358, state.fee_wei);
    super::paint_button::paint_button(fb, 368, 394, 180, b"Generate");
}

fn paint_u64(fb: &mut PaintBuffer, x: u32, y: u32, value: u64) {
    let mut out = [0u8; 20];
    let n = super::format_u64::format_u64(value, &mut out);
    fb.text(x, y, &out[..n], FG);
}

fn lower_u64(value: &[u8; 32]) -> u64 {
    u64::from_be_bytes([
        value[24], value[25], value[26], value[27], value[28], value[29], value[30], value[31],
    ])
}
