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
use crate::wallet::theme::{ACCENT, AMBER, AMBER_INK, DIM, FG, GREEN, GREEN_INK, INK, MUTED};

pub fn paint_proof_view(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    let _ = fb.text_ttf(cx as i32, 160, "SIGNED TRANSACTIONS", DIM(), 12.1);

    if !state.tx_ready {
        ui::card(fb, cx, 180, cw, 96);
        let _ = fb.text_ttf((cx + 20) as i32, 212, "No transactions yet", MUTED(), 17.2);
        let _ = fb.text_ttf(
            (cx + 20) as i32,
            238,
            "A signed transfer and its on-chain receipt appear here.",
            DIM(),
            14.4,
        );
        return;
    }

    // The one real transaction this session signed, with its live status.
    let mut hb = [0u8; 13];
    short_hash(&state.tx_hash, &mut hb);
    let hash = core::str::from_utf8(&hb).unwrap_or("");

    let (tag, bg, fg): (&[u8], u32, u32) = if state.receipt_ready {
        if state.receipt_ok {
            (b"CONFIRMED", GREEN(), GREEN_INK())
        } else {
            (b"FAILED", AMBER(), AMBER_INK())
        }
    } else if state.broadcast_ready {
        (b"SENT", ACCENT(), INK())
    } else {
        (b"SIGNED", ACCENT(), INK())
    };

    ui::card(fb, cx, 180, cw, 60);
    fb.fill_rect(cx, 180, 3, 60, ACCENT());
    let _ = fb.text_ttf_mono((cx + 20) as i32, 192, hash, FG(), 18.4);
    let mut meta = [0u8; 32];
    let ml = kind_meta(state.tx_kind, &mut meta);
    let _ = fb.text_ttf(
        (cx + 20) as i32,
        216,
        core::str::from_utf8(&meta[..ml]).unwrap_or(""),
        DIM(),
        14.4,
    );
    let bw = fb.measure_ttf(core::str::from_utf8(tag).unwrap_or(""), 12.6).max(0) as u32 + 18;
    ui::badge(fb, cx + cw - 20 - bw, 197, tag, bg, fg);
}

// "0x" + first two and last two bytes of the 32-byte hash.
pub(super) fn short_hash(h: &[u8; 32], out: &mut [u8; 13]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out[0] = b'0';
    out[1] = b'x';
    out[2] = HEX[(h[0] >> 4) as usize];
    out[3] = HEX[(h[0] & 15) as usize];
    out[4] = HEX[(h[1] >> 4) as usize];
    out[5] = HEX[(h[1] & 15) as usize];
    out[6] = 0xE2;
    out[7] = 0x80;
    out[8] = 0xA6; // ellipsis
    out[9] = HEX[(h[30] >> 4) as usize];
    out[10] = HEX[(h[30] & 15) as usize];
    out[11] = HEX[(h[31] >> 4) as usize];
    out[12] = HEX[(h[31] & 15) as usize];
}

fn kind_meta(kind: &[u8], out: &mut [u8; 32]) -> usize {
    let label: &[u8] = match kind {
        b"ETH" => b"ETH transfer",
        b"NOX" => b"NOX approve",
        b"APPROVE" => b"Staking approve",
        b"STAKE" => b"NOX stake",
        _ => b"Transaction",
    };
    let n = label.len().min(out.len());
    out[..n].copy_from_slice(&label[..n]);
    n
}
