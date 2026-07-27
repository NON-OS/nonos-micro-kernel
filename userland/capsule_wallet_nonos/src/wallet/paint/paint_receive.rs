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

pub fn paint_receive(state: &State, fb: &mut PaintBuffer) {
    // The one-time backup screen replaces the whole view until the user
    // confirms the phrase is written down; nothing else may draw over it.
    if state.backup_active {
        super::paint_backup::paint_backup(state, fb);
        return;
    }
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    ui::card(fb, cx, 146, 300, 300);
    qr(fb, state, cx + 40, 176);
    let cap = "Scan to send to this account";
    let cwid = fb.measure_ttf(cap, 14.9).max(0) as u32;
    let _ = fb.text_ttf((cx + 150 - cwid / 2) as i32, 420, cap, DIM(), 14.9);

    let rx = cx + 316;
    let rw = cw - 316;
    ui::card(fb, rx, 146, rw, 116);
    let _ = fb.text_ttf((rx + 20) as i32, 164, "YOUR ADDRESS", DIM(), 12.1);
    let mut a = [0u8; 42];
    hex_addr(&state.address, &mut a);
    let _ =
        fb.text_ttf_mono((rx + 20) as i32, 190, core::str::from_utf8(&a).unwrap_or(""), FG(), 17.2);
    ui::primary(fb, rx + rw - 92, 182, 72, b"COPY");
    ui::outline(fb, rx + 20, 222, 88, b"Share");
    ui::outline(fb, rx + 118, 222, 100, b"Save QR");

    setup(fb, state, rx, rw);

    // The one account this wallet holds, at its real derivation path. The
    // wallet does not yet manage several accounts, so no others are invented.
    ui::card(fb, rx, 384, rw, 96);
    if state.export_active {
        let _ = fb.text_ttf(
            (rx + 20) as i32,
            402,
            "PRIVATE KEY \u{2014} never share this",
            ACCENT(),
            12.1,
        );
        // The full 66-char key (0x + 64 hex) on one line, so it is obviously a
        // complete private key, with a caption confirming what it is.
        let key = core::str::from_utf8(&state.export_hex).unwrap_or("");
        let _ = fb.text_ttf_mono((rx + 20) as i32, 432, key, FG(), 14.0);
        let _ = fb.text_ttf(
            (rx + 20) as i32,
            458,
            "64 hex chars \u{00b7} import this anywhere to recover this exact account",
            DIM(),
            11.5,
        );
        ui::outline(fb, rx + 300, 392, 96, b"Hide");
    } else {
        let _ = fb.text_ttf((rx + 20) as i32, 402, "ACCOUNT", DIM(), 12.1);
        let _ = fb.text_ttf_mono((rx + 20) as i32, 432, "m/44'/60'/0'/0/0", MUTED(), 14.9);
        if state.address_ready {
            let mut full = [0u8; 42];
            hex_addr(&state.address, &mut full);
            let _ = fb.text_ttf_mono(
                (rx + 20) as i32,
                456,
                core::str::from_utf8(&full).unwrap_or(""),
                FG(),
                14.9,
            );
            ui::outline(fb, rx + 300, 392, 96, b"Export (K)");
        } else {
            let _ = fb.text_ttf((rx + 20) as i32, 456, "Generate an account first", MUTED(), 14.9);
        }
    }
}

