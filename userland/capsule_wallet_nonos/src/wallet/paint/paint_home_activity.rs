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
use crate::wallet::theme::{ACCENT, DIM, FG, LINE2, MUTED};

pub fn paint_home_activity(state: &State, fb: &mut PaintBuffer, cx: u32, cw: u32) {
    let col = (cw - 28) / 2;
    let rx = cx + col + 28;
    let _ = fb.text_ttf(cx as i32, 486, "ENABLED RAILS", DIM(), 10.5);

    // ETH and NOX show live balances; a field still loading shows "fetching"
    // while the link is up, a dash only when there is no route.
    let up = state.net.rpc_chain_ok;
    let pending = if up { "\u{2026}" } else { "\u{2014}" };
    let mut buf = [0u8; 40];
    let eth = if state.balance_ready {
        let n = format_eth(lower_u64(&state.balance_wei), &mut buf);
        core::str::from_utf8(&buf[..n]).unwrap_or(pending)
    } else {
        pending
    };
    rail(fb, cx, 508, col, "ETH", b"L1", eth);
    let mut nox_b = [0u8; 48];
    let nox = crate::wallet::nox::live_amount(
        state.nox.balance_ready,
        &state.nox.balance_wei,
        up,
        &mut nox_b,
    );
    rail(fb, cx, 570, col, "NOX", b"ERC-20", nox);
    rail(fb, cx, 632, col, "PR", b"RSVD", "\u{2014}");

    // No transaction index is fetched, so the wallet does not invent history.
    let _ = fb.text_ttf(rx as i32, 486, "RECENT ACTIVITY", DIM(), 10.5);
    ui::card(fb, rx, 508, col, 116);
    let _ = fb.text_ttf((rx + 18) as i32, 540, "No transactions yet", MUTED(), 13.0);
    let _ = fb.text_ttf((rx + 18) as i32, 566, "Sent and received transfers", DIM(), 11.5);
    let _ = fb.text_ttf((rx + 18) as i32, 584, "will appear here", DIM(), 11.5);
}

fn rail(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, sym: &str, tag: &[u8], val: &str) {
    ui::card(fb, x, y, w, 54);
    fb.fill_rect(x, y, 3, 54, ACCENT());
    let sx = fb.text_ttf((x + 18) as i32, (y + 18) as i32, sym, FG(), 16.0);
    ui::badge(fb, sx as u32 + 10, y + 18, tag, LINE2(), MUTED());
    let vw = fb.measure_ttf(val, 17.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 18 - vw) as i32, (y + 17) as i32, val, FG(), 17.0);
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
