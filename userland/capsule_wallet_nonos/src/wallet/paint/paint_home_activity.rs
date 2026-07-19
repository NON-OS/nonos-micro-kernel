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
use crate::wallet::theme::{ACCENT, AMBER, AMBER_INK, DIM, FG, GREEN, GREEN_INK, LINE2, MUTED, PANEL_2};

pub fn paint_home_activity(_state: &State, fb: &mut PaintBuffer, cx: u32, cw: u32) {
    let col = (cw - 28) / 2;
    let rx = cx + col + 28;
    let _ = fb.text_ttf(cx as i32, 486, "ENABLED RAILS", DIM(), 10.5);
    rail(fb, cx, 508, col, "ETH", b"L1", "2.4091");
    rail(fb, cx, 570, col, "NOX", b"ERC-20", "18,204");
    rail(fb, cx, 632, col, "PR", b"RSVD", "\u{2014}");

    let _ = fb.text_ttf(rx as i32, 486, "RECENT ACTIVITY", DIM(), 10.5);
    act(fb, rx, 508, col, "^", "Sent to 0x9A2c\u{2026}", "2 min ago", "-1.5 ETH", b"PROVED", GREEN(), GREEN_INK());
    act(fb, rx, 570, col, "#", "Staked NOX", "1 h ago", "+4,000", b"CONF", GREEN(), GREEN_INK());
    act(fb, rx, 632, col, "v", "Received", "yesterday", "+0.8 ETH", b"CONF", GREEN(), GREEN_INK());
    act(fb, rx, 694, col, "*", "Shield note", "2 d ago", "0.34 ETH", b"PEND", AMBER(), AMBER_INK());
}

fn rail(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, sym: &str, tag: &[u8], val: &str) {
    ui::card(fb, x, y, w, 54);
    fb.fill_rect(x, y, 3, 54, ACCENT());
    let sx = fb.text_ttf((x + 18) as i32, (y + 18) as i32, sym, FG(), 16.0);
    ui::badge(fb, sx as u32 + 10, y + 18, tag, LINE2(), MUTED());
    let vw = fb.measure_ttf(val, 17.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 18 - vw) as i32, (y + 17) as i32, val, FG(), 17.0);
}

fn act(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, ic: &str, t: &str, s: &str, amt: &str, b: &[u8], bg: u32, fg: u32) {
    ui::card(fb, x, y, w, 54);
    ui::bordered(fb, x + 14, y + 12, 30, 30, PANEL_2(), LINE2());
    let _ = fb.text_ttf((x + 25) as i32, (y + 17) as i32, ic, MUTED(), 14.0);
    let _ = fb.text_ttf((x + 56) as i32, (y + 9) as i32, t, FG(), 14.0);
    let _ = fb.text_ttf((x + 56) as i32, (y + 30) as i32, s, DIM(), 12.0);
    let aw = fb.measure_ttf(amt, 14.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 18 - aw) as i32, (y + 9) as i32, amt, FG(), 14.0);
    let bw = fb.measure_ttf(core::str::from_utf8(b).unwrap_or(""), 11.0).max(0) as u32 + 18;
    ui::badge(fb, x + w - 18 - bw, y + 30, b, bg, fg);
}