// Wallet setup: create a fresh HD account, import an existing key, or recover
// from a BIP39 phrase. During import only the number of characters typed is
// shown, never the key itself; the recovery phrase shows while typed so the
// user can check the words, and is wiped on submit or cancel.
fn setup(fb: &mut PaintBuffer, state: &State, rx: u32, rw: u32) {
    ui::card(fb, rx, 278, rw, 96);
    if state.recover_active {
        let _ = fb.text_ttf((rx + 20) as i32, 296, "RECOVER FROM PHRASE", DIM(), 12.1);
        let fw = rw - 40;
        ui::bordered(fb, rx + 20, 314, fw, 36, PANEL_2(), ACCENT());
        let typed = core::str::from_utf8(&state.recover_buf[..state.recover_len]).unwrap_or("");
        // Show the tail that fits so the caret area stays visible while long
        // phrases are typed.
        let mut shown = typed;
        while fb.measure_ttf(shown, 14.9).max(0) as u32 > fw - 24 && !shown.is_empty() {
            shown = &shown[1..];
        }
        let _ = fb.text_ttf_mono((rx + 32) as i32, 324, shown, FG(), 14.9);
        let words = state.recover_buf[..state.recover_len]
            .split(|&b| b == b' ')
            .filter(|w| !w.is_empty())
            .count() as u64;
        let mut nb = [0u8; 20];
        let dn = super::format_u64::format_u64(words, &mut nb);
        let mut cnt = [0u8; 12];
        let cl = build_word_count(&nb[..dn], &mut cnt);
        let cs = core::str::from_utf8(&cnt[..cl]).unwrap_or("");
        let cw2 = fb.measure_ttf(cs, 13.8).max(0) as u32;
        let _ = fb.text_ttf((rx + rw - 32 - cw2) as i32, 326, cs, MUTED(), 13.8);
        let _ = fb.text_ttf(
            (rx + 20) as i32,
            360,
            "12-24 words, space separated  \u{00b7}  Enter to recover  \u{00b7}  Esc to cancel",
            DIM(),
            13.2,
        );
        return;
    }
    if state.import_active {
        let _ = fb.text_ttf((rx + 20) as i32, 296, "IMPORT PRIVATE KEY", DIM(), 12.1);
        // Show the key as it is typed so the user can read every character back
        // and confirm it, the way a hardware wallet's import screen does. This
        // is entered only on a machine the user already trusts with the key. A
        // progress underline fills toward 64.
        let fw = rw - 40;
        ui::bordered(fb, rx + 20, 314, fw, 36, PANEL_2(), ACCENT());
        // "0x" then the typed hex; show the visible tail so the caret stays in
        // view for a full-length key.
        let mut buf = [0u8; 2 + 64];
        buf[0] = b'0';
        buf[1] = b'x';
        let n = state.import_len.min(64);
        buf[2..2 + n].copy_from_slice(&state.import_hex[..n]);
        let typed = core::str::from_utf8(&buf[..2 + n]).unwrap_or("0x");
        let mut shown = typed;
        while fb.measure_ttf(shown, 14.9).max(0) as u32 > fw - 24 && shown.len() > 2 {
            shown = &shown[1..];
        }
        let _ = fb.text_ttf_mono((rx + 32) as i32, 324, shown, FG(), 14.9);
        let mut nb = [0u8; 20];
        let dn = super::format_u64::format_u64(state.import_len as u64, &mut nb);
        let mut cnt = [0u8; 8];
        let cl = build_count(&nb[..dn], &mut cnt);
        let cs = core::str::from_utf8(&cnt[..cl]).unwrap_or("");
        let cw = fb.measure_ttf(cs, 13.8).max(0) as u32;
        let _ = fb.text_ttf((rx + rw - 32 - cw) as i32, 326, cs, MUTED(), 13.8);
        let fill = (fw - 4) * (state.import_len.min(64) as u32) / 64;
        fb.fill_rect(rx + 22, 348, fill, 2, ACCENT());
        let _ = fb.text_ttf(
            (rx + 20) as i32,
            360,
            "Paste or type 64 hex chars  \u{00b7}  Enter to import  \u{00b7}  Esc to cancel",
            DIM(),
            13.2,
        );
    } else {
        let _ = fb.text_ttf((rx + 20) as i32, 296, "SET UP THIS WALLET", DIM(), 12.1);
        ui::primary(fb, rx + 20, 318, 150, b"Generate (G)");
        ui::outline(fb, rx + 182, 318, 180, b"Import key (I)");
        ui::outline(fb, rx + 374, 318, 160, b"Recover (M)");
        let _ = fb.text_ttf(
            (rx + 20) as i32,
            366,
            "Generate makes a fresh key with a 12-word phrase. Recover rebuilds the same account from yours.",
            DIM(),
            13.2,
        );
    }
}

fn build_word_count(digits: &[u8], out: &mut [u8]) -> usize {
    let n = digits.len().min(out.len());
    out[..n].copy_from_slice(&digits[..n]);
    let suf = b" words";
    let take = suf.len().min(out.len() - n);
    out[n..n + take].copy_from_slice(&suf[..take]);
    n + take
}

fn build_count(digits: &[u8], out: &mut [u8]) -> usize {
    let n = digits.len().min(out.len());
    out[..n].copy_from_slice(&digits[..n]);
    let suf = b" / 64";
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
