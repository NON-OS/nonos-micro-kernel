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
use crate::wallet::theme::{CYAN, DIM, FG, GREEN, MUTED};

pub fn paint_portfolio(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let col = (cw - 16) / 2;

    // Transparent: the live on-chain balance.
    ui::card(fb, cx, 146, col, 120);
    let _ = fb.text_ttf((cx + 20) as i32, 166, "TRANSPARENT  \u{00b7}  ON-CHAIN", DIM(), 10.5);
    let mut buf = [0u8; 40];
    let bal = if state.balance_ready {
        let n = format_eth(lower_u64(&state.balance_wei), &mut buf);
        core::str::from_utf8(&buf[..n]).unwrap_or("\u{2014}")
    } else {
        "\u{2014}"
    };
    let px = fb.text_ttf((cx + 20) as i32, 188, bal, FG(), 34.0);
    let _ = fb.text_ttf(px + 8, 202, "ETH", CYAN(), 16.0);
    let _ = fb.text_ttf((cx + 20) as i32, 240, "public balance", MUTED(), 13.0);

    // Shielded: no note scanner is wired yet, so no private balance is shown.
    let rx = cx + col + 16;
    ui::card(fb, rx, 146, col, 120);
    let _ = fb.text_ttf((rx + 20) as i32, 166, "SHIELDED  \u{00b7}  PRIVATE", DIM(), 10.5);
    let sx = fb.text_ttf((rx + 20) as i32, 188, "\u{2014}", GREEN(), 34.0);
    let _ = fb.text_ttf(sx + 8, 202, "ETH", GREEN(), 16.0);
    let _ = fb.text_ttf((rx + 20) as i32, 240, "shielded notes not scanned", MUTED(), 13.0);
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
