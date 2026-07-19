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
use crate::wallet::theme::{ACCENT, BG, CYAN, FG, MUTED, NEUTRAL_400, NEUTRAL_800};

pub fn paint_account_card(state: &State, fb: &mut PaintBuffer) {
    let x: u32 = 368;
    let _ = fb.text_ttf(x as i32, 150, "ACCOUNT", ACCENT, 10.0);
    let head = if state.wallet_id == 0 { "Not created" } else { "Active wallet" };
    let _ = fb.text_ttf(x as i32, 166, head, NEUTRAL_400, 15.0);

    let mut buf = [0u8; 40];
    let n = format_eth(lower_u64(&state.balance_wei), &mut buf);
    let bal = core::str::from_utf8(&buf[..n]).unwrap_or("0");
    let pen = fb.text_ttf(x as i32, 190, bal, CYAN, 46.0);
    let _ = fb.text_ttf(pen + 8, 214, "ETH", ACCENT, 20.0);

    let ready = if state.address_ready { "Receive address ready" } else { "Generate a wallet to start" };
    let _ = fb.text_ttf(x as i32, 250, ready, ACCENT, 12.0);

    stat(fb, x, 278, 148, "NONCE", state.live_nonce);
    stat(fb, x + 160, 278, 148, "GAS WEI", state.fee_wei);

    let label: &[u8] = if state.wallet_id == 0 { b"Generate wallet" } else { b"Send ETH" };
    ui::primary(fb, x, 348, 308, label);
}

fn stat(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, value: u64) {
    fb.fill_rect(x, y, w, 52, BG);
    fb.fill_rect(x, y, w, 1, NEUTRAL_800);
    fb.fill_rect(x, y + 51, w, 1, NEUTRAL_800);
    fb.fill_rect(x, y, 1, 52, NEUTRAL_800);
    fb.fill_rect(x + w - 1, y, 1, 52, NEUTRAL_800);
    let _ = fb.text_ttf((x + 12) as i32, (y + 9) as i32, label, MUTED, 10.0);
    let mut out = [0u8; 20];
    let n = super::format_u64::format_u64(value, &mut out);
    let s = core::str::from_utf8(&out[..n]).unwrap_or("0");
    let _ = fb.text_ttf((x + 12) as i32, (y + 26) as i32, s, FG, 17.0);
}

fn format_eth(v: u64, out: &mut [u8]) -> usize {
    let whole = v / 1_000_000_000_000_000_000u64;
    let frac = ((v % 1_000_000_000_000_000_000u64) / 100_000_000_000_000u64) as u32;
    let mut wb = [0u8; 20];
    let wn = super::format_u64::format_u64(whole, &mut wb);
    out[..wn].copy_from_slice(&wb[..wn]);
    let mut n = wn;
    out[n] = b'.';
    n += 1;
    out[n] = b'0' + ((frac / 1000) % 10) as u8;
    out[n + 1] = b'0' + ((frac / 100) % 10) as u8;
    out[n + 2] = b'0' + ((frac / 10) % 10) as u8;
    out[n + 3] = b'0' + (frac % 10) as u8;
    n + 4
}

fn lower_u64(value: &[u8; 32]) -> u64 {
    u64::from_be_bytes([
        value[24], value[25], value[26], value[27], value[28], value[29], value[30], value[31],
    ])
}
