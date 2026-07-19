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
use crate::wallet::theme::{ACCENT, AMBER, CYAN, DIM, FG, GREEN, GREEN_INK, INK, LINE2, MUTED, PANEL_2};

pub fn paint_send(state: &State, fb: &mut PaintBuffer) {
    let cx = 226u32;
    let lw = 640u32;
    let ix = cx + 20;
    let iw = lw - 40;
    ui::card(fb, cx, 146, lw, 590);

    let _ = fb.text_ttf(ix as i32, 162, "RECIPIENT", DIM, 10.5);
    field(fb, ix, 182, iw, "vitalik.eth", state.send_focus == SEND_FIELD_TO);
    ui::badge(fb, ix, 230, "ENS OK  \u{00b7}  0x9A2c\u{2026}B841".as_bytes(), GREEN, GREEN_INK);

    let _ = fb.text_ttf(ix as i32, 262, "RECENT", DIM, 10.5);
    let a = ui::badge(fb, ix, 282, "0x9A2c\u{2026}B841".as_bytes(), PANEL_2, MUTED);
    let b = ui::badge(fb, ix + a + 10, 282, b"vitalik.eth", PANEL_2, MUTED);
    ui::badge(fb, ix + a + b + 20, 282, "0x0a4f\u{2026}9d12".as_bytes(), PANEL_2, MUTED);

    let _ = fb.text_ttf(ix as i32, 322, "AMOUNT", DIM, 10.5);
    seg(fb, ix + iw - 96, 318, state.usd_mode);
    field(fb, ix, 342, iw - 92, "1.5", state.send_focus == SEND_FIELD_AMOUNT);
    ui::outline(fb, ix + iw - 80, 342, 80, b"MAX");
    let _ = fb.text_ttf(ix as i32, 390, "= $4680.00", MUTED, 13.0);

    let _ = fb.text_ttf(ix as i32, 420, "FEE / SPEED", DIM, 10.5);
    let fw = (iw - 24) / 3;
    fee(fb, ix, 440, fw, "Slow", "$0.9 - ~2m", state.fee_tier == 0);
    fee(fb, ix + fw + 12, 440, fw, "Avg", "$1.4 - ~30s", state.fee_tier == 1);
    fee(fb, ix + 2 * (fw + 12), 440, fw, "Fast", "$2.2 - ~12s", state.fee_tier == 2);

    ui::bordered(fb, ix, 516, iw, 42, PANEL_2, LINE2);
    let _ = fb.text_ttf((ix + 14) as i32, 528, "You'll have left", MUTED, 14.0);
    let lv = "0.9091 ETH";
    let lw2 = fb.measure_ttf(lv, 16.0).max(0) as u32;
    let _ = fb.text_ttf((ix + iw - 14 - lw2) as i32, 527, lv, CYAN, 16.0);

    ui::bordered(fb, ix, 574, iw, 40, 0xFF17_130A, 0xFF5A_4A1E);
    fb.fill_rect(ix + 14, 588, 10, 10, AMBER);
    let _ = fb.text_ttf((ix + 34) as i32, 586, "Broadcast waits for NONOS HTTPS RPC. Verify recipient.", AMBER, 13.0);

    ui::primary(fb, ix, 636, 150, b"Sign locally");
    ui::outline(fb, ix + 162, 636, 150, b"Add to batch");
    super::paint_send_side::paint_send_side(state, fb);
}

fn field(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, val: &str, active: bool) {
    let e = if active { ACCENT } else { LINE2 };
    ui::bordered(fb, x, y, w, 40, PANEL_2, e);
    let _ = fb.text_ttf((x + 12) as i32, (y + 11) as i32, val, FG, 15.0);
}

fn seg(fb: &mut PaintBuffer, x: u32, y: u32, usd: bool) {
    if usd {
        ui::edge(fb, x, y, 44, 24, LINE2);
        fb.fill_rect(x + 44, y, 44, 24, ACCENT);
    } else {
        fb.fill_rect(x, y, 44, 24, ACCENT);
        ui::edge(fb, x + 44, y, 44, 24, LINE2);
    }
    let eth = if usd { MUTED } else { INK };
    let usdc = if usd { INK } else { MUTED };
    let _ = fb.text_ttf((x + 11) as i32, (y + 5) as i32, "ETH", eth, 12.0);
    let _ = fb.text_ttf((x + 55) as i32, (y + 5) as i32, "USD", usdc, 12.0);
}

fn fee(fb: &mut PaintBuffer, x: u32, y: u32, w: u32, name: &str, sub: &str, sel: bool) {
    let e = if sel { ACCENT } else { LINE2 };
    ui::bordered(fb, x, y, w, 58, PANEL_2, e);
    let nc = if sel { ACCENT } else { FG };
    let nw = fb.measure_ttf(name, 14.0).max(0) as u32;
    let _ = fb.text_ttf((x + w / 2 - nw / 2) as i32, (y + 12) as i32, name, nc, 14.0);
    let sw = fb.measure_ttf(sub, 12.0).max(0) as u32;
    let _ = fb.text_ttf((x + w / 2 - sw / 2) as i32, (y + 34) as i32, sub, DIM, 12.0);
}
