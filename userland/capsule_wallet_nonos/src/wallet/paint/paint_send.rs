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
use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_NONCE, SEND_FIELD_TO};
use crate::wallet::theme::{ACCENT, BG, FG, MUTED, NEUTRAL_800, WARN};

pub fn paint_send(state: &State, fb: &mut PaintBuffer) {
    let w = fb.width.saturating_sub(368);
    let left_w = w * 3 / 5;
    let right_x = 368 + left_w + 24;
    let right_w = w.saturating_sub(left_w + 64);
    ui::title(fb, 368, 128, b"SEND", "Compose ETH transfer");
    super::panel::panel(fb, 336, 196, left_w, fb.height.saturating_sub(290));
    if right_w > 240 {
        super::panel::panel(fb, right_x - 32, 196, right_w, 190);
    }

    let iw = left_w.saturating_sub(64);
    let addr: &[u8] = if state.send_to_len == 0 { b"40 hex characters" } else { &state.send_to_hex[..state.send_to_len] };
    field(fb, 368, 226, iw, "Recipient", addr, state.send_focus == SEND_FIELD_TO);
    let hw = (iw.saturating_sub(16)) / 2;
    let mut ab = [0u8; 20];
    let an = super::format_u64::format_u64(state.send_amount_milli_eth as u64, &mut ab);
    field(fb, 368, 300, hw, "Amount mETH", &ab[..an], state.send_focus == SEND_FIELD_AMOUNT);
    let mut nb = [0u8; 20];
    let nn = super::format_u64::format_u64(state.send_nonce, &mut nb);
    field(fb, 368 + hw + 16, 300, hw, "Nonce", &nb[..nn], state.send_focus == SEND_FIELD_NONCE);

    ui::primary(fb, 368, 372, 200, b"Sign locally");
    ui::badge(fb, 584, 380, b"Broadcast waits for NONOS HTTPS RPC", MUTED);

    if right_w > 240 {
        let _ = fb.text_ttf(right_x as i32, 214, "ROUTE", ACCENT, 10.0);
        let _ = fb.text_ttf(right_x as i32, 230, "PublicNode Ethereum RPC", FG, 15.0);
        ui::badge(fb, right_x, 262, super::paint_send_route_label::paint_send_route_label(state), WARN);
        let _ = fb.text_ttf(right_x as i32, 300, "Est. confirmation ~12s", MUTED, 12.0);
    }
    super::paint_tx::paint_tx(state, fb);
}

fn field(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, value: &[u8], active: bool) {
    let _ = fb.text_ttf(x as i32, y as i32, label, MUTED, 12.0);
    let iy = y + 20;
    fb.fill_rect(x, iy, w, 34, BG);
    let bc = if active { ACCENT } else { NEUTRAL_800 };
    fb.fill_rect(x, iy, w, 1, bc);
    fb.fill_rect(x, iy + 33, w, 1, bc);
    fb.fill_rect(x, iy, 1, 34, bc);
    fb.fill_rect(x + w - 1, iy, 1, 34, bc);
    let s = core::str::from_utf8(value).unwrap_or("");
    let _ = fb.text_ttf((x + 10) as i32, (iy + 9) as i32, s, FG, 13.0);
}
