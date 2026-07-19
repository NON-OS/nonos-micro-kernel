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
use crate::wallet::theme::{ACCENT, FG, MUTED, OK};

pub fn paint_proofs(state: &State, fb: &mut PaintBuffer) {
    if state.proof_count < 2 {
        return;
    }
    let w = fb.width.saturating_sub(384);
    let mut eth = [0u8; 66];
    let mut nox = [0u8; 66];
    super::hex_hash::hex_hash(&state.proof_eth_hash, &mut eth);
    super::hex_hash::hex_hash(&state.proof_nox_hash, &mut nox);
    row(fb, w, 200, &eth[..34], "ETH transfer - confirmed on-chain", b"confirmed", OK);
    row(fb, w, 268, &nox[..34], "NOX attestation - sealed locally", b"verified", ACCENT);
}

fn row(fb: &mut PaintBuffer, w: u32, y: u32, hash: &[u8], meta: &str, tag: &[u8], tone: u32) {
    ui::card(fb, 336, y, w, 56);
    let hs = core::str::from_utf8(hash).unwrap_or("");
    let _ = fb.text_ttf_mono(368, (y + 12) as i32, hs, FG, 14.0);
    let _ = fb.text_ttf(368, (y + 34) as i32, meta, MUTED, 11.0);
    let tw = fb.measure_ttf(core::str::from_utf8(tag).unwrap_or(""), 11.0).max(0) as u32 + 20;
    ui::badge(fb, 336 + w - 24 - tw, y + 17, tag, tone);
}
