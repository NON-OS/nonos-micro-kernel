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
use crate::wallet::hex::hex_addr;
use crate::wallet::state::State;
use crate::wallet::theme::{ACCENT, DIM, FG, MUTED, PANEL_2};

pub const GEN_BTN_X: u32 = 1162;
pub const GEN_BTN_Y: u32 = 182;
pub const GEN_BTN_W: u32 = 72;
pub const GEN_BTN_H: u32 = 42;

pub fn paint_receive(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    ui::card(fb, cx, 146, 300, 300);
    qr(fb, state, cx + 40, 176);
    let cap = "Scan to send to this account";
    let cwid = fb.measure_ttf(cap, 13.0).max(0) as u32;
    let _ = fb.text_ttf((cx + 150 - cwid / 2) as i32, 420, cap, DIM(), 13.0);

    let rx = cx + 316;
    let rw = cw - 316;
    ui::card(fb, rx, 146, rw, 116);
    let _ = fb.text_ttf((rx + 20) as i32, 164, "YOUR ADDRESS", DIM(), 10.5);
    let mut a = [0u8; 42];
    hex_addr(&state.address, &mut a);
    let _ =
        fb.text_ttf_mono((rx + 20) as i32, 190, core::str::from_utf8(&a).unwrap_or(""), FG(), 15.0);
    ui::primary(fb, rx + rw - 92, 182, 72, b"COPY");
    ui::outline(fb, rx + 20, 222, 88, b"Share");
    ui::outline(fb, rx + 118, 222, 100, b"Save QR");

    setup(fb, state, rx, rw);

    // The one account this wallet holds, at its real derivation path. The
    // wallet does not yet manage several accounts, so no others are invented.
    ui::card(fb, rx, 384, rw, 96);
    let _ = fb.text_ttf((rx + 20) as i32, 402, "ACCOUNT", DIM(), 10.5);
    let _ = fb.text_ttf_mono((rx + 20) as i32, 432, "m/44'/60'/0'/0/0", MUTED(), 13.0);
    if state.address_ready {
        let mut full = [0u8; 42];
        hex_addr(&state.address, &mut full);
        let _ = fb.text_ttf_mono(
            (rx + 20) as i32,
            456,
            core::str::from_utf8(&full).unwrap_or(""),
            FG(),
            13.0,
        );
    } else {
        let _ = fb.text_ttf((rx + 20) as i32, 456, "Generate an account first", MUTED(), 13.0);
    }
}

// Wallet setup: create a fresh account or import an existing key. During import
// only the number of characters typed is shown, never the key itself.
fn setup(fb: &mut PaintBuffer, state: &State, rx: u32, rw: u32) {
    ui::card(fb, rx, 278, rw, 90);
    if state.import_active {
        let _ = fb.text_ttf((rx + 20) as i32, 296, "IMPORT PRIVATE KEY", DIM(), 10.5);
        let track = rw - 40;
        let fill = track * (state.import_len.min(64) as u32) / 64;
        fb.fill_rect(rx + 20, 322, track, 6, PANEL_2());
        fb.fill_rect(rx + 20, 322, fill, 6, ACCENT());
        let mut nb = [0u8; 20];
        let dn = super::format_u64::format_u64(state.import_len as u64, &mut nb);
        let mut line = [0u8; 32];
        let ll = build_count(&nb[..dn], &mut line);
        let s = core::str::from_utf8(&line[..ll]).unwrap_or("");
        let _ = fb.text_ttf((rx + 20) as i32, 338, s, MUTED(), 12.0);
        let _ = fb.text_ttf((rx + 20) as i32, 358, "Enter to import, Esc to cancel", DIM(), 11.5);
    } else {
        let _ = fb.text_ttf((rx + 20) as i32, 296, "SET UP THIS WALLET", DIM(), 10.5);
        ui::primary(fb, rx + 20, 320, 150, b"Generate (G)");
        ui::outline(fb, rx + 182, 320, 180, b"Import key (I)");
        let _ = fb.text_ttf(
            (rx + 20) as i32,
            360,
            "Import hands the key straight to the keyring, never stored here.",
            DIM(),
            11.5,
        );
    }
}

fn build_count(digits: &[u8], out: &mut [u8]) -> usize {
    let n = digits.len().min(out.len());
    out[..n].copy_from_slice(&digits[..n]);
    let suf = b" / 64 hex chars";
    let take = suf.len().min(out.len() - n);
    out[n..n + take].copy_from_slice(&suf[..take]);
    n + take
}

// A real, scannable QR of the account as an EIP-681 payment URI, centred in a
// 220px panel with the four-module quiet zone the standard requires. No chain
// suffix, so wallets read it as a plain mainnet address, never a bogus network.
fn qr(fb: &mut PaintBuffer, state: &State, x: u32, y: u32) {
    const PANEL: u32 = 220;
    const QUIET: u32 = 4;
    const LIGHT: u32 = 0xFFEE_F2F7;
    const DARK: u32 = 0xFF0A_0D12;
    fb.fill_rect(x, y, PANEL, PANEL, LIGHT);

    if !state.address_ready {
        return;
    }

    // ethereum:0x{40 hex} in a fixed 51-byte buffer.
    let mut uri = [0u8; 51];
    uri[..9].copy_from_slice(b"ethereum:");
    let mut hex = [0u8; 42];
    hex_addr(&state.address, &mut hex);
    uri[9..].copy_from_slice(&hex);

    let Some(code) = nonos_qr::encode(&uri, nonos_qr::Ecc::Medium) else {
        return;
    };
    let modules = code.size as u32;
    let span = modules + 2 * QUIET;
    let scale = (PANEL / span).max(1);
    let drawn = span * scale;
    let ox = x + (PANEL - drawn) / 2 + QUIET * scale;
    let oy = y + (PANEL - drawn) / 2 + QUIET * scale;
    for my in 0..code.size {
        for mx in 0..code.size {
            if code.get(mx, my) {
                fb.fill_rect(ox + mx as u32 * scale, oy + my as u32 * scale, scale, scale, DARK);
            }
        }
    }
}
