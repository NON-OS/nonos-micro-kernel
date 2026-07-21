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
use crate::wallet::theme::{DIM, FG, LINE, MUTED, PANEL_2};

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

    ui::card(fb, rx, 278, rw, 90);
    let _ = fb.text_ttf((rx + 20) as i32, 296, "REQUEST A SPECIFIC AMOUNT", DIM(), 10.5);
    ui::bordered(fb, rx + 20, 320, rw - 190, 34, PANEL_2(), LINE());
    let _ = fb.text_ttf((rx + 32) as i32, 329, "0.00 ETH", MUTED(), 14.0);
    ui::primary(fb, rx + rw - 150, 320, 130, b"Create link");

    ui::card(fb, rx, 384, rw, 180);
    let _ = fb.text_ttf((rx + 20) as i32, 402, "DERIVED ADDRESSES", DIM(), 10.5);
    drow(fb, rx, rw, 428, "m/44'/60'/0'/0", "0x71C9\u{2026}4e2B", "2.41 ETH");
    drow(fb, rx, rw, 468, "m/44'/60'/0'/1", "0x33aa\u{2026}9f01", "0.00 ETH");
    drow(fb, rx, rw, 508, "m/44'/60'/0'/2", "0xd1c8\u{2026}6b2e", "0.12 ETH");
}

fn drow(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, path: &str, addr: &str, amt: &str) {
    fb.fill_rect(x + 20, y + 28, w - 40, 1, LINE());
    let _ = fb.text_ttf_mono((x + 20) as i32, (y + 4) as i32, path, MUTED(), 13.0);
    let aw = fb.measure_ttf_mono(addr, 13.0).max(0) as u32;
    let _ = fb.text_ttf_mono((x + w / 2 - aw / 2) as i32, (y + 4) as i32, addr, FG(), 13.0);
    let vw = fb.measure_ttf(amt, 13.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 20 - vw) as i32, (y + 4) as i32, amt, FG(), 13.0);
}

// A real, scannable QR of the account as an EIP-681 payment URI, centred in a
// 220px panel with the four-module quiet zone the standard requires.
fn qr(fb: &mut PaintBuffer, state: &State, x: u32, y: u32) {
    const PANEL: u32 = 220;
    const QUIET: u32 = 4;
    const LIGHT: u32 = 0xFFEE_F2F7;
    const DARK: u32 = 0xFF0A_0D12;
    fb.fill_rect(x, y, PANEL, PANEL, LIGHT);

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
