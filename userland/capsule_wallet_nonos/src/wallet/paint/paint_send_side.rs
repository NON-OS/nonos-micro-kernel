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
use crate::wallet::theme::{CYAN, DIM, FG, GREEN, GREEN_INK, MUTED};

pub fn paint_send_side(_state: &State, fb: &mut PaintBuffer) {
    let x = 890u32;
    let w = fb.width.saturating_sub(x + 26);
    ui::card(fb, x, 146, w, 180);
    let _ = fb.text_ttf((x + 20) as i32, 164, "ROUTE", DIM(), 10.5);
    let _ = fb.text_ttf((x + 20) as i32, 186, "PublicNode Ethereum RPC", FG(), 16.0);
    ui::badge(fb, x + 20, 218, b"TLS SECURED", GREEN(), GREEN_INK());
    kv(fb, x, w, 252, "Gas (est.)", "0.00042 ETH");
    kv(fb, x, w, 278, "Relayer fee", "0.00010 ETH");
    kv(fb, x, w, 304, "Confirmation", "~12s");

    ui::card(fb, x, 342, w, 156);
    let _ = fb.text_ttf((x + 20) as i32, 360, "BATCH QUEUE  \u{00b7}  2", DIM(), 10.5);
    qrow(fb, x, w, 386, "0x9A2c\u{2026}B841", "1.5 ETH");
    qrow(fb, x, w, 428, "devfund.eth", "0.4 ETH");
    ui::outline(fb, x + 20, 470, w - 40, b"Sign batch");
}

fn kv(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, k: &str, v: &str) {
    let _ = fb.text_ttf((x + 20) as i32, y as i32, k, MUTED(), 13.0);
    let vw = fb.measure_ttf(v, 13.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 20 - vw) as i32, y as i32, v, FG(), 13.0);
}

fn qrow(fb: &mut PaintBuffer, x: u32, w: u32, y: u32, addr: &str, amt: &str) {
    super::ui::bordered(fb, x + 20, y, w - 40, 34, 0xFF0A_0F17, 0xFF26_313F);
    let _ = fb.text_ttf_mono((x + 32) as i32, (y + 9) as i32, addr, FG(), 13.0);
    let aw = fb.measure_ttf(amt, 13.0).max(0) as u32;
    let _ = fb.text_ttf((x + w - 32 - aw) as i32, (y + 9) as i32, amt, CYAN(), 13.0);
}
