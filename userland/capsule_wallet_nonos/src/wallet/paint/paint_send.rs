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

use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_NONCE, SEND_FIELD_TO};
use crate::wallet::theme::{ACCENT, CYAN, FG, MUTED, PANEL_2, WARN};

pub fn paint_send(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width.saturating_sub(368);
    let left_w = w * 3 / 5;
    let right_x = 368 + left_w + 24;
    let right_w = w.saturating_sub(left_w + 64);
    super::panel::panel(fb, 336, 128, left_w, fb.height.saturating_sub(220));
    if right_w > 240 {
        super::panel::panel(fb, right_x - 32, 128, right_w, fb.height.saturating_sub(220));
    }
    fb.text(368, 164, b"Compose ETH transfer", MUTED);
    field(fb, 368, 214, left_w.saturating_sub(64), b"Recipient", state.send_focus == SEND_FIELD_TO);
    to_value(state, fb, 520, 238);
    field(fb, 368, 296, left_w.saturating_sub(64), b"Amount mETH", state.send_focus == SEND_FIELD_AMOUNT);
    num_value(fb, 560, 320, state.send_amount_milli_eth as u64);
    field(fb, 368, 378, left_w.saturating_sub(64), b"Nonce", state.send_focus == SEND_FIELD_NONCE);
    num_value(fb, 560, 402, state.send_nonce);
    super::paint_button::paint_button(fb, 368, 474, 210, b"Sign locally");
    fb.text(600, 486, b"Broadcast waits for NONOS HTTPS RPC.", WARN);
    if right_w > 240 {
        fb.text(right_x, 164, b"Route", MUTED);
        fb.text(right_x, 214, b"PublicNode Ethereum RPC", CYAN);
        fb.text(right_x, 258, if state.net.rpc_tcp_ok { b"TCP ready, TLS pending" } else { b"TLS pending" }, WARN);
    }
    super::paint_tx::paint_tx(state, fb);
}

fn field(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &[u8], active: bool) {
    fb.fill_rect(x, y, w, 58, PANEL_2);
    fb.fill_rect(x, y, 5, 58, if active { ACCENT } else { MUTED });
    fb.text(x + 22, y + 22, label, MUTED);
}

fn to_value(state: &State, fb: &mut PaintBuffer, x: u32, y: u32) {
    if state.send_to_len == 0 {
        fb.text(x, y, b"40 hex characters", FG);
    } else {
        fb.text(x, y, &state.send_to_hex[..state.send_to_len], FG);
    }
}

fn num_value(fb: &mut PaintBuffer, x: u32, y: u32, value: u64) {
    let mut out = [0u8; 20];
    let n = super::format_u64::format_u64(value, &mut out);
    fb.text(x, y, &out[..n], FG);
}
