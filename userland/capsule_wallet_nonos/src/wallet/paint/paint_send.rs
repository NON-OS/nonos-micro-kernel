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
use crate::wallet::state::{State, SEND_FIELD_AMOUNT, SEND_FIELD_TO};
use crate::wallet::theme::{ACCENT, AMBER, DIM, FG, INK, LINE2, MUTED, PANEL_2};

pub fn paint_send(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let lw = 640u32;
    let ix = cx + 20;
    let iw = lw - 40;
    ui::card(fb, cx, 146, lw, 470);

    // Asset toggle: send ETH (a value transfer) or NOX (an ERC-20 transfer).
    asset_tab(fb, ix + iw - 156, 152, "ETH", state.send_token == 0);
    asset_tab(fb, ix + iw - 76, 152, "NOX", state.send_token == 1);

    // The recipient exactly as typed, prefixed 0x, or an empty-field prompt.
    let _ = fb.text_ttf(ix as i32, 162, "RECIPIENT", DIM(), 10.5);
    let mut to = [0u8; 42];
    to[0] = b'0';
    to[1] = b'x';
    to[2..2 + state.send_to_len].copy_from_slice(&state.send_to_hex[..state.send_to_len]);
    let to_str = if state.send_to_len == 0 {
        "Paste a 0x recipient address"
    } else {
        core::str::from_utf8(&to[..2 + state.send_to_len]).unwrap_or("")
    };
    field(fb, ix, 182, iw, to_str, state.send_focus == SEND_FIELD_TO);
    let ok = state.send_to_len == 40;
    let _ = fb.text_ttf(
        ix as i32,
        232,
        if ok { "20-byte address" } else { "enter 40 hex characters" },
        if ok { ACCENT() } else { MUTED() },
        12.0,
    );

    // The amount exactly as entered (milli-units), in the selected asset.
    let amount_label = if state.send_token == 1 { "AMOUNT (NOX)" } else { "AMOUNT (ETH)" };
    let _ = fb.text_ttf(ix as i32, 272, amount_label, DIM(), 10.5);
    let mut ab = [0u8; 24];
    let an = format_milli_eth(state.send_amount_milli_eth, &mut ab);
    field(
        fb,
        ix,
        292,
        iw,
        core::str::from_utf8(&ab[..an]).unwrap_or("0"),
        state.send_focus == SEND_FIELD_AMOUNT,
    );

    // The live network fee, not a fabricated dollar figure.
    let _ = fb.text_ttf(ix as i32, 352, "NETWORK FEE", DIM(), 10.5);
    ui::bordered(fb, ix, 372, iw, 42, PANEL_2(), LINE2());
    let mut gb = [0u8; 32];
    let gn = gwei(state.fee_wei, &mut gb);
    let fee_txt = if state.fee_ready {
        core::str::from_utf8(&gb[..gn]).unwrap_or("\u{2014}")
    } else {
        "fetching\u{2026}"
    };
    let _ = fb.text_ttf((ix + 14) as i32, 384, "Gas price", MUTED(), 14.0);
    let fw = fb.measure_ttf(fee_txt, 15.0).max(0) as u32;
    let _ = fb.text_ttf((ix + iw - 14 - fw) as i32, 383, fee_txt, FG(), 15.0);

    ui::bordered(fb, ix, 432, iw, 40, 0xFF17_130A, 0xFF5A_4A1E);
    fb.fill_rect(ix + 14, 446, 10, 10, AMBER());
    let _ = fb.text_ttf(
        (ix + 34) as i32,
        444,
        "Verify the recipient. Transfers cannot be reversed.",
        AMBER(),
        13.0,
    );

    ui::primary(fb, ix, 500, 150, b"Sign & send");
    super::paint_send_side::paint_send_side(state, fb);
}

// milli-ETH to a "W.FFF" ETH string.
fn format_milli_eth(milli: u32, out: &mut [u8]) -> usize {
    let whole = milli / 1000;
    let frac = milli % 1000;
    let mut wb = [0u8; 20];
    let wn = super::format_u64::format_u64(whole as u64, &mut wb);
    out[..wn].copy_from_slice(&wb[..wn]);
    out[wn] = b'.';
    out[wn + 1] = b'0' + ((frac / 100) % 10) as u8;
    out[wn + 2] = b'0' + ((frac / 10) % 10) as u8;
    out[wn + 3] = b'0' + (frac % 10) as u8;
    wn + 4
}

// wei-per-gas to a "N.NN gwei" string, two decimals so a sub-gwei price shows.
fn gwei(wei: u64, out: &mut [u8]) -> usize {
    let whole = wei / 1_000_000_000;
    let cents = (wei % 1_000_000_000) / 10_000_000;
    let mut gb = [0u8; 20];
    let n = super::format_u64::format_u64(whole, &mut gb);
    out[..n].copy_from_slice(&gb[..n]);
    out[n] = b'.';
    out[n + 1] = b'0' + ((cents / 10) % 10) as u8;
    out[n + 2] = b'0' + (cents % 10) as u8;
    out[n + 3..n + 8].copy_from_slice(b" gwei");
    n + 8
}

fn asset_tab(fb: &mut PaintBuffer, x: u32, y: u32, label: &str, on: bool) {
    if on {
        fb.fill_rect(x, y, 74, 26, ACCENT());
    } else {
        ui::edge(fb, x, y, 74, 26, LINE2());
    }
    let c = if on { INK() } else { MUTED() };
    let tw = fb.measure_ttf(label, 12.0).max(0) as u32;
    let _ = fb.text_ttf((x + 37 - tw / 2) as i32, (y + 6) as i32, label, c, 12.0);
}

fn field(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, val: &str, active: bool) {
    let e = if active { ACCENT() } else { LINE2() };
    ui::bordered(fb, x, y, w, 40, PANEL_2(), e);
    let _ = fb.text_ttf((x + 12) as i32, (y + 11) as i32, val, FG(), 15.0);
}
