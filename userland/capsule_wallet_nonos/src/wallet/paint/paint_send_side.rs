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
use crate::wallet::state::State;
use crate::wallet::theme::{DIM, FG, GREEN, GREEN_INK, MUTED};

pub fn paint_send_side(state: &State, fb: &mut PaintBuffer) {
    let x = 890u32;
    let w = fb.width.saturating_sub(x + 26);
    ui::card(fb, x, 146, w, 200);
    let _ = fb.text_ttf((x + 20) as i32, 164, "ROUTE", DIM(), 12.1);
    let _ = fb.text_ttf((x + 20) as i32, 186, "PublicNode Ethereum RPC", FG(), 18.4);
    ui::badge(fb, x + 20, 218, b"TLS 1.3", GREEN(), GREEN_INK());

    // Real values fetched from the chain, or a dash before they arrive.
    let mut nb = [0u8; 20];
    let nonce = if state.nonce_ready {
        let n = super::format_u64::format_u64(state.send_nonce, &mut nb);
        core::str::from_utf8(&nb[..n]).unwrap_or("\u{2014}")
    } else {
        "\u{2014}"
    };
    kv(fb, x, w, 258, "Nonce", nonce);
    let mut gnum = [0u8; 20];
    let mut gb = [0u8; 32];
    let gas = if state.fee_ready {
        let whole = state.fee_wei / 1_000_000_000;
        let cents = (state.fee_wei % 1_000_000_000) / 10_000_000;
        let gn = super::format_u64::format_u64(whole, &mut gnum);
        gb[..gn].copy_from_slice(&gnum[..gn]);
        gb[gn] = b'.';
        gb[gn + 1] = b'0' + ((cents / 10) % 10) as u8;
        gb[gn + 2] = b'0' + (cents % 10) as u8;
        gb[gn + 3..gn + 8].copy_from_slice(b" gwei");
        core::str::from_utf8(&gb[..gn + 8]).unwrap_or("\u{2014}")
    } else {
        "\u{2014}"
    };
    kv(fb, x, w, 284, "Gas price", gas);
    kv(fb, x, w, 310, "Gas limit", "21000");
}

fn kv(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, k: &str, v: &str) {
    let _ = fb.text_ttf((x + 20) as i32, y as i32, k, MUTED(), 14.9);
    let vw = fb.measure_ttf(v, 14.9).max(0) as u32;
    let _ = fb.text_ttf((x + w - 20 - vw) as i32, y as i32, v, FG(), 14.9);
}
