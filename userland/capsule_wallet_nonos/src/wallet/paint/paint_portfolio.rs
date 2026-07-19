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
use crate::wallet::theme::{ACCENT, CYAN, FG, MUTED};

// Portfolio: transparent balance and shielded balances per asset, reconstructed
// locally from the note store. The split is always explicit: public vs private.
pub fn paint_portfolio(state: &State, fb: &mut PaintBuffer) {
    let cx = 368;
    let w = fb.width.saturating_sub(cx + 32);
    super::ui::title(fb, cx, 140, b"PORTFOLIO", "Transparent and shielded");

    super::ui::section(fb, cx, 216, "Transparent");
    let col = (w.saturating_sub(24)) / 2;
    if state.balance_ready {
        let mut b = [0u8; 20];
        let n = super::format_u64::format_u64(eth_of(&state.balance_wei), &mut b);
        super::ui::metric(fb, cx, 244, col, b"On-chain balance", &b[..n], b"ETH", CYAN);
    } else {
        super::ui::metric(fb, cx, 244, col, b"On-chain balance", b"...", b"", MUTED);
    }

    super::ui::section(fb, cx, 356, "Shielded");
    if state.notes.is_empty() {
        super::ui::empty_state(
            fb,
            cx,
            384,
            w,
            b"No shielded notes yet",
            b"Sync is view-tag scanned once the chain feed connects; balances rebuild locally.",
        );
    } else {
        let mut y = 384u32;
        for asset in state.notes.assets() {
            asset_row(fb, cx, y, w, asset, state.notes.balance(asset));
            y = y.saturating_add(88);
        }
    }

    let mut c = [0u8; 10];
    let cn = super::format_u32::format_u32(state.notes.unspent_count() as u32, &mut c);
    let yy = fb.height.saturating_sub(150);
    let bw = super::ui::badge(fb, cx, yy, b"unspent notes", ACCENT);
    let s = core::str::from_utf8(&c[..cn]).unwrap_or("0");
    let _ = fb.text_ttf((cx + bw + 10) as i32, (yy + 4) as i32, s, FG, 12.0);
}

fn asset_row(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, asset: u32, value_wei: u128) {
    let mut a = [0u8; 10];
    let na = super::format_u32::format_u32(asset, &mut a);
    let mut label = [b'a', b's', b's', b'e', b't', b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let ll = 6 + na;
    label[6..ll].copy_from_slice(&a[..na]);
    let mut b = [0u8; 20];
    let n = super::format_u64::format_u64((value_wei / 1_000_000_000_000_000_000) as u64, &mut b);
    super::ui::metric(fb, x, y, w, &label[..ll], &b[..n], b"", ACCENT);
}

fn eth_of(wei32: &[u8; 32]) -> u64 {
    let mut lo = [0u8; 16];
    lo.copy_from_slice(&wei32[16..]);
    (u128::from_be_bytes(lo) / 1_000_000_000_000_000_000) as u64
}
