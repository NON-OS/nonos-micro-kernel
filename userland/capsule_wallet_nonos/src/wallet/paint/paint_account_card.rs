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
use crate::wallet::theme::{ACCENT, DIM, FG, GREEN, GREEN_INK, MUTED};

const BAL: [u8; 14] = [64, 52, 66, 42, 50, 30, 44, 28, 40, 26, 36, 20, 34, 18];

pub fn paint_account_card(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    ui::card(fb, x, y, w, 200);
    let _ = fb.text_ttf((x + 20) as i32, (y + 18) as i32, "TOTAL BALANCE  \u{00b7}  0x71C9\u{2026}4e2B", DIM, 10.5);
    let aw = fb.measure_ttf("ACTIVE", 11.0).max(0) as u32 + 18;
    ui::badge(fb, x + w - 20 - aw, y + 15, b"ACTIVE", GREEN, GREEN_INK);

    let mut buf = [0u8; 40];
    let n = format_eth(lower_u64(&state.balance_wei), &mut buf);
    let bal = core::str::from_utf8(&buf[..n]).unwrap_or("0");
    let pen = fb.text_ttf((x + 20) as i32, (y + 38) as i32, bal, FG, 44.0);
    let _ = fb.text_ttf(pen + 10, (y + 58) as i32, "ETH", ACCENT, 20.0);

    let dx = fb.text_ttf((x + 20) as i32, (y + 96) as i32, "= $7,516.40    ", MUTED, 13.0);
    ui::badge(fb, dx as u32 + 4, y + 94, b"+2.8% 24h", GREEN, GREEN_INK);

    ui::bars(fb, x + 20, y + 128, w - 40, 56, &BAL, ACCENT);
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

fn lower_u64(v: &[u8; 32]) -> u64 {
    u64::from_be_bytes([v[24], v[25], v[26], v[27], v[28], v[29], v[30], v[31]])
}
