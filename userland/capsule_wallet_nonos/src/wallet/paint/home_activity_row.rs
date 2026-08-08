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

use super::activity_status::status_of;
use super::scale;
use super::ui;
use crate::wallet::state::State;
use crate::wallet::theme::{DIM, FG, MUTED};

/// Recent activity: what this session actually signed and sent.
///
/// This panel used to print "no transactions yet" unconditionally, so a
/// transfer that signed, broadcast and confirmed still read as nothing having
/// happened. The wallet has always known: it holds the kind, the hash and the
/// receipt, and the proof screen showed them. Home simply never looked.
///
/// It still does not invent history. There is no transaction index being
/// fetched, so what is shown is this session's work and it says so, rather
/// than implying a complete account history.
pub fn activity(state: &State, fb: &mut PaintBuffer, rx: u32, col: u32, y: u32) {
    let _ = fb.text_ttf(rx as i32, (y - 22) as i32, "RECENT ACTIVITY", DIM(), scale::LABEL);
    ui::card(fb, rx, y, col, 116);
    if !state.tx_ready {
        let _ = fb.text_ttf(
            (rx + 18) as i32,
            (y + 32) as i32,
            "No transactions yet",
            MUTED(),
            scale::BODY,
        );
        let _ = fb.text_ttf(
            (rx + 18) as i32,
            (y + 58) as i32,
            "Sent and received transfers",
            DIM(),
            13.2,
        );
        let _ =
            fb.text_ttf((rx + 18) as i32, (y + 76) as i32, "will appear here", DIM(), scale::SMALL);
        return;
    }
    let kind = core::str::from_utf8(state.tx_kind).unwrap_or("TX");
    let _ = fb.text_ttf((rx + 18) as i32, (y + 22) as i32, kind, FG(), scale::BODY);
    let (label, tone) = status_of(state);
    let tw = fb.measure_ttf(label, scale::SMALL).max(0) as u32;
    let _ = fb.text_ttf(
        (rx + col).saturating_sub(18 + tw) as i32,
        (y + 24) as i32,
        label,
        tone,
        scale::SMALL,
    );
    // The hash is the thing worth copying into a block explorer, so it gets
    // the room rather than being truncated to a stub.
    let mut hb = [0u8; 13];
    super::paint_proof_view::short_hash(&state.broadcast_hash, &mut hb);
    if let Ok(h) = core::str::from_utf8(&hb) {
        let _ = fb.text_ttf((rx + 18) as i32, (y + 52) as i32, h, MUTED(), scale::SMALL);
    }
    let _ = fb.text_ttf((rx + 18) as i32, (y + 78) as i32, "this session", DIM(), scale::LABEL);
}
