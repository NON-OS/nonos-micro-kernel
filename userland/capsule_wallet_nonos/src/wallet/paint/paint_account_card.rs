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
use crate::wallet::hex::short_addr;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, DIM, FG, GREEN, GREEN_INK, MUTED};

pub fn paint_account_card(state: &State, fb: &mut PaintBuffer, x: u32, y: u32, w: u32) {
    ui::card(fb, x, y, w, 132);

    // The real account address, or a clear prompt before one is generated.
    if state.address_ready {
        let mut sa = [0u8; 13];
        short_addr(&state.address, &mut sa);
        let label = core::str::from_utf8(&sa).unwrap_or("");
        let lx =
            fb.text_ttf((x + 20) as i32, (y + 18) as i32, "TOTAL BALANCE  \u{00b7}  ", DIM(), 12.1);
        let _ = fb.text_ttf(lx, (y + 18) as i32, label, DIM(), 12.1);
        let aw = fb.measure_ttf("ACTIVE", 12.6).max(0) as u32 + 18;
        ui::badge(fb, x + w - 20 - aw, y + 15, b"ACTIVE", GREEN(), GREEN_INK());
    } else {
        let _ = fb.text_ttf((x + 20) as i32, (y + 18) as i32, "NO ACCOUNT YET", DIM(), 12.1);
    }

    // Headline the NOX balance (the native token), with the live ETH balance on
    // the line beneath it. Each shows a fetching mark while its read is in
    // flight and a dash only when there is no route.
    let up = state.net.rpc_chain_ok;
    let mut nb = [0u8; 48];
    let nox = crate::wallet::nox::live_amount(
        state.nox.balance_ready,
        &state.nox.balance_wei,
        up,
        &mut nb,
    );
    let pen = fb.text_ttf((x + 20) as i32, (y + 44) as i32, nox, FG(), 46.0);
    let _ = fb.text_ttf(pen + 10, (y + 62) as i32, "NOX", ACCENT(), 20.7);

    let mut eb = [0u8; 40];
    let eth = if state.balance_ready {
        let n = format_eth(lower_u64(&state.balance_wei), &mut eb);
        core::str::from_utf8(&eb[..n]).unwrap_or("0")
    } else if up {
        "\u{2026}"
    } else {
        "\u{2014}"
    };
    let ex = fb.text_ttf((x + 20) as i32, (y + 102) as i32, eth, MUTED(), 17.2);
    let _ = fb.text_ttf(ex + 6, (y + 103) as i32, "ETH", DIM(), 14.9);
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
