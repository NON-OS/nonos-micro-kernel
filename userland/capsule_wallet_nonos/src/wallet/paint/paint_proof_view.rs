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
use crate::wallet::theme::{ACCENT, AMBER, AMBER_INK, CYAN, DIM, FG, GREEN, GREEN_INK, INK, LINE2, MUTED, PANEL_2};

pub fn paint_proof_view(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let cw = fb.width.saturating_sub(252);
    ui::card(fb, cx, 146, cw, 64);
    fb.fill_rect(cx + 20, 172, 10, 10, ACCENT);
    let _ = fb.text_ttf((cx + 40) as i32, 170, "Generating proof  \u{00b7}  0x33f1\u{2026}", FG, 14.0);
    let _ = fb.text_ttf((cx + cw - 60) as i32, 170, "68%", CYAN, 14.0);
    ui::bordered(fb, cx + 20, 192, cw - 40, 6, PANEL_2, PANEL_2);
    fb.fill_rect(cx + 20, 192, (cw - 40) * 68 / 100, 6, ACCENT);

    ui::bordered(fb, cx, 228, cw - 320, 40, PANEL_2, LINE2);
    let _ = fb.text_ttf((cx + 14) as i32, 239, "Search by hash\u{2026}", MUTED, 14.0);
    seg(fb, cx + cw - 300, 228, state.proof_filter);

    prow(fb, cx, cw, 288, "0x9f2a\u{2026}7bd1", "2 min ago  \u{00b7}  1.5 ETH", b"PROVED", GREEN, GREEN_INK);
    prow(fb, cx, cw, 352, "0x114c\u{2026}2e90", "9 min ago  \u{00b7}  0.2 ETH", b"PEND", AMBER, AMBER_INK);
    prow(fb, cx, cw, 416, "0x77ab\u{2026}01c4", "1 h ago  \u{00b7}  4.0 ETH", b"PROVED", GREEN, GREEN_INK);
}

fn seg(fb: &mut PaintBuffer, x: u32, y: u32, sel: u8) {
    cell(fb, x, y, 60, "All", sel == 0);
    cell(fb, x + 60, y, 110, "Proved", sel == 1);
    cell(fb, x + 170, y, 130, "Pending", sel == 2);
}

fn cell(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, label: &str, on: bool) {
    if on {
        fb.fill_rect(x, y, w, 40, ACCENT);
    } else {
        ui::edge(fb, x, y, w, 40, LINE2);
    }
    let c = if on { INK } else { MUTED };
    let tw = fb.measure_ttf(label, 13.0).max(0) as u32;
    let _ = fb.text_ttf((x + w / 2 - tw / 2) as i32, (y + 12) as i32, label, c, 13.0);
}

fn prow(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, hash: &str, meta: &str, b: &[u8], bg: u32, fg: u32) {
    ui::card(fb, x, y, w, 54);
    fb.fill_rect(x, y, 3, 54, ACCENT);
    let _ = fb.text_ttf_mono((x + 20) as i32, (y + 10) as i32, hash, FG, 16.0);
    let _ = fb.text_ttf((x + 20) as i32, (y + 32) as i32, meta, DIM, 12.0);
    let bw = fb.measure_ttf(core::str::from_utf8(b).unwrap_or(""), 11.0).max(0) as u32 + 18;
    ui::badge(fb, x + w - 20 - bw, y + 17, b, bg, fg);
}
